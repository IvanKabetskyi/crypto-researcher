mod schema;

use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection,
};

use crate::domain::prediction::entities::Prediction;

use crate::infrastructure::services::database::get_db;

use crate::application::error::DataError;
use crate::application::request_dto::filter_params_dto::FilterParams;
use crate::application::request_dto::history_params_dto::HistoryParams;

pub use schema::PredictionSchema;

pub struct PredictionRepository {
    collection: Collection<PredictionSchema>,
}

impl PredictionRepository {
    pub async fn new() -> Self {
        let database = get_db().await.lock().await;

        Self {
            collection: database.db.collection::<PredictionSchema>("predictions"),
        }
    }

    pub async fn save_prediction(&self, prediction: &Prediction) -> Result<Prediction, DataError> {
        let document = PredictionSchema::from_prediction(prediction);

        let inserted_id = self
            .collection
            .insert_one(document, None)
            .await
            .map_err(|_| DataError::new("failed to save prediction"))?
            .inserted_id;

        let schema = self
            .collection
            .find_one(Some(doc! {"_id": inserted_id.as_object_id()}), None)
            .await
            .map_err(|_| DataError::new("failed to retrieve saved prediction"))?
            .ok_or_else(|| DataError::new("saved prediction not found"))?;

        Ok(schema.to_prediction())
    }

    pub async fn get_predictions(
        &self,
        filter: FilterParams,
    ) -> Result<Vec<Prediction>, DataError> {
        let mut filter_doc = doc! {};

        if let Some(ref symbol) = filter.symbol {
            filter_doc.insert("symbol", symbol);
        }

        if let Some(ref direction) = filter.direction {
            filter_doc.insert("direction", direction);
        }

        if let Some(min_confidence) = filter.min_confidence {
            filter_doc.insert("confidence", doc! {"$gte": min_confidence});
        }

        let limit = filter.limit.unwrap_or(50);

        let find_options = FindOptions::builder()
            .sort(doc! {"created_at": -1})
            .limit(limit)
            .build();

        let mut cursor = self
            .collection
            .find(Some(filter_doc), Some(find_options))
            .await
            .map_err(|_| DataError::new("failed to query predictions"))?;

        let mut predictions: Vec<Prediction> = Vec::new();

        while let Some(Ok(s)) = cursor.next().await {
            predictions.push(s.to_prediction());
        }

        Ok(predictions)
    }

    pub async fn update_outcome(
        &self,
        id: ObjectId,
        outcome: &str,
        actual_price: f64,
    ) -> Result<(), DataError> {
        let filter = doc! {"_id": &id};
        let update = doc! {"$set": {
            "outcome": outcome,
            "actual_price_after": actual_price,
        }};

        self.collection
            .update_one(filter, update, None)
            .await
            .map_err(|_| DataError::new("failed to update prediction outcome"))?;

        Ok(())
    }

    pub async fn get_history(
        &self,
        params: HistoryParams,
    ) -> Result<(Vec<Prediction>, i64, i64, i64), DataError> {
        let mut filter_doc = doc! {};

        if let Some(ref symbol) = params.symbol {
            filter_doc.insert("symbol", symbol);
        }

        if let Some(ref direction) = params.direction {
            filter_doc.insert("direction", direction);
        }

        if let Some(ref outcome) = params.outcome {
            if outcome == "pending" {
                filter_doc.insert("outcome", doc! {"$in": [null, "pending"]});
            } else {
                filter_doc.insert("outcome", outcome);
            }
        }

        if params.date_from.is_some() || params.date_to.is_some() {
            let mut date_filter = doc! {};
            if let Some(ref date_from) = params.date_from {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_from) {
                    let bson_dt = bson::DateTime::from_millis(dt.timestamp_millis());
                    date_filter.insert("$gte", bson_dt);
                }
            }
            if let Some(ref date_to) = params.date_to {
                if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(date_to) {
                    let bson_dt = bson::DateTime::from_millis(dt.timestamp_millis());
                    date_filter.insert("$lte", bson_dt);
                }
            }
            if !date_filter.is_empty() {
                filter_doc.insert("created_at", date_filter);
            }
        }

        let page = params.page.unwrap_or(1).max(1);
        let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
        let skip = (page - 1) * per_page;

        let total = self
            .collection
            .count_documents(Some(filter_doc.clone()), None)
            .await
            .map_err(|_| DataError::new("failed to count history predictions"))? as i64;

        let find_options = FindOptions::builder()
            .sort(doc! {"created_at": -1})
            .skip(Some(skip as u64))
            .limit(Some(per_page))
            .build();

        let mut cursor = self
            .collection
            .find(Some(filter_doc), Some(find_options))
            .await
            .map_err(|_| DataError::new("failed to query history predictions"))?;

        let mut predictions: Vec<Prediction> = Vec::new();

        while let Some(Ok(s)) = cursor.next().await {
            predictions.push(s.to_prediction());
        }

        Ok((predictions, total, page, per_page))
    }
}

mod schema;

use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Collection,
};

use std::collections::HashMap;

use crate::domain::prediction::entities::Prediction;

use crate::infrastructure::services::database::get_db;

use crate::application::error::DataError;
use crate::application::request_dto::filter_params_dto::FilterParams;
use crate::application::request_dto::history_params_dto::HistoryParams;

pub use schema::PredictionSchema;

pub struct AccuracyStats {
    pub total: i64,
    pub correct: i64,
    pub incorrect: i64,
    pub pending: i64,
    pub by_symbol: HashMap<String, SymbolStats>,
}

pub struct SymbolStats {
    pub total: i64,
    pub correct: i64,
    pub incorrect: i64,
    pub pending: i64,
}

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
        let document = PredictionSchema {
            id: prediction.get_id(),
            symbol: prediction.get_symbol(),
            direction: prediction.get_direction(),
            confidence: prediction.get_confidence(),
            reasoning: prediction.get_reasoning(),
            entry_price: prediction.get_entry_price(),
            target_price: prediction.get_target_price(),
            stop_loss: prediction.get_stop_loss(),
            created_at: prediction.get_created_at(),
            outcome: prediction.get_outcome(),
            actual_price_after: prediction.get_actual_price_after(),
            timeframe: prediction.get_timeframe(),
        };

        let result = self.collection.insert_one(document, None).await;

        if result.is_err() {
            return Err(DataError::new("failed to save prediction"));
        }

        let inserted_id = result.unwrap().inserted_id;

        let saved = self
            .collection
            .find_one(Some(doc! {"_id": inserted_id.as_object_id()}), None)
            .await;

        if saved.is_err() {
            return Err(DataError::new("failed to retrieve saved prediction"));
        }

        let schema = saved.unwrap();

        if schema.is_none() {
            return Err(DataError::new("saved prediction not found"));
        }

        let s = schema.unwrap();

        Ok(Prediction::new(
            &s.symbol,
            &s.direction,
            s.confidence,
            &s.reasoning,
            s.entry_price,
            s.target_price,
            s.stop_loss,
            Some(s.id),
            Some(s.created_at),
            s.outcome,
            s.actual_price_after,
            s.timeframe,
        ))
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

        let cursor_result = self
            .collection
            .find(Some(filter_doc), Some(find_options))
            .await;

        if cursor_result.is_err() {
            return Err(DataError::new("failed to query predictions"));
        }

        let mut cursor = cursor_result.unwrap();

        let mut predictions: Vec<Prediction> = Vec::new();

        while let Some(result) = cursor.next().await {
            if let Ok(s) = result {
                predictions.push(Prediction::new(
                    &s.symbol,
                    &s.direction,
                    s.confidence,
                    &s.reasoning,
                    s.entry_price,
                    s.target_price,
                    s.stop_loss,
                    Some(s.id),
                    Some(s.created_at),
                    s.outcome,
                    s.actual_price_after,
                    s.timeframe,
                ));
            }
        }

        Ok(predictions)
    }

    pub async fn update_outcome(
        &self,
        id: ObjectId,
        outcome: String,
        actual_price: f64,
    ) -> Result<(), DataError> {
        let filter = doc! {"_id": &id};
        let update = doc! {"$set": {
            "outcome": &outcome,
            "actual_price_after": actual_price,
        }};

        let response = self.collection.update_one(filter, update, None).await;

        if response.is_err() {
            return Err(DataError::new("failed to update prediction outcome"));
        }

        Ok(())
    }

    pub async fn get_accuracy_stats(&self) -> Result<AccuracyStats, DataError> {
        let all_cursor = self.collection.find(None, None).await;

        if all_cursor.is_err() {
            return Err(DataError::new("failed to query predictions for accuracy"));
        }

        let mut cursor = all_cursor.unwrap();

        let mut total: i64 = 0;
        let mut correct: i64 = 0;
        let mut incorrect: i64 = 0;
        let mut pending: i64 = 0;
        let mut by_symbol: HashMap<String, SymbolStats> = HashMap::new();

        while let Some(result) = cursor.next().await {
            if let Ok(s) = result {
                total += 1;

                let outcome = s.outcome.as_deref().unwrap_or("pending");

                match outcome {
                    "correct" => correct += 1,
                    "incorrect" => incorrect += 1,
                    _ => pending += 1,
                }

                let entry = by_symbol.entry(s.symbol.clone()).or_insert(SymbolStats {
                    total: 0,
                    correct: 0,
                    incorrect: 0,
                    pending: 0,
                });

                entry.total += 1;

                match outcome {
                    "correct" => entry.correct += 1,
                    "incorrect" => entry.incorrect += 1,
                    _ => entry.pending += 1,
                }
            }
        }

        Ok(AccuracyStats {
            total,
            correct,
            incorrect,
            pending,
            by_symbol,
        })
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
            filter_doc.insert("outcome", outcome);
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

        let count_result = self.collection.count_documents(Some(filter_doc.clone()), None).await;
        if count_result.is_err() {
            return Err(DataError::new("failed to count history predictions"));
        }
        let total = count_result.unwrap() as i64;

        let find_options = FindOptions::builder()
            .sort(doc! {"created_at": -1})
            .skip(Some(skip as u64))
            .limit(Some(per_page))
            .build();

        let cursor_result = self
            .collection
            .find(Some(filter_doc), Some(find_options))
            .await;

        if cursor_result.is_err() {
            return Err(DataError::new("failed to query history predictions"));
        }

        let mut cursor = cursor_result.unwrap();
        let mut predictions: Vec<Prediction> = Vec::new();

        while let Some(result) = cursor.next().await {
            if let Ok(s) = result {
                predictions.push(Prediction::new(
                    &s.symbol,
                    &s.direction,
                    s.confidence,
                    &s.reasoning,
                    s.entry_price,
                    s.target_price,
                    s.stop_loss,
                    Some(s.id),
                    Some(s.created_at),
                    s.outcome,
                    s.actual_price_after,
                    s.timeframe,
                ));
            }
        }

        Ok((predictions, total, page, per_page))
    }
}

use crate::domain::market::entities::NewsArticle;

pub struct CryptoRssService {
    client: reqwest::Client,
}

impl CryptoRssService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::limited(5))
                .timeout(std::time::Duration::from_secs(15))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub async fn fetch_news(
        &self,
        currencies: &[String],
    ) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_articles: Vec<NewsArticle> = Vec::new();

        // Bitcoin Magazine RSS feed
        match self.fetch_bitcoin_magazine_rss().await {
            Ok(articles) => {
                tracing::info!("Fetched {} articles from Bitcoin Magazine RSS", articles.len());
                all_articles.extend(articles);
            }
            Err(e) => {
                tracing::warn!("Failed to fetch Bitcoin Magazine RSS: {}", e);
            }
        }

        // CoinTelegraph RSS feed
        match self.fetch_cointelegraph_rss().await {
            Ok(articles) => {
                tracing::info!("Fetched {} articles from CoinTelegraph RSS", articles.len());
                all_articles.extend(articles);
            }
            Err(e) => {
                tracing::warn!("Failed to fetch CoinTelegraph RSS: {}", e);
            }
        }

        // Filter articles relevant to requested currencies
        let currencies_lower: Vec<String> = currencies.iter().map(|c| c.to_lowercase()).collect();
        let filtered: Vec<NewsArticle> = all_articles
            .into_iter()
            .filter(|article| {
                let title_lower = article.get_title().to_lowercase();
                // Keep article if it mentions any of the requested currencies
                // or common crypto terms (for general market sentiment)
                currencies_lower.iter().any(|c| title_lower.contains(c))
                    || title_lower.contains("bitcoin")
                    || title_lower.contains("btc")
                    || title_lower.contains("ethereum")
                    || title_lower.contains("eth")
                    || title_lower.contains("crypto")
                    || title_lower.contains("market")
            })
            .take(5)
            .collect();

        tracing::info!(
            "Filtered to {} relevant articles for currencies: {:?}",
            filtered.len(),
            currencies
        );

        Ok(filtered)
    }

    async fn fetch_bitcoin_magazine_rss(
        &self,
    ) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://bitcoinmagazine.com/.rss/full/";
        self.parse_rss_feed(url, "Bitcoin Magazine").await
    }

    async fn fetch_cointelegraph_rss(
        &self,
    ) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://cointelegraph.com/rss";
        self.parse_rss_feed(url, "CoinTelegraph").await
    }

    async fn parse_rss_feed(
        &self,
        url: &str,
        source_name: &str,
    ) -> Result<Vec<NewsArticle>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", "CryptoResearcher/1.0")
            .send()
            .await?;

        let body = response.text().await?;
        let mut articles = Vec::new();

        // Simple XML parsing for RSS <item> elements
        for item_block in body.split("<item>").skip(1) {
            let end = item_block.find("</item>").unwrap_or(item_block.len());
            let item = &item_block[..end];

            let title = extract_xml_tag(item, "title").unwrap_or_default();
            let pub_date = extract_xml_tag(item, "pubDate").unwrap_or_default();

            if !title.is_empty() {
                let sentiment = guess_sentiment(&title);

                articles.push(NewsArticle::new(
                    &title,
                    source_name,
                    sentiment,
                    &pub_date,
                ));
            }

            if articles.len() >= 15 {
                break;
            }
        }

        Ok(articles)
    }
}

fn extract_xml_tag(xml: &str, tag: &str) -> Option<String> {
    let open_tag = format!("<{}", tag);
    let close_tag = format!("</{}>", tag);

    let start_pos = xml.find(&open_tag)?;
    let after_open = &xml[start_pos..];

    // Find the end of the opening tag (handle attributes)
    let content_start = after_open.find('>')? + 1;
    let content = &after_open[content_start..];

    let end_pos = content.find(&close_tag)?;
    let raw = &content[..end_pos];

    // Handle CDATA sections
    let cleaned = raw
        .trim()
        .trim_start_matches("<![CDATA[")
        .trim_end_matches("]]>")
        .trim()
        .to_string();

    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}

fn guess_sentiment(title: &str) -> Option<String> {
    let lower = title.to_lowercase();

    let positive_words = [
        "surge", "rally", "bullish", "gains", "soars", "jumps", "rises",
        "high", "record", "breakout", "adoption", "approval", "boost",
        "growth", "up", "positive", "profit",
    ];
    let negative_words = [
        "crash", "bearish", "drops", "falls", "plunge", "decline", "dump",
        "low", "fear", "hack", "scam", "fraud", "ban", "regulation",
        "loss", "down", "negative", "sell-off", "selloff",
    ];

    let pos_count = positive_words.iter().filter(|w| lower.contains(*w)).count();
    let neg_count = negative_words.iter().filter(|w| lower.contains(*w)).count();

    if pos_count > neg_count {
        Some("positive".into())
    } else if neg_count > pos_count {
        Some("negative".into())
    } else {
        Some("neutral".into())
    }
}

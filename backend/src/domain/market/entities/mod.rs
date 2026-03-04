use std::collections::HashMap;

pub struct Ticker {
    symbol: String,
    last_price: f64,
    price_24h_pcnt: f64,
    volume_24h: f64,
    high_price_24h: f64,
    low_price_24h: f64,
}

impl Ticker {
    pub fn new(
        symbol: &str,
        last_price: f64,
        price_24h_pcnt: f64,
        volume_24h: f64,
        high_price_24h: f64,
        low_price_24h: f64,
    ) -> Self {
        Self {
            symbol: String::from(symbol),
            last_price,
            price_24h_pcnt,
            volume_24h,
            high_price_24h,
            low_price_24h,
        }
    }

    pub fn get_symbol(&self) -> String {
        self.symbol.clone()
    }

    pub fn get_last_price(&self) -> f64 {
        self.last_price
    }

    pub fn get_price_24h_pcnt(&self) -> f64 {
        self.price_24h_pcnt
    }

    pub fn get_volume_24h(&self) -> f64 {
        self.volume_24h
    }

    pub fn get_high_price_24h(&self) -> f64 {
        self.high_price_24h
    }

    pub fn get_low_price_24h(&self) -> f64 {
        self.low_price_24h
    }
}

pub struct Kline {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    timestamp: i64,
}

impl Kline {
    pub fn new(open: f64, high: f64, low: f64, close: f64, volume: f64, timestamp: i64) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            timestamp,
        }
    }

    pub fn get_open(&self) -> f64 {
        self.open
    }

    pub fn get_high(&self) -> f64 {
        self.high
    }

    pub fn get_low(&self) -> f64 {
        self.low
    }

    pub fn get_close(&self) -> f64 {
        self.close
    }

    pub fn get_volume(&self) -> f64 {
        self.volume
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }
}

pub struct NewsArticle {
    title: String,
    source: String,
    url: String,
    sentiment: Option<String>,
    published_at: String,
}

impl NewsArticle {
    pub fn new(
        title: &str,
        source: &str,
        url: &str,
        sentiment: Option<String>,
        published_at: &str,
    ) -> Self {
        Self {
            title: String::from(title),
            source: String::from(source),
            url: String::from(url),
            sentiment,
            published_at: String::from(published_at),
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_source(&self) -> String {
        self.source.clone()
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_sentiment(&self) -> Option<String> {
        self.sentiment.clone()
    }

    pub fn get_published_at(&self) -> String {
        self.published_at.clone()
    }
}

pub struct MarketSnapshot {
    tickers: Vec<Ticker>,
    klines: HashMap<String, Vec<Kline>>,
    news: Vec<NewsArticle>,
}

impl MarketSnapshot {
    pub fn new(
        tickers: Vec<Ticker>,
        klines: HashMap<String, Vec<Kline>>,
        news: Vec<NewsArticle>,
    ) -> Self {
        Self {
            tickers,
            klines,
            news,
        }
    }

    pub fn get_tickers(&self) -> &Vec<Ticker> {
        &self.tickers
    }

    pub fn get_klines(&self) -> &HashMap<String, Vec<Kline>> {
        &self.klines
    }

    pub fn get_news(&self) -> &Vec<NewsArticle> {
        &self.news
    }

    pub fn tickers_to_json(&self) -> String {
        let entries: Vec<String> = self
            .tickers
            .iter()
            .map(|t| {
                format!(
                    "{{\"symbol\":\"{}\",\"price\":{},\"change_24h\":{},\"volume\":{},\"high\":{},\"low\":{}}}",
                    t.get_symbol(),
                    t.get_last_price(),
                    t.get_price_24h_pcnt(),
                    t.get_volume_24h(),
                    t.get_high_price_24h(),
                    t.get_low_price_24h()
                )
            })
            .collect();
        format!("[{}]", entries.join(","))
    }

    pub fn klines_to_json(&self) -> String {
        let entries: Vec<String> = self
            .klines
            .iter()
            .map(|(symbol, klines)| {
                let kline_entries: Vec<String> = klines
                    .iter()
                    .map(|k| {
                        format!(
                            "{{\"o\":{},\"h\":{},\"l\":{},\"c\":{},\"v\":{},\"t\":{}}}",
                            k.get_open(),
                            k.get_high(),
                            k.get_low(),
                            k.get_close(),
                            k.get_volume(),
                            k.get_timestamp()
                        )
                    })
                    .collect();
                format!("\"{}\":[{}]", symbol, kline_entries.join(","))
            })
            .collect();
        format!("{{{}}}", entries.join(","))
    }

    pub fn news_to_json(&self) -> String {
        let entries: Vec<String> = self
            .news
            .iter()
            .map(|n| {
                let sentiment = match n.get_sentiment() {
                    Some(s) => format!("\"{}\"", s),
                    None => "null".to_string(),
                };
                format!(
                    "{{\"title\":\"{}\",\"source\":\"{}\",\"sentiment\":{},\"published\":\"{}\"}}",
                    n.get_title().replace('"', "\\\""),
                    n.get_source().replace('"', "\\\""),
                    sentiment,
                    n.get_published_at()
                )
            })
            .collect();
        format!("[{}]", entries.join(","))
    }
}

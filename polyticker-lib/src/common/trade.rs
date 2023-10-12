use chrono::{DateTime, Utc};

pub struct TradeData {
    pub symbol: String,
    pub currency: String,
    pub price: f64,
    pub timestamp: DateTime<Utc>,
    pub exchange_id: i64,
}

pub enum Currency {
    USD,
}
pub trait Trade {
    fn get_trade(&self) -> anyhow::Result<TradeData>;
}

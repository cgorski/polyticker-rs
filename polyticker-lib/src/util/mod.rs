use chrono::{DateTime, Utc};
use serde::Deserialize;

pub struct TimeUtil;

impl TimeUtil {
    pub fn timestamp_milliseconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let ts_milliseconds: i64 = Deserialize::deserialize(deserializer)?;
        let ts_seconds = ts_milliseconds / 1000;
        let naive_datetime = chrono::NaiveDateTime::from_timestamp_opt(ts_seconds, 0).ok_or(serde::de::Error::custom("invalid timestamp"))?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive_datetime, Utc))
    }
}

pub struct Stocks;
impl Stocks {
    pub fn default_is_otc_ticker() -> bool {
        false
    }
  
}
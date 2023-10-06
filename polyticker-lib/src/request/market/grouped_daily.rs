use reqwest;
use serde::Deserialize;

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::request::BASE_URL;
use crate::util::{TimeUtil, Stocks};

/// Represents an interface for fetching grouped daily data for the entire stocks/equities market.
pub struct GroupedDaily {
    /// The API key used for authenticating requests.
    api_key: String,
}

impl GroupedDaily {
    /// Creates a new `GroupedDaily` instance with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string representing the Polygon API key.
    pub fn new(api_key: String) -> GroupedDaily {
        GroupedDaily { api_key }
    }

    /// Fetches grouped daily data for the entire stocks/equities market for a given date.
    ///
    /// # Arguments
    ///
    /// * `date` - The beginning date for the aggregate window, formatted as YYYY-MM-DD.
    /// * `adjusted` - Whether or not the results are adjusted for splits.
    /// * `include_otc` - Whether to include OTC securities in the response.
    ///
    /// # Returns
    ///
    /// A `Result` containing `GroupedDailyApiResponse` if successful, or an error otherwise.
    pub async fn get_data(
        &self,
        date: &str,
        adjusted: bool,
        include_otc: bool,
    ) -> Result<GroupedDailyApiResponse, reqwest::Error> {
        let url = format!(
            "{base}/v2/aggs/grouped/locale/us/market/stocks/{date}",
            base = BASE_URL,
            date = date
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .query(&[("adjusted", adjusted.to_string()), ("include_otc", include_otc.to_string())])
            .send()
            .await?
            .json::<GroupedDailyApiResponse>()
            .await?;

        Ok(response)
    }
}

/// Represents the response from the Polygon grouped daily API.
#[derive(Deserialize, Debug)]
pub struct GroupedDailyApiResponse {
    /// Whether or not this response was adjusted for splits.
    adjusted: bool,
    /// The number of aggregates (minute or day) used to generate the response.
    #[serde(rename = "queryCount")]
    query_count: i64,
    /// A request id assigned by the server.
    request_id: String,
    /// The total number of results for this request.
    #[serde(rename = "resultsCount")]
    results_count: i64,
    /// The status of this request's response.
    status: String,
    /// An array of aggregate results for the given stock.
    results: Vec<GroupedDailyResult>,
}

/// Represents a single aggregate data point for the entire stocks/equities market over a specific time window.
#[derive(Deserialize, Debug)]
pub struct GroupedDailyResult {
    /// The exchange symbol that this item is traded under.
    #[serde(rename = "T")]
    ticker: String,
    /// The close price for the symbol in the given time period.
    #[serde(rename = "c")]
    close_price: f64,
    /// The highest price for the symbol in the given time period.
    #[serde(rename = "h")]
    highest_price: f64,
    /// The lowest price for the symbol in the given time period.
    #[serde(rename = "l")]
    lowest_price: f64,
    /// The number of transactions that occurred in the aggregate window.
    #[serde(rename = "n")]
    number_of_transactions: Option<u64>,
    /// The open price for the symbol in the given time period.
    #[serde(rename = "o")]
    open_price: f64,
    /// Whether this aggregate is for an OTC (Over The Counter) ticker.
    #[serde(rename = "otc", default = "Stocks::default_is_otc_ticker")]
    is_otc_ticker: bool,
    /// The Unix Msec timestamp marking the start of the aggregate window.
    #[serde(rename = "t", deserialize_with = "TimeUtil::timestamp_milliseconds")]
    timestamp: DateTime<Utc>,
    /// The trading volume of the symbol in the given time period.
    #[serde(rename = "v")]
    trading_volume: f64,
    /// The volume-weighted average price. Might be omitted in some cases.
    #[serde(rename = "vw")]
    volume_weighted_avg_price: Option<f64>,
}

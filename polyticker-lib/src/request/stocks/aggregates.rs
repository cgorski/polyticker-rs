use reqwest;
use serde::Deserialize;

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::request::BASE_URL;
use crate::util::{TimeUtil, Stocks};

/// Represents an interface for fetching stock aggregates.
pub struct Aggregates {
    /// The API key used for authenticating requests.
    api_key: String,
}

impl Aggregates {
    /// Creates a new `Aggregates` instance with the provided API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string representing the Polygon API key.
    pub fn new(api_key: String) -> Aggregates {
        Aggregates { api_key }
    }

    /// Fetches aggregate data for a stock over a given date range in custom time window sizes.
    ///
    /// # Arguments
    ///
    /// * `stocks_ticker` - The ticker symbol of the stock/equity.
    /// * `multiplier` - The size of the timespan multiplier.
    /// * `timespan` - The size of the time window (e.g., "day").
    /// * `from` - The start of the aggregate time window, formatted as YYYY-MM-DD.
    /// * `to` - The end of the aggregate time window, formatted as YYYY-MM-DD.
    /// * `adjusted` - Whether or not the results are adjusted for splits.
    /// * `sort` - The order of sorting, either "asc" or "desc".
    /// * `limit` - Limits the number of base aggregates queried.
    ///
    /// # Returns
    ///
    /// A `Result` containing `ApiResponse` if successful, or an error otherwise.
    pub async fn get_stock_data(
        &self,
        stocks_ticker: &str,
        multiplier: &str,
        timespan: &str,
        from: &str,
        to: &str,
        adjusted: bool,
        sort: &str,
        limit: i32,

    ) -> Result<ApiResponse, reqwest::Error> {
        let url = format!(
            "{base}/v2/aggs/ticker/{ticker}/range/{multiplier}/{timespan}/{from}/{to}",
            base = BASE_URL,
            ticker = stocks_ticker,
            multiplier = multiplier,
            timespan = timespan,
            from = from,
            to = to
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header("Authorization", format!("Bearer {}", &self.api_key))
            .query(&[("adjusted", adjusted.to_string()), ("sort", sort.to_string()), ("limit", limit.to_string())])
            .send()
            .await?
            .json::<ApiResponse>()
            .await?;

        Ok(response)
    }
}


/// Represents the response from the Polygon aggregates API.
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    /// The exchange symbol that this item is traded under.
    ticker: String,
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
    results: Vec<AggregateResult>,
    /// If present, this value can be used to fetch the next page of data.
    next_url: Option<String>,
}

/// Represents a single aggregate data point for a stock over a specific time window.
#[derive(Deserialize, Debug)]
pub struct AggregateResult {
    /// The close price for the stock in the given time period.
    #[serde(rename = "c")]
    close_price: f64,

    /// The highest price for the stock in the given time period.
    #[serde(rename = "h")]
    highest_price: f64,

    /// The lowest price for the stock in the given time period.
    #[serde(rename = "l")]
    lowest_price: f64,

    /// The number of transactions that occurred in the aggregate window.
    #[serde(rename = "n")]
    number_of_transactions: u64,

    /// The open price for the stock in the given time period.
    #[serde(rename = "o")]
    open_price: f64,

    /// Whether this aggregate is for an OTC (Over The Counter) ticker.
    #[serde(rename = "otc", default = "Stocks::default_is_otc_ticker")]
    is_otc_ticker: bool,

    /// The Unix Msec timestamp marking the start of the aggregate window.
    #[serde(rename = "t", deserialize_with = "TimeUtil::timestamp_milliseconds")]
    timestamp: DateTime<Utc>,

    /// The trading volume of the stock in the given time period.
    #[serde(rename = "v")]
    trading_volume: f64,

    /// The volume-weighted average price. Might be omitted in some cases.
    #[serde(rename = "vw")]
    volume_weighted_avg_price: Option<f64>,
}





use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use serde::{Deserialize, Serialize};

/// Represents the primary structure for managing stock data and operations.
pub struct Stocks {
    api_key: String,
}

impl Stocks {
    /// Creates a new `Stocks` instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice representing the API key for authentication.
    ///
    /// # Returns
    ///
    /// Returns an instance of `Stocks`.
    pub fn new(api_key: String) -> Stocks {
        Stocks { api_key }
    }

    /// Asynchronously establishes a connection to the server and listens for data.
    ///
    /// # Arguments
    ///
    /// * `api_key` - A string slice representing the API key for authentication.
    pub async fn open_data_channel(api_key: String) {
        let url = "wss://delayed.polygon.io/stocks";

        // Connect to the server
        let (mut ws_stream, _) = connect_async(url)
            .await
            .expect("Could not connect to the server");

        println!("Connected to {}", url);

        // Authentication and subscription logic omitted for brevity...

        loop {
            let msg = ws_stream.next().await;
            match msg {
                Some(Ok(message)) => {
                    println!("Received: {}", message);

                    let value: Value = serde_json::from_str(&message.to_string()).expect("Failed to parse the message");
                    if let Some(trade_event) = TradeEvent::from_value(&value) {
                        println!("{:?}", trade_event);
                    }
                }
                Some(Err(e)) => {
                    println!("Error: {}", e);
                    break;
                }
                None => break,
            }
        }
    }
}

/// Represents a trade event data structure received from the server.
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeEvent {
    #[serde(rename = "ev")]
    event_type: String,

    #[serde(rename = "sym")]
    symbol: String,

    #[serde(rename = "x")]
    exchange_id: u64,

    #[serde(rename = "i")]
    trade_id: String,

    #[serde(rename = "z")]
    tape: u64,

    #[serde(rename = "p")]
    price: f64,

    #[serde(rename = "s")]
    trade_size: u64,

    #[serde(rename = "c")]
    trade_conditions: Vec<i32>,

    #[serde(rename = "t")]
    timestamp: u64,

    #[serde(rename = "q")]
    sequence_number: i64,

    #[serde(rename = "trfi", default)]
    trade_reporting_facility_id: Option<i64>,

    #[serde(rename = "trft", default)]
    trf_timestamp: Option<u64>
}

impl TradeEvent {
    /// Deserializes a JSON value into a `TradeEvent` if the event type is "T".
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to a `serde_json::Value` containing the data.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a `TradeEvent` if the deserialization is successful.
    pub fn from_value(value: &Value) -> Option<Self> {
        if value["ev"] == "T" {
            serde_json::from_value(value.clone()).ok()
        } else {
            None
        }
    }
}

use crate::util::TimeUtil;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

pub struct Stocks {}

pub struct DataChannel {
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
}

impl Stocks {
    pub async fn open_data_channel(
        api_key: String,
        output_channel_size: usize,
    ) -> Receiver<TradeEvent> {
        let (mut tx, rx): (Sender<TradeEvent>, Receiver<TradeEvent>) =
            mpsc::channel(output_channel_size);

        tokio::task::spawn(async move {
            let url = "wss://delayed.polygon.io/stocks";

            // Connect to the server
            let (mut ws_stream, _) = connect_async(url)
                .await
                .expect("Could not connect to the server");

            println!("Connected to {}", url);

            let msg = ws_stream.next().await;

            if let Some(Ok(message)) = msg {
                let response: Value = serde_json::from_str(&message.to_string())
                    .expect("Failed to parse the message");
                if response[0]["status"] == "connected" {
                    println!("Connected Successfully!");

                    let key = format!(r#"{{"action":"auth","params":"{}"}}"#, api_key);
                    // Authenticate
                    ws_stream
                        .send(Message::Text(key))
                        .await
                        .expect("Failed to send auth message");

                    // Read the auth response
                    let auth_msg = ws_stream.next().await;

                    if let Some(Ok(auth_response)) = auth_msg {
                        let auth_value: Value = serde_json::from_str(&auth_response.to_string())
                            .expect("Failed to parse the auth message");
                        if auth_value[0]["status"] == "auth_success" {
                            println!("Authenticated!");

                            // Subscribe
                            let sub_msg = r#"{"action":"subscribe","params":"AM.LPL,AM.MSFT"}"#;
                            ws_stream
                                .send(Message::Text(sub_msg.to_string()))
                                .await
                                .expect("Failed to send subscribe message");
                        } else {
                            println!("Authentication failed!");
                            return;
                        }
                    }
                }
            }

            loop {
                let msg = ws_stream.next().await;
                match msg {
                    Some(Ok(message)) => {
                        println!("Received: {:?}", message);
                        match &message {
                            Message::Text(text) => {
                                let value: Value = serde_json::from_str(&message.to_string())
                                    .expect("Failed to parse the message");
                                if let Some(trade_event) = TradeEvent::from_value(&value) {
                                    if tx.send(trade_event).await.is_err() {
                                        eprintln!("Receiver dropped!");
                                        break;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        println!("Error: {}", e);
                        break;
                    }
                    None => break,
                }
            }
        });
        rx
    }
}

/// Represents a trade event data structure received from the server.
#[derive(Debug, Deserialize)]
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

    #[serde(rename = "t", deserialize_with = "TimeUtil::timestamp_milliseconds")]
    timestamp: DateTime<Utc>,

    #[serde(rename = "q")]
    sequence_number: i64,

    #[serde(rename = "trfi", default)]
    trade_reporting_facility_id: Option<i64>,

    #[serde(rename = "trft", default)]
    trf_timestamp: Option<u64>,
}

impl TradeEvent {
    pub fn from_value(value: &Value) -> Option<Self> {
        if value["ev"] == "T" {
            serde_json::from_value(value.clone()).ok()
        } else {
            None
        }
    }
}

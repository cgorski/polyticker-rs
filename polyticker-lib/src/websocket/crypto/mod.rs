use crate::util::TimeUtil;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::common::trade::{Trade, TradeData};
use tracing::info;

pub struct Crypto {
    api_key: String,
}

impl Crypto {
    pub async fn open_data_channel(
        api_key: String,
        pairs: String,
        output_channel_size: usize,
    ) -> Receiver<CryptoTradeEvent> {
        let (mut tx, rx): (Sender<CryptoTradeEvent>, Receiver<CryptoTradeEvent>) =
            mpsc::channel(output_channel_size);

        tokio::task::spawn(async move {
            let url = "wss://socket.polygon.io/crypto";

            // Connect to the server
            let (mut ws_stream, _) = connect_async(url)
                .await
                .expect("Could not connect to the server");

            info!("Connected to {}", url);

            // Authenticate
            let key = format!(r#"{{"action":"auth","params":"{}"}}"#, &api_key);
            ws_stream
                .send(Message::Text(key.clone()))
                .await
                .expect("Failed to send auth message");
            info!("Sent auth message: {}", key);
            // Subscribe
            let sub_msg = format!(r#"{{"action":"subscribe", "params":"{}"}}"#, pairs);
            ws_stream
                .send(Message::Text(sub_msg.clone()))
                .await
                .expect("Failed to send subscribe message");
            println!("Sent subscribe message: {}", sub_msg);

            loop {
                let msg = ws_stream.next().await;
                match msg {
                    Some(Ok(message)) => {
                        info!("Received: {:?}", message);
                        match &message {
                            Message::Text(text) => {
                                let value: Value = serde_json::from_str(&message.to_string())
                                    .expect("Failed to parse the message");
                                // value is a JSON array, convert it to a Vec<Value>
                                let values = match value.as_array() {
                                    Some(values) => values,
                                    None => {
                                        println!("Error: value is not an array");
                                        continue;
                                    }
                                };
                                for value in values {
                                    info!("Value: {}", value);
                                    match CryptoTradeEvent::from_value(&value) {
                                        Ok(crypto_trade_event) => {
                                            if tx.send(crypto_trade_event.clone()).await.is_err() {
                                                eprintln!("Receiver dropped!");
                                                break;
                                            }
                                        }
                                        Err(e) => {
                                            println!("Error: {}", e);
                                        }
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

/// Represents a crypto trade event data structure received from the server.
#[derive(Clone, Debug, Deserialize)]
pub struct CryptoTradeEvent {
    #[serde(rename = "ev")]
    event_type: String,

    #[serde(rename = "pair")]
    pair: String,

    #[serde(rename = "p")]
    price: f64,

    #[serde(rename = "t", deserialize_with = "TimeUtil::timestamp_milliseconds")]
    timestamp: DateTime<Utc>,

    #[serde(rename = "s")]
    size: f64,

    #[serde(rename = "c")]
    conditions: Vec<i32>,

    #[serde(rename = "i")]
    trade_id: String,

    #[serde(rename = "x")]
    exchange_id: i64,

    #[serde(rename = "r", deserialize_with = "TimeUtil::timestamp_milliseconds")]
    received_timestamp: DateTime<Utc>,
}

impl CryptoTradeEvent {
    pub fn from_value(value: &Value) -> anyhow::Result<Self> {
        if value["ev"] == "XT" {
            return serde_json::from_value(value.clone()).map_err(|e| anyhow::Error::msg(e));
        }
        Err(anyhow::Error::msg("Not a crypto trade event"))
    }
}

impl Trade for CryptoTradeEvent {
    fn get_trade(&self) -> anyhow::Result<TradeData> {
        // split self.pair by hyphen
        let pair_split = self.pair.split('-').collect::<Vec<&str>>();
        let symbol = pair_split
            .get(0)
            .ok_or(anyhow::Error::msg("No symbol"))?
            .to_string();
        let currency = pair_split
            .get(1)
            .ok_or(anyhow::Error::msg("No currency"))?
            .to_string();
        let price = self.price;
        let timestamp = self.timestamp;
        let exchange_id = self.exchange_id;
        Ok(TradeData {
            symbol,
            currency,
            price,
            timestamp,
            exchange_id,
        })
    }
}

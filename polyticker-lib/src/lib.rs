pub mod request;
pub mod util;

use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::connect_async;
use serde_json::Value;
use futures::{SinkExt, StreamExt};

pub async fn websocket(api_key: String) {
    let url = "wss://delayed.polygon.io/stocks";

    // Connect to the server
    let (mut ws_stream, _) = connect_async(url)
        .await
        .expect("Could not connect to the server");

    println!("Connected to {}", url);

    // Read the connection status
    let msg = ws_stream.next().await;

    if let Some(Ok(message)) = msg {
        let response: Value = serde_json::from_str(&message.to_string()).expect("Failed to parse the message");
        if response[0]["status"] == "connected" {
            println!("Connected Successfully!");

            let key = format!(r#"{{"action":"auth","params":"{}"}}"#, api_key);
            // Authenticate
            ws_stream.send(Message::Text(key)).await.expect("Failed to send auth message");

            // Read the auth response
            let auth_msg = ws_stream.next().await;

            if let Some(Ok(auth_response)) = auth_msg {
                let auth_value: Value = serde_json::from_str(&auth_response.to_string()).expect("Failed to parse the auth message");
                if auth_value[0]["status"] == "auth_success" {
                    println!("Authenticated!");

                    // Subscribe
                    let sub_msg = r#"{"action":"subscribe","params":"AM.LPL,AM.MSFT"}"#;
                    ws_stream.send(Message::Text(sub_msg.to_string())).await.expect("Failed to send subscribe message");
                } else {
                    println!("Authentication failed!");
                    return;
                }
            }
        }
    }

    // Keep the connection alive or add more logic to handle incoming messages...
    loop {
        let msg = ws_stream.next().await;
        match msg {
            Some(Ok(message)) => {
                println!("Received: {}", message);
            }
            Some(Err(e)) => {
                println!("Error: {}", e);
                break;
            }
            None => break,
        }
    }
}

use tokio_tungstenite::tungstenite::Message::Text;
use tokio_tungstenite::connect_async;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use tokio_tungstenite::WebSocketStream;

async fn send_request(stream: &mut WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let request = r#"
        {
            "jsonrpc": "2.0",
            "id": 420,
            "method": "transactionSubscribe",
            "params": [
                {
                    "vote": false,
                    "failed": false,
                    "accountInclude": ["4DoNfFBfF7UokCC2FQzriy7yHK6DY6NVdYpuekQ5pRgg"]
                },
                {
                    "commitment": "processed",
                    "encoding": "base64",
                    "transactionDetails": "full",
                    "showRewards": true,
                    "maxSupportedTransactionVersion": 0
                }
            ]
        }
    "#;

    stream.send(Text(request.into())).await
}

#[tokio::main]
async fn main() {
    let ws_url = "wss://atlas-mainnet.helius-rpc.com/?api-key=PAID_HELIUS_API_KEY_HERE";

    match connect_async(ws_url).await {
        Ok((mut stream, _)) => {
            println!("WebSocket is open");
            
            if let Err(e) = send_request(&mut stream).await {
                eprintln!("Failed to send request: {:?}", e);
            }

            while let Some(Ok(message)) = stream.next().await {
                match message {
                    Text(data) => {
                        println!("Received data: {}", data);
                    }
                    _ => {
                        println!("Received non-text message");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("WebSocket error: {:?}", e);
        }
    }

    println!("WebSocket is closed");
}

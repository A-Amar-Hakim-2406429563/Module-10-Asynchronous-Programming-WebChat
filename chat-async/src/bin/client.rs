use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use std::error::Error;
use std::env;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await
            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();
    let client_label = env::var("HOSTNAME").unwrap_or_else(|_| "Client".to_string());

    loop {
        tokio::select! {
            line_result = stdin.next_line() => {
                match line_result? {
                    Some(line) => {
                        ws_stream
                            .send(Message::text(line))
                            .await
                            .map_err(|err| -> Box<dyn Error + Send + Sync> { Box::new(err) })?;
                    }
                    None => break,
                }
            }
            maybe_msg = ws_stream.next() => {
                match maybe_msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        if let Some(text) = msg.as_text() {
                            println!("{client_label} - From server: {text}");
                        }
                    }
                    Some(Ok(msg)) if msg.is_close() => break,
                    Some(Ok(_)) => {}
                    Some(Err(err)) => return Err(Box::new(err) as Box<dyn Error + Send + Sync>),
                    None => break,
                }
            }
        }
    }

    Ok(())
}
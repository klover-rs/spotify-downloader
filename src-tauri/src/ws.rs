use tauri::Manager;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;
use futures_util::StreamExt;

use anyhow::Result;

const DOWNLOAD_STATE: &str = "download_state";

pub async fn connect_ws(app_handle: tauri::AppHandle) -> Result<()> {
    let url = Url::parse("ws://127.0.0.1:4040/send")?;
    let (mut ws_stream, _) = connect_async(url).await?;

    loop {
        tokio::select! {
            Some(msg) = ws_stream.next() => {
                match msg {
                    Ok(Message::Text(text)) => {
                        app_handle.emit_all(DOWNLOAD_STATE, text)?;
                    }
                    Ok(Message::Close(_)) => {
                        println!("server disconnected");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        }
    }

    Ok(())

}


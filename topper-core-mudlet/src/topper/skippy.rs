use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use futures::{SinkExt, StreamExt};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub const SKIPPY_MODULE_NAME: &str = "skippy";

const POLL_INTERVAL: Duration = Duration::from_millis(20);
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// Websocket client module. Connects (and reconnects) to a skippy server on a
/// background thread running its own Tokio runtime. `ModuleMsg("skippy", text)`
/// requests are sent to the server, and anything received is surfaced as the
/// "skippy" passive value.
pub struct SkippyModule {
    outgoing: Sender<String>,
    incoming: Receiver<String>,
}

impl SkippyModule {
    pub fn new(url: &str) -> Self {
        let (outgoing, outgoing_rx) = mpsc::channel();
        let (incoming_tx, incoming) = mpsc::channel();
        let url = url.to_string();
        thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to start skippy runtime");
            runtime.block_on(run_client(url, outgoing_rx, incoming_tx));
        });
        SkippyModule { outgoing, incoming }
    }

    fn drain_incoming(&self) -> Vec<String> {
        let mut messages = Vec::new();
        while let Ok(message) = self.incoming.try_recv() {
            messages.push(message);
        }
        messages
    }
}

impl<'s, TS, BS> TopperModule<'s, TS, BS> for SkippyModule {
    type Siblings = ();
    fn handle_message(
        &mut self,
        message: &TopperMessage<TS>,
        _siblings: Self::Siblings,
    ) -> Result<TopperResponse<BS>, String> {
        if let TopperMessage::Request(TopperRequest::ModuleMsg(module, text)) = message {
            if module == SKIPPY_MODULE_NAME {
                self.outgoing
                    .send(text.clone())
                    .map_err(|_| "Skippy client thread has stopped.".to_string())?;
            }
        }
        let received = self.drain_incoming();
        if received.is_empty() {
            Ok(TopperResponse::silent())
        } else {
            Ok(TopperResponse::passive(
                SKIPPY_MODULE_NAME.to_string(),
                received.join(";;"),
            ))
        }
    }
}

async fn run_client(url: String, outgoing: Receiver<String>, incoming: Sender<String>) {
    loop {
        match tokio_tungstenite::connect_async(url.as_str()).await {
            Ok((socket, _response)) => {
                if let Pump::Shutdown = pump(socket, &outgoing, &incoming).await {
                    return;
                }
            }
            Err(err) => {
                info!("Skippy connection to {} failed: {}", url, err);
            }
        }
        // Discard anything queued while disconnected rather than replaying it stale.
        while outgoing.try_recv().is_ok() {}
        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}

enum Pump {
    Disconnected,
    Shutdown,
}

async fn pump(
    mut socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    outgoing: &Receiver<String>,
    incoming: &Sender<String>,
) -> Pump {
    loop {
        loop {
            match outgoing.try_recv() {
                Ok(text) => {
                    if socket.send(Message::text(text)).await.is_err() {
                        return Pump::Disconnected;
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    let _ = socket.close(None).await;
                    return Pump::Shutdown;
                }
            }
        }
        match tokio::time::timeout(POLL_INTERVAL, socket.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => {
                if incoming.send(text.to_string()).is_err() {
                    return Pump::Shutdown;
                }
            }
            Ok(Some(Ok(Message::Close(_)))) | Ok(None) => return Pump::Disconnected,
            Ok(Some(Ok(_))) => {}
            Ok(Some(Err(_))) => return Pump::Disconnected,
            Err(_) => {} // Timed out; loop back to flush outgoing messages.
        }
    }
}

use crate::topper::{TopperMessage, TopperModule, TopperRequest, TopperResponse};
use futures::{Sink, SinkExt, StreamExt};
use log::{info, warn};
use skippy_protocol::{SimulacrumCommand, SimulacrumEvent, SimulacrumLocation, SkippyId};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::sync::mpsc as tokio_mpsc;
use tokio_tungstenite::tungstenite::Message;

pub const SKIPPY_MODULE_NAME: &str = "skippy";

/// ID and display name this module registers with the skippy server.
const APP_ID: &str = "topper";
const APP_NAME: &str = "Topper";
const RECONNECT_DELAY: Duration = Duration::from_secs(5);

/// Websocket client for the skippy server's `/client` endpoint. Runs on a
/// background thread with its own Tokio runtime, registers an app on every
/// (re)connection, and subscribes to its events.
///
/// `ModuleMsg("skippy", json)` requests must contain a JSON `SimulacrumCommand`
/// and are forwarded once the app is registered. Every server message is
/// surfaced as the "skippy" passive value, joined with `;;`.
pub struct SkippyModule {
    outgoing: tokio_mpsc::UnboundedSender<SimulacrumCommand>,
    incoming: Receiver<SimulacrumEvent>,
}

impl SkippyModule {
    /// `url` is the full websocket URL of the `/client` endpoint, e.g. `ws://localhost:8080/client`.
    pub fn new(url: &str) -> Self {
        let (outgoing, outgoing_rx) = tokio_mpsc::unbounded_channel();
        let (incoming_tx, incoming) = mpsc::channel();
        let url = url.to_string();
        thread::spawn(move || {
            let runtime = Runtime::new().expect("Failed to start skippy runtime");
            runtime.block_on(run_client(url, outgoing_rx, incoming_tx));
        });
        SkippyModule { outgoing, incoming }
    }

    fn drain_incoming(&self) -> Vec<SimulacrumEvent> {
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
        let mut response = TopperResponse::silent();
        if let TopperMessage::Request(TopperRequest::ModuleMsg(module, text)) = message {
            if module == SKIPPY_MODULE_NAME {
                match serde_json::from_str::<SimulacrumCommand>(text) {
                    Ok(command) => self
                        .outgoing
                        .send(command)
                        .map_err(|_| "Skippy client thread has stopped.".to_string())?,
                    Err(err) => {
                        response = TopperResponse::error(format!("Invalid skippy command: {err}"));
                    }
                }
            }
        }
        let received = self.drain_incoming();
        if !received.is_empty() {
            response = response.then(TopperResponse::passive(
                SKIPPY_MODULE_NAME.to_string(),
                serde_json::to_string(&received).unwrap_or_default(),
            ));
        }
        Ok(response)
    }
}

async fn run_client(
    url: String,
    mut outgoing: tokio_mpsc::UnboundedReceiver<SimulacrumCommand>,
    incoming: Sender<SimulacrumEvent>,
) {
    loop {
        match tokio_tungstenite::connect_async(url.as_str()).await {
            Ok((socket, _response)) => {
                info!("Skippy connected to {url}");
                if let Session::Shutdown = session(socket, &mut outgoing, &incoming).await {
                    return;
                }
                info!("Skippy disconnected from {url}");
            }
            Err(err) => warn!("Skippy connection to {url} failed: {err}"),
        }
        // The server forgets registrations and subscriptions on disconnect, so
        // commands queued for the old connection are stale.
        while outgoing.try_recv().is_ok() {}
        tokio::time::sleep(RECONNECT_DELAY).await;
    }
}

enum Session {
    Disconnected,
    Shutdown,
}

fn app_id() -> SkippyId {
    SkippyId::Simple(APP_ID.to_string())
}

/// Runs one connection: registers the app, then pumps commands out and server
/// messages in. Commands are held back until the server confirms registration,
/// since it rejects commands for apps that aren't registered on the connection.
async fn session(
    socket: tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    outgoing: &mut tokio_mpsc::UnboundedReceiver<SimulacrumCommand>,
    incoming: &Sender<SimulacrumEvent>,
) -> Session {
    let (mut write, mut read) = socket.split();

    let register = SimulacrumCommand::RegisterApp {
        id: app_id(),
        name: APP_NAME.to_string(),
        location_hint: SimulacrumLocation::EyeLevel,
    };
    if send_command(&mut write, &register).await.is_err() {
        return Session::Disconnected;
    }

    let mut registered = false;
    loop {
        tokio::select! {
            command = outgoing.recv(), if registered => {
                let Some(command) = command else {
                    return Session::Shutdown;
                };
                if send_command(&mut write, &command).await.is_err() {
                    return Session::Disconnected;
                }
            }
            frame = read.next() => {
                let text = match frame {
                    Some(Ok(Message::Text(text))) => text,
                    Some(Ok(Message::Close(_))) | None => return Session::Disconnected,
                    Some(Ok(_)) => continue,
                    Some(Err(err)) => {
                        warn!("Skippy websocket error: {err}");
                        return Session::Disconnected;
                    }
                };
                let text = text.as_str();

                if let Ok(SimulacrumEvent::RegistrationSuccessful { app_id: id }) =
                    serde_json::from_str::<SimulacrumEvent>(text)
                {
                    if id == app_id() {
                        info!("Skippy registered app {APP_ID}");
                        registered = true;
                        let subscribe = SimulacrumCommand::SubscribeAppEvents { app_id: app_id() };
                        if send_command(&mut write, &subscribe).await.is_err() {
                            return Session::Disconnected;
                        }
                    }
                }

                if let Ok(event) = serde_json::from_str::<SimulacrumEvent>(text) {
                    if incoming.send(event).is_err() {
                        return Session::Shutdown;
                    }
                }
            }
        }
    }
}

async fn send_command<W>(write: &mut W, command: &SimulacrumCommand) -> Result<(), ()>
where
    W: Sink<Message> + Unpin,
{
    let json = serde_json::to_string(command).map_err(|_| ())?;
    write.send(Message::text(json)).await.map_err(|_| ())
}

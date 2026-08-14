use domain::agent::SessionId;

use crate::message::listener_message::ListenerMessage;

#[derive(Debug, Clone)]
pub struct SessionActor {
    pub id: SessionId,
    pub outbound: tokio::sync::mpsc::Sender<ListenerMessage>,
}

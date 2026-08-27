use std::net::SocketAddr;

use application::domain::model::listener_model::{ListenerModel, ListenerProtocol};
use tokio::sync::oneshot;
use uuid::Uuid;

pub enum C2Message {
    Listener(ListenerMessage),
    Agent(AgentMessage),
}

pub enum ListenerMessage {
    ListListener {
        reply: oneshot::Sender<anyhow::Result<Vec<ListenerModel>>>,
    },
    AddListener {
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    StartListener {
        listener_id: Uuid,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    StopListener {
        listener_id: Uuid,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    RemoveListener {
        listener_id: Uuid,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    ListenerRequestReceived,

}

pub enum AgentMessage {
}

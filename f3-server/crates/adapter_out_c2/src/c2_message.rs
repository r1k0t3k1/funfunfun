use std::net::SocketAddr;

use application::{domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol}, outbound::agent::AgentId};
use tokio::sync::oneshot;

pub enum C2Message {
    Listener(ListenerMessage),
    Agent(AgentMessage),
}

pub enum ListenerMessage {
    List {
        reply: oneshot::Sender<anyhow::Result<Vec<ListenerModel>>>,
    },
    Add {
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    Start {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Stop {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Remove {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Query {
        listener_id: ListenerId,
        agent_id: AgentId,
        reply: oneshot::Sender<anyhow::Result<[u8;32]>>,
    },
    AgentCheckinReceived {
        listener_id: ListenerId,
        agent_id: AgentId,
        agent_pubkey: [u8; 32],
    },
    AgentCheckinCompleted {
        listener_id: ListenerId,
        agent_id: AgentId
    },
    CheckinAgent {
        agent_id: AgentId,
        agent_pubkey: [u8; 32],
    },
    CompleteCheckinAgent {
        listener_id: ListenerId,
        agent_id: AgentId
    },
    QuerySecret {
        listener_id: ListenerId,
        agent_id: AgentId,
        reply: oneshot::Sender<anyhow::Result<[u8;32]>>,
    },
}

pub enum AgentMessage {
    CheckinComplete,
    QuerySecret {
        reply: oneshot::Sender<anyhow::Result<[u8;32]>>,
    }
}

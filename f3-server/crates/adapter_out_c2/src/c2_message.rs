use std::net::SocketAddr;

use application::domain::model::{agent_model::AgentModel, id::{AgentId, ListenerId}, listener_model::{ListenerModel, ListenerProtocol}};
use tokio::sync::oneshot;

pub enum C2Message {
    QueryListener {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    ListListener {
        reply: oneshot::Sender<anyhow::Result<Vec<ListenerModel>>>,
    },
    AddListener {
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    RemoveListener {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    ListAgent {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<Vec<AgentModel>>>,
    },
    QueryAgent {
        agent_id: AgentId,
        reply: oneshot::Sender<anyhow::Result<AgentModel>>,
    },
    AddAgent {
        listener_id: ListenerId,
        received_pubkey: [u8; 32],
        reply: oneshot::Sender<anyhow::Result<AgentModel>>,
    },
    ToListener { listener_id: ListenerId, msg: ListenerMessage },
    ToAgent { agent_id: AgentId, msg: AgentMessage },
}

pub enum ListenerMessage {
    Start {
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Stop {
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    Query {
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    //AgentCheckinReceived {
    //    listener_id: ListenerId,
    //    agent_id: AgentId,
    //    agent_pubkey: [u8; 32],
    //},
    //AgentCheckinCompleted {
    //    listener_id: ListenerId,
    //    agent_id: AgentId
    //},
    //CheckinAgent {
    //    agent_id: AgentId,
    //    agent_pubkey: [u8; 32],
    //},
    //CompleteCheckinAgent {
    //    listener_id: ListenerId,
    //    agent_id: AgentId
    //},
}

pub enum AgentMessage {
    Query {
        reply: oneshot::Sender<anyhow::Result<AgentModel>>,
    },
    CheckinComplete,
}

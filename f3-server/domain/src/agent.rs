use uuid::Uuid;

use crate::{listener::ListenerId, model::packet_model::{CheckinCompleteResponse, CheckinResponse}};

pub type AgentId = Uuid;

pub struct Agent {
    pub agent_id: AgentId,
    pub listener_id: ListenerId,
}

type AgentPublicKey = [u8; 32];

pub enum AgentEvent {
    Checkin { 
        agent_public_key: AgentPublicKey,
        response_sender: tokio::sync::oneshot::Sender<CheckinResponse>,
    },
    CheckinComplete { 
        agent_info: String,
        response_sender: tokio::sync::oneshot::Sender<CheckinCompleteResponse>,
    }, // TODO
}

use crate::model::packet_model::{CheckinCompleteResponse, CheckinResponse};

pub struct Agent {
    pub agent_id: String,
    pub listener_id: String,
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

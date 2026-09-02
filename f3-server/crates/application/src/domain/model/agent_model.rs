use crate::domain::model::id::{AgentId, ListenerId};

#[derive(Debug, Clone)]
pub enum AgentStatus {
    CheckinProcessStarted,
    CheckinProcessCompleted,
}

#[derive(Clone)]
pub struct AgentModel {
    pub id: AgentId,
    pub listener_id: ListenerId,
    pub status: AgentStatus,
    pub session_pubkey: [u8;32],
    pub shared_secret: [u8;32],
}

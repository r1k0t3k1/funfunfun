use crate::error::DomainError;

pub type AgentEvent = String;
pub type ListenerId = String;
pub type AgentId = String;

#[async_trait::async_trait]
pub trait TransportGateway {
    async fn send(&self) -> Result<(), DomainError>;
    async fn recv(&self) -> Result<AgentEvent, DomainError>;
}

pub struct CheckinRequest {
    pub listener_id: ListenerId,
    pub agent_pubkey: [u8; 32],
}

pub struct CheckinResponse {
    pub listener_id: ListenerId,
    pub agent_id: AgentId,
    pub listner_pubkey: [u8; 32],
}

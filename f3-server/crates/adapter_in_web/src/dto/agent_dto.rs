use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct ListAgentRequest {
    pub listener_id: String,
}

#[derive(Deserialize, ToSchema, IntoParams)]
pub struct GetAgentRequest {
    pub agent_id: String,
}

#[derive(Serialize, ToSchema)]
pub struct AgentResponse {
    pub id: String,
    pub listener_id: String,
    pub status: String,
    pub session_pubkey: [u8;32],
    pub shared_secret: [u8;32],
}

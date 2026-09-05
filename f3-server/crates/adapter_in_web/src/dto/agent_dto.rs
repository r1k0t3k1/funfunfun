use application::domain::model::agent_model::AgentModel;
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
    pub process_id: u64,
    pub thread_id: u64,
    pub arch: String,
    pub is_admin: bool,
    pub process_name: String,
    pub os: String,
    pub domain_name: String,
    pub computer_name: String,
    pub user_name: String,
}

impl From<AgentModel> for AgentResponse {
    fn from(value: AgentModel) -> Self {
        Self {
            id: value.id.to_string(), 
            listener_id: value.listener_id.to_string(), 
            process_id: value.process_id,
            thread_id: value.thread_id,
            arch: value.arch, 
            is_admin: value.is_admin,
            process_name: value.process_name,
            os: value.os,
            domain_name: value.domain_name, 
            computer_name: value.computer_name,
            user_name: value.user_name ,
        }
    }
} 


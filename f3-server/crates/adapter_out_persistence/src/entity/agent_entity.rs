use application::domain::model::agent_model::AgentModel;
use sqlx::types::{Uuid, chrono};

#[derive(Clone)]
pub struct AgentEntity {
    id: Uuid,
    listener_id: Uuid,
    process_id: u64,
    thread_id: u64,
    arch: String,
    is_admin: bool,
    process_name: String,
    os: String,
    domain_name: String,
    computer_name: String,
    user_name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl Into<AgentModel> for AgentEntity {
    fn into(self) -> AgentModel {
        AgentModel {
            id: self.id.into(),
            listener_id: self.listener_id.into(),
            process_id: self.process_id,
            thread_id: self.thread_id,
            arch: self.arch,
            is_admin: self.is_admin,
            process_name: self.process_name,
            os: self.os,
            domain_name: self.domain_name,
            computer_name: self.computer_name,
            user_name: self.user_name,
            checkin_at: self.created_at.into(),
        }
    }
}

use std::time;

use crate::domain::model::id::{AgentId, ListenerId};

#[derive(Clone)]
pub struct AgentModel {
    pub id: AgentId,
    pub listener_id: ListenerId,
    pub process_id: u64,
    pub thread_id: u64,
    pub arch: String,
    pub is_admin: bool,
    pub process_name: String,
    pub os: String,
    pub domain_name: String,
    pub computer_name: String,
    pub user_name: String,
    pub checkin_at: time::SystemTime,
}


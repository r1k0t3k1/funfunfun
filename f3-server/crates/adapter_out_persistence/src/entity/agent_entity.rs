use application::domain::model::agent_model::AgentModel;
use sqlx::types::{Uuid, chrono};

#[derive(Clone)]
pub struct AgentEntity {
    pub id: Uuid,
    pub listener_id: Uuid,
    pub shared_secret: Vec<u8>, // modelへの詰替で[u8;32]へ変換
    pub process_id: i64, // postgresに合わせてi64で定義、モデルへの詰め替え時にu64に変換
    pub thread_id: i64,
    pub arch: String,
    pub is_admin: bool,
    pub process_name: String,
    pub os: String,
    pub domain_name: String,
    pub computer_name: String,
    pub user_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Into<AgentModel> for AgentEntity {
    fn into(self) -> AgentModel {
        AgentModel {
            id: self.id.into(),
            listener_id: self.listener_id.into(),
            shared_secret: self.shared_secret.try_into().unwrap(), // Postgres側で長さを保証してるのでunwrap可
            process_id: self.process_id as u64,
            thread_id: self.thread_id as u64,
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

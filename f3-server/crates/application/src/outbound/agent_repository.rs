use crate::{
    domain::model::{agent_model::AgentModel, id::{AgentId, ListenerId}}, outbound::error::RepositoryError
};

#[async_trait::async_trait]
pub trait AgentRepository: Send + Sync {
    async fn find_by_id(&self, agent_id: AgentId) -> Result<Option<AgentModel>, RepositoryError>;
    async fn list(&self) -> Result<Vec<AgentModel>, RepositoryError>;
    async fn list_by_listener_id(&self, listener_id: ListenerId) -> Result<Vec<AgentModel>, RepositoryError>;
    async fn insert(
        &self,
        listener_id: ListenerId,
        shared_secret: [u8;32],
        process_id: u64,
        thread_id: u64,
        arch: String,
        is_admin: bool,
        process_name: String,
        os: String,
        domain_name: String,
        computer_name: String,
        user_name: String,
    ) -> Result<AgentModel, RepositoryError>;

    async fn save(&self, agent: AgentModel) -> Result<AgentModel, RepositoryError>;
    async fn delete_by_id(&self, agent_id: AgentId) -> Result<(), RepositoryError>;
}

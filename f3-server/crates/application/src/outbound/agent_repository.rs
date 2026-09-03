use crate::{
    domain::model::{agent_model::AgentModel, id::AgentId}, outbound::error::RepositoryError
};

#[async_trait::async_trait]
pub trait AgentRepository: Send + Sync {
    async fn find_by_id(&self, agent_id: String) -> Result<Option<AgentModel>, RepositoryError>;
    async fn list(&self) -> Result<Vec<AgentModel>, RepositoryError>;
    async fn insert(
        &self,
    ) -> Result<AgentModel, RepositoryError>;

    async fn save(&self, agent: AgentModel) -> Result<AgentModel, RepositoryError>;
    async fn start_by_id(&self, agent_id: AgentId) -> Result<(), RepositoryError>;
    async fn stop_by_id(&self, agent_id: AgentId) -> Result<(), RepositoryError>;
    async fn delete_by_id(&self, agent_id: AgentId) -> Result<(), RepositoryError>;
}

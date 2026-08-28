use crate::{domain::model::{agent_model::{AgentId, AgentModel}, listener_model::ListenerId }, inbound::error::AgentUsecaseError};

#[async_trait::async_trait]
pub trait AgentUsecase: Send + Sync {
    async fn list_agents(&self, listener_id: ListenerId) -> Result<Vec<AgentModel>, AgentUsecaseError>;
    async fn get_agent(&self, agent_id: AgentId) -> Result<AgentModel, AgentUsecaseError>;
}

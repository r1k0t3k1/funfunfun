use crate::{
    domain::model::{agent_model::AgentModel, id::{AgentId, ListenerId}},
    outbound::error::C2Error,
};


#[async_trait::async_trait]
pub trait AgentControllerPort: Send + Sync {
    async fn list(&self, listener_id: ListenerId) -> Result<Vec<AgentModel>, C2Error>;
    async fn find_by_id(&self, agent_id: AgentId) -> Result<AgentModel, C2Error>;
}


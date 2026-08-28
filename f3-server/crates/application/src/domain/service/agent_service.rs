use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{domain::model::{agent_model::{AgentId, AgentModel}, listener_model::ListenerId}, inbound::{agent_usecase::AgentUsecase, error::AgentUsecaseError}, outbound::agent::AgentControllerPort};


#[derive(Clone)]
pub struct AgentService {
    agent_controller: Arc<Mutex<dyn AgentControllerPort>>,
}

#[async_trait::async_trait]
impl AgentUsecase for AgentService {
    async fn list_agents(&self,listener_id: ListenerId) -> Result<Vec<AgentModel> ,AgentUsecaseError> {
        self.agent_controller.lock()
            .await
            .list(listener_id)
            .await
            .map_err(|e| AgentUsecaseError::Unexpected(e))
    }

    async fn get_agent(&self,agent_id: AgentId) -> Result<AgentModel, AgentUsecaseError> {
        self.agent_controller.lock()
            .await
            .find_by_id(agent_id)
            .await
            .map_err(|e| AgentUsecaseError::Unexpected(e))
    }
}

impl AgentService {
    pub fn new(agent_controller: Arc<Mutex<dyn AgentControllerPort>>) -> Self {
        Self { agent_controller }
    }
}

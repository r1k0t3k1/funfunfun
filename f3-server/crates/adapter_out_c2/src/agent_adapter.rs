use application::{domain::model::{agent_model::{AgentId, AgentModel}, listener_model::ListenerId }, outbound::{agent::AgentControllerPort, error::C2Error}};
use crate::actor::c2_manager_actor::C2ManagerHandle;

pub struct AgentAdapter {
    c2_manager_handle: C2ManagerHandle,
}

impl AgentAdapter {
    pub fn new(c2_manager_handle: C2ManagerHandle) -> Self {
        Self { c2_manager_handle } 
    }
}

// アダプタではoneshotは取り扱わない
#[async_trait::async_trait]
impl AgentControllerPort for AgentAdapter {
    async fn list(&self, listener_id: ListenerId) -> Result<Vec<AgentModel>, C2Error> {
        self.c2_manager_handle
            .list_agent(listener_id)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }

    async fn find_by_id(&self, agent_id: AgentId) -> Result<AgentModel,C2Error> {
        self.c2_manager_handle
            .get_agent(agent_id)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }
}

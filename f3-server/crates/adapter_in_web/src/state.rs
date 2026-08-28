use std::sync::Arc;

use application::inbound::agent_usecase::AgentUsecase;
use application::inbound::auth_usecase::AuthUsecase;
use application::inbound::listener_usecase::ListenerUsecase;
use application::inbound::operator_usecase::OperatorUsecase;

#[derive(Clone)]
pub struct AppState {
    pub auth_usecase: Arc<dyn AuthUsecase>,
    pub listener_usecase: Arc<dyn ListenerUsecase>,
    pub operator_usecase: Arc<dyn OperatorUsecase>,
    pub agent_usecase: Arc<dyn AgentUsecase>,
}

impl AppState {
    pub fn new(
        auth_usecase: Arc<dyn AuthUsecase>,
        listener_usecase: Arc<dyn ListenerUsecase>,
        operator_usecase: Arc<dyn OperatorUsecase>,
        agent_usecase: Arc<dyn AgentUsecase>,
    ) -> Self {
        Self {
            auth_usecase,
            listener_usecase,
            operator_usecase,
            agent_usecase,
        }
    }
}

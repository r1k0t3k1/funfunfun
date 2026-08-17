use std::sync::Arc;

use application::port::inbound::auth_usecase::AuthUsecase;
use application::port::inbound::listener_usecase::ListenerUsecase;

#[derive(Clone)]
pub struct AppState {
    pub auth_usecase: Arc<dyn AuthUsecase>,
    pub listener_usecase: Arc<dyn ListenerUsecase>,
}

impl AppState {
    pub fn new(
        auth_usecase: Arc<dyn AuthUsecase>,
        listener_usecase: Arc<dyn ListenerUsecase>,
    ) -> Self {
        Self {
            auth_usecase,
            listener_usecase,
        }
    }
}

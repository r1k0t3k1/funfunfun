use crate::domain::model::listener_model::ListenerId;

use super::error::UsecaseError;

#[async_trait::async_trait]
pub trait ListenerUsecase {
    async fn list_listeners(&self) -> Vec<String>;

    async fn create_listener(
        &self,
        listener_type: String,
        lhost: String,
        lport: u16,
    ) -> Result<(), UsecaseError>;

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError>;

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError>;

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError>;
}

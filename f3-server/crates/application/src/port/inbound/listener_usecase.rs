use crate::domain::model::listener_model::{ListenerModel, ListenerId};

use super::error::ListenerUsecaseError;

#[async_trait::async_trait]
pub trait ListenerUsecase: Send + Sync {
    async fn list_listeners(&self) -> Vec<ListenerModel>;

    async fn create_listener(
        &self,
        listener_type: String,
        lhost: String,
        lport: u16,
    ) -> Result<(), ListenerUsecaseError>;

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;
}

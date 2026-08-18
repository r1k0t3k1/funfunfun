use crate::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};

use super::error::ListenerUsecaseError;

#[async_trait::async_trait]
pub trait ListenerUsecase: Send + Sync {
    async fn list_listeners(&self) -> Vec<ListenerModel>;

    async fn create_listener(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        protocol: ListenerProtocol,
    ) -> Result<(), ListenerUsecaseError>;

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError>;
}

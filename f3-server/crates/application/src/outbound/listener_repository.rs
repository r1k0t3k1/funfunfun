use crate::{
    domain::model::{id::ListenerId, listener_model::ListenerModel, role_model::Role},
    outbound::error::RepositoryError,
};

#[async_trait::async_trait]
pub trait ListenerRepository: Send + Sync {
    async fn find_by_id(&self, listener_id: String) -> Result<Option<ListenerModel>, RepositoryError>;
    async fn list(&self) -> Result<Vec<ListenerModel>, RepositoryError>;
    async fn insert(
        &self,
        id: String,
        password: String,
        name: String,
        description: String,
        role: Role,
        is_enabled: bool,
    ) -> Result<ListenerModel, RepositoryError>;

    async fn save(&self, listener: ListenerModel) -> Result<ListenerModel, RepositoryError>;
    async fn start_by_id(&self, listener_id: ListenerId) -> Result<(), RepositoryError>;
    async fn stop_by_id(&self, listener_id: ListenerId) -> Result<(), RepositoryError>;
    async fn delete_by_id(&self, listener_id: ListenerId) -> Result<(), RepositoryError>;
}

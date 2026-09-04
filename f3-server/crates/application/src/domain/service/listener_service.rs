use crate::{
    domain::model::{id::ListenerId, listener_model::{ListenerConfig, ListenerModel}},
    inbound::{error::ListenerUsecaseError, listener_usecase::ListenerUsecase}, outbound::{listener::ListenerControllerPort, listener_repository::ListenerRepository},
};
use std::{net::{Ipv4Addr, SocketAddr}, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ListenerService {
    listener_repository: Arc<dyn ListenerRepository>,
    listener_controller: Arc<Mutex<dyn ListenerControllerPort>>,
}

#[async_trait::async_trait]
impl ListenerUsecase for ListenerService {
    async fn list_listeners(&self) -> Result<Vec<ListenerModel>, ListenerUsecaseError> {
        self.listener_repository
            .list()
            .await
            .map_err(|e| ListenerUsecaseError::Repository(e))
    }

    async fn create_listener(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        config: ListenerConfig,
    ) -> Result<ListenerModel, ListenerUsecaseError> {
        let _: Ipv4Addr = lhost
            .parse()
            .map_err(|_| ListenerUsecaseError::InvalidAddress)?;
        
        let listener = self.listener_repository
            .insert(name.clone(), lhost, lport, config.clone())
            .await
            .map_err(|e| ListenerUsecaseError::Repository(e))?;

        let listener_controller = self.listener_controller.lock().await;

        let result = listener_controller.add(listener)
            .await
            .map_err(|e| ListenerUsecaseError::Unexpected(e))?;

        Ok(result)
    }

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        let _ = listener_controller
            .start(listener_id.clone())
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStart)?;

        let Some(mut listener) = self.listener_repository
            .find_by_id(listener_id)
            .await
            .map_err(|e| ListenerUsecaseError::Repository(e))? else {
                return Err(ListenerUsecaseError::NotFound);
        };
        
        listener.is_running = true;

        self.listener_repository
            .save(listener)
            .await
            .map(|_| ())
            .map_err(|e| ListenerUsecaseError::Repository(e))
    }

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        let _ = listener_controller
            .stop(listener_id.clone())
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStop);

        let Some(mut listener) = self.listener_repository
            .find_by_id(listener_id)
            .await
            .map_err(|e| ListenerUsecaseError::Repository(e))? else {
                return Err(ListenerUsecaseError::NotFound);
        };
        
        listener.is_running = false;

        self.listener_repository
            .save(listener)
            .await
            .map(|_| ())
            .map_err(|e| ListenerUsecaseError::Repository(e))
    }

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        let _ = listener_controller
            .remove(listener_id.clone())
            .await
            .map_err(|e| {
                log::error!("{e}");
                return ListenerUsecaseError::FailedToRemove;
            })?;

        self.listener_repository
            .delete_by_id(listener_id)
            .await
            .map_err(|e| ListenerUsecaseError::Repository(e))
    }
}

impl ListenerService {
    pub fn new(
        listener_repository: Arc<dyn ListenerRepository>,
        listener_controller: Arc<Mutex<dyn ListenerControllerPort>>
    ) -> Self {
        Self { listener_repository, listener_controller }
    }
}

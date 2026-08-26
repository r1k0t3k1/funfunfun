use crate::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    inbound::{error::ListenerUsecaseError, listener_usecase::ListenerUsecase}, outbound::listener::ListenerControllerPort,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ListenerService {
    listener_controller: Arc<Mutex<dyn ListenerControllerPort>>,
}

#[async_trait::async_trait]
impl ListenerUsecase for ListenerService {
    async fn list_listeners(&self) -> Result<Vec<ListenerModel>, ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        listener_controller
            .list()
            .await
            .map_err(|e| ListenerUsecaseError::Unexpected(e))
    }

    async fn create_listener(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        protocol: ListenerProtocol,
    ) -> Result<ListenerModel, ListenerUsecaseError> {
        let ipv4_addr = lhost
            .parse()
            .map_err(|_| ListenerUsecaseError::InvalidAddress)?;
        
        let addr = std::net::IpAddr::V4(ipv4_addr);
        let socket_addr = SocketAddr::new(addr, lport);

        let listener_controller = self.listener_controller.lock().await;

        listener_controller.add(name, socket_addr, protocol)
            .await
            .map_err(|e| ListenerUsecaseError::Unexpected(e))
    }

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        listener_controller
            .start(listener_id)
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStart)
    }

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        listener_controller
            .stop(listener_id)
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStop)
    }

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let listener_controller = self.listener_controller.lock().await;
        listener_controller
            .remove(listener_id)
            .await
            .map_err(|_| ListenerUsecaseError::FailedToRemove)
    }
}

impl ListenerService {
    pub fn new(listener_controller: Arc<Mutex<dyn ListenerControllerPort>>) -> Self {
        Self { listener_controller }
    }
}

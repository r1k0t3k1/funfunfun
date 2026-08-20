use crate::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    inbound::{error::ListenerUsecaseError, listener_usecase::ListenerUsecase},
    outbound::c2_manager::C2Manager,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ListenerService {
    c2_manager: Arc<Mutex<dyn C2Manager>>,
}

#[async_trait::async_trait]
impl ListenerUsecase for ListenerService {
    async fn list_listeners(&self) -> Vec<ListenerModel> {
        let c2_manager = self.c2_manager.lock().await;
        c2_manager.list_listener().await
    }
    async fn create_listener(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        protocol: ListenerProtocol,
    ) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        let ipv4_addr = lhost
            .parse()
            .map_err(|_| ListenerUsecaseError::InvalidAddress)?;
        let addr = std::net::IpAddr::V4(ipv4_addr);

        let socket_addr = SocketAddr::new(addr, lport);
        c2_manager
            .add_listener(name, socket_addr, protocol)
            .await
            .map_err(|_| ListenerUsecaseError::AddressAlreadyInUse)?;
        Ok(())
    }

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager
            .start(listener_id)
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStart)?;
        Ok(())
    }

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager
            .stop(listener_id)
            .await
            .map_err(|_| ListenerUsecaseError::FailedToStop)?;
        Ok(())
    }

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager
            .remove_listener(listener_id)
            .map_err(|_| ListenerUsecaseError::FailedToRemove)?;
        Ok(())
    }
}

impl ListenerService {
    pub fn new(c2_manager: Arc<Mutex<dyn C2Manager>>) -> Self {
        Self { c2_manager }
    }
}

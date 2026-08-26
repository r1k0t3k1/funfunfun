use std::net::SocketAddr;

use application::{domain::model::listener_model::{ListenerModel, ListenerProtocol}, outbound::{error::C2Error, listener::ListenerControllerPort}};
use uuid::Uuid;
use crate::actor::c2_manager_actor::C2ManagerHandle;

pub struct ListenerAdapter {
    c2_manager_handle: C2ManagerHandle,
}

impl ListenerAdapter {
    pub fn new(c2_manager_handle: C2ManagerHandle) -> Self {
        Self { c2_manager_handle } 
    }
}

// アダプタではoneshotは取り扱わない
#[async_trait::async_trait]
impl ListenerControllerPort for ListenerAdapter {
    async fn list(&self) -> Result<Vec<ListenerModel>, C2Error> {
        self.c2_manager_handle
            .list_listener()
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }

    async fn add(&self, name: String,addr: SocketAddr,protocol: ListenerProtocol) -> Result<ListenerModel, C2Error> {
        self.c2_manager_handle
            .add_listener(name, addr, protocol)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }

    async fn start(&self, listener_id: Uuid) -> Result<(), C2Error> {
        self.c2_manager_handle
            .start_listener(listener_id)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }

    async fn stop(&self, listener_id: Uuid) -> Result<(), C2Error> {
        self.c2_manager_handle
            .stop_listener(listener_id)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }

    async fn remove(&self, listener_id: Uuid) -> Result<(), C2Error> {
        self.c2_manager_handle
            .remove_listener(listener_id)
            .await
            .map_err(|e| C2Error::Unexpected(e))
    }
}

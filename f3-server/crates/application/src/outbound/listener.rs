use std::net::SocketAddr;

use crate::{
    domain::model::{id::ListenerId, listener_model::{ListenerModel, ListenerProtocol}},
    outbound::error::C2Error,
};

#[async_trait::async_trait]
pub trait ListenerControllerPort: Send + Sync {
    async fn list(&self) -> Result<Vec<ListenerModel>, C2Error>;
    async fn add(&self, name: String, addr: SocketAddr, protocol: ListenerProtocol) -> Result<ListenerModel, C2Error>;
    async fn start(&self, listener_id: ListenerId) -> Result<(), C2Error>;
    async fn stop(&self, listener_id: ListenerId) -> Result<(), C2Error>;
    async fn remove(&self, listener_id: ListenerId) -> Result<(), C2Error>;
}


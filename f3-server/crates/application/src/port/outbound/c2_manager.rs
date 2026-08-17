use std::net::SocketAddr;

use crate::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};

#[async_trait::async_trait]
pub trait C2Manager: Send + Sync {
    fn add_listener(
        &mut self,
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
    ) -> anyhow::Result<ListenerModel>;

    async fn start(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop_all(&mut self) -> anyhow::Result<()>;
    fn remove_listener(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn list_listener(&self) -> Vec<ListenerModel>;
}

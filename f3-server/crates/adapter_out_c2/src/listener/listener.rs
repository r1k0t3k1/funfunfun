use std::net::SocketAddr;

use application::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};

#[async_trait::async_trait]
pub trait ListenerPort: Send + Sync {
    fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self) -> anyhow::Result<()>;
    fn id(&self) -> ListenerId;
    fn name(&self) -> String;
    fn addr(&self) -> SocketAddr;
    fn protocol(&self) -> ListenerProtocol;
    fn listener_model(&self) -> ListenerModel;
}


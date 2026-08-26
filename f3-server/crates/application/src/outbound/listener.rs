use std::net::SocketAddr;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    outbound::{agent::{Agent, AgentId}, error::C2Error},
};

#[async_trait::async_trait]
pub trait ListenerControllerPort: Send + Sync {
    async fn list(&self) -> Result<Vec<ListenerModel>, C2Error>;
    async fn add(&self, name: String, addr: SocketAddr, protocol: ListenerProtocol) -> Result<ListenerModel, C2Error>;
    async fn start(&self, listener_id: Uuid) -> anyhow::Result<()>;
    async fn stop(&self, listener_id: Uuid) -> anyhow::Result<()>;
    async fn remove(&self, listener_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
pub trait ListenerPort: Send + Sync {
    async fn stop(&mut self) -> anyhow::Result<()>;
    fn id(&self) -> ListenerId;
    fn name(&self) -> String;
    fn addr(&self) -> SocketAddr;
    fn protocol(&self) -> ListenerProtocol;
    fn listener_model(&self) -> ListenerModel;
    fn set_join_handle(&mut self, join_handle: JoinHandle<()>);
    fn get_cancel_token(&mut self) -> CancellationToken;
    fn list_agents(&self) -> Vec<Agent>;
    fn add_agents(&mut self, agent: Agent) -> anyhow::Result<()>;
    fn remove_agent(&mut self, agent_id: AgentId) -> anyhow::Result<()>;
    fn remove_all_agent(&mut self);
}

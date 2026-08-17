use std::net::SocketAddr;

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use crate::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    port::outbound::agent::{Agent, AgentId},
};

#[async_trait::async_trait]
pub trait ListenerPort: Send + Sync {
    async fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self) -> anyhow::Result<()>;
    fn id(&self) -> ListenerId;
    fn name(&self) -> String;
    fn addr(&self) -> SocketAddr;
    fn protocol(&self) -> ListenerProtocol;
    fn listener_model(&self) -> ListenerModel;
    fn set_join_handle(&mut self, join_handle: JoinHandle<()>);
    fn set_cancel_token(&mut self, cancel_token: CancellationToken);
    fn get_cancel_token(&mut self) -> Option<CancellationToken>;
    fn list_agents(&self) -> Vec<Agent>;
    fn add_agents(&mut self, agent: Agent) -> anyhow::Result<()>;
    fn remove_agent(&mut self, agent_id: AgentId) -> anyhow::Result<()>;
    fn remove_all_agent(&mut self);
}

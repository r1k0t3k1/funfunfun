use std::net::SocketAddr;

use application::{domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol}, outbound::agent::{Agent, AgentId}};

#[async_trait::async_trait]
pub trait ListenerPort: Send + Sync {
    fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self) -> anyhow::Result<()>;
    fn id(&self) -> ListenerId;
    fn name(&self) -> String;
    fn addr(&self) -> SocketAddr;
    fn protocol(&self) -> ListenerProtocol;
    fn listener_model(&self) -> ListenerModel;
    fn list_agents(&self) -> Vec<Agent>;
    fn add_agents(&mut self, agent: Agent) -> anyhow::Result<()>;
    fn remove_agent(&mut self, agent_id: AgentId) -> anyhow::Result<()>;
    fn remove_all_agent(&mut self);
}


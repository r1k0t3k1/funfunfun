use std::collections::HashMap;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{agent::{Agent, AgentId}, error::DomainError};

pub type ListenerId = Uuid;



#[async_trait::async_trait]
pub trait Listener: Send + Sync {
    async fn start(&mut self) -> anyhow::Result<()>;
    async fn stop(&mut self) -> anyhow::Result<()>;
    fn set_join_handle(&mut self, join_handle: JoinHandle<()>);
    fn set_cancel_token(&mut self, cancel_token: CancellationToken);
    fn get_cancel_token(&mut self) ->  Option<CancellationToken>;
    fn list_agents(&self) ->  HashMap<AgentId, Agent>;
    fn add_agents(&mut self, agent: Agent) -> anyhow::Result<()>;
    fn remove_agent(&mut self, agent_id: AgentId) -> anyhow::Result<()>;
    fn remove_all_agent(&mut self);
}

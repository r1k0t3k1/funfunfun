use std::collections::VecDeque;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio::task::AbortHandle;
use tokio_util::sync::CancellationToken;

use crate::agent::{Agent, AgentEvent};

#[async_trait::async_trait]
pub trait ListenerManager: Send + Sync {
    fn add_listener(
        &mut self,
        name: String,
        addr: SocketAddr,
        notifier: UnboundedSender<Vec<AgentEvent>>,
    ) -> anyhow::Result<()>;
    async fn start_http(&mut self, listener_id: String) -> anyhow::Result<()>;
    async fn stop(&mut self, listener_id: String) -> anyhow::Result<()>;
    async fn stop_all(&mut self);
    async fn remove_listener(&mut self, listener_id: String) -> anyhow::Result<()>;
    fn list(&self) -> &HashMap<String, ListenerInfo>;
}

pub struct ListenerHandle {
    pub abort_handle: AbortHandle,
    pub token: CancellationToken,
    pub protocol: String,
}

impl ListenerHandle {
    pub fn new(abort_handle: AbortHandle, token: CancellationToken, protocol: String) -> Self {
        Self {
            abort_handle,
            token,
            protocol,
        }
    }
}

pub struct ListenerInfo {
    pub name: String,
    pub addr: SocketAddr,
    pub handle: Option<ListenerHandle>,
    pub notifier: UnboundedSender<Vec<AgentEvent>>,
    pub command_queue: Arc<Mutex<VecDeque<String>>>,
    pub agents: Vec<Agent>,
}

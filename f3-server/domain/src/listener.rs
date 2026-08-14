use std::{collections::HashMap, net::SocketAddr};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::{agent::{Agent, AgentId}, c2_manager::ListenerProtocol};

pub type ListenerId = Uuid;

pub struct Listener {
    pub id: ListenerId,
    pub name: String,
    pub protocol: ListenerProtocol,
    pub addr: SocketAddr,
    pub join_handle: Option<JoinHandle<()>>,
    pub cancel_token: CancellationToken,
    pub agents: HashMap<AgentId, Agent>,
}

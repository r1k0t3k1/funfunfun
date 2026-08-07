use std::{any, collections::HashMap, net::SocketAddr, sync::mpsc};

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

struct ListenerHandle {
    join_handle: JoinHandle<()>,
    token: CancellationToken,
    addr: SocketAddr,
    protocol: String,
}

//pub struct ListenerManager {
//    handles: HashMap<String, ListenerHandle>,
//    inbound_tx: mpsc::Sender<()>,
//}

#[async_trait::async_trait]
pub trait ListenerManager {
    fn new(inbound_tx: mpsc::Sender<()>) -> Self;
    fn start_http(&mut self, name: &str, addr: SocketAddr) -> anyhow::Result<()>;
    async fn stop(&mut self, name: &str) -> anyhow::Result<()>;
    async fn stop_all(&mut self);
    fn list(&self) -> Vec<ListenerInfo>;
}

pub struct ListenerInfo {
    pub name: String,
    pub protocol: String,
    pub addr: SocketAddr,
}

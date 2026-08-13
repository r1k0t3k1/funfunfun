use std::{net::SocketAddr, sync::{Arc, mpsc}};

use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

#[derive(Clone)]
struct ListenerHandle {
    token: CancellationToken,
    addr: SocketAddr,
    protocol: String,
    response_sender: mpsc::Sender<ListenerMessage>,
}

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

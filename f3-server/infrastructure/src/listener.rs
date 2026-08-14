mod http;

use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::Arc,
};

use domain::{agent::AgentEvent, listener::{ListenerHandle, ListenerId, ListenerInfo, ListenerManager}};
use tokio::sync::{
    Mutex,
    mpsc::{self, UnboundedSender},
};
use tokio_util::sync::CancellationToken;

use crate::listener;

pub struct ListenerManagerImpl {
    listeners: HashMap<ListenerId, ListenerInfo>,
    inbound_tx: mpsc::UnboundedSender<()>,
}

impl ListenerManagerImpl {
    pub fn new(inbound_tx: mpsc::UnboundedSender<()>) -> Self {
        Self {
            listeners: HashMap::new(),
            inbound_tx,
        }
    }
}

#[async_trait::async_trait]
impl ListenerManager for ListenerManagerImpl {
    fn add_listener(
        &mut self,
        name: String,
        addr: SocketAddr,
        notifier: UnboundedSender<Vec<AgentEvent>>,
    ) -> anyhow::Result<()> {
        self.listeners.insert(
            ListenerId::new_v4(),
            ListenerInfo {
                name,
                addr,
                handle: None,
                notifier,
                command_queue: Arc::new(Mutex::new(VecDeque::new())),
                agents: vec![],
            },
        );
        Ok(())
    }

    async fn start_http(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let listener = self
            .listeners
            .get_mut(&listener_id)
            .ok_or_else(|| anyhow::anyhow!("listener {listener_id} already running"))?;

        let token = CancellationToken::new();
        let addr = listener.addr.clone();
        let notifier = listener.notifier.clone();
        let command_queue = listener.command_queue.clone();

        let handle = tokio::spawn({
            let token = token.clone();
            async move {
                if let Err(e) = listener::http::serve(listener_id, addr, notifier, command_queue, token).await {
                    log::error!("HTTP Listener error: {e}")
                }
            }
        });

        listener.handle = Some(ListenerHandle::new(handle, token, "HTTP".to_string()));
        log::info!(
            "[+] HTTP listener {} started on {}",
            listener.name,
            listener.addr
        );
        Ok(())
    }

    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let listener = self
            .listeners
            .get_mut(&listener_id)
            .ok_or_else(|| anyhow::anyhow!("listener {listener_id} not found"))?;

        let handle = listener
            .handle
            .take()
            .ok_or_else(|| anyhow::anyhow!("listener {listener_id} not runnning"))?;

        handle.token.cancel();

        tokio::time::timeout(std::time::Duration::from_secs(10), handle.join_handle)
            .await
            .map_err(|_| anyhow::anyhow!("listener {listener_id} did not stop within 10s"))?
            .map_err(|e| anyhow::anyhow!("listener task panicked: {e}"))?;

        log::info!("[*] Listener {listener_id} stopped");
        Ok(())
    }

    async fn stop_all(&mut self) {
        let listener_ids: Vec<ListenerId> = self.listeners.keys().cloned().collect();
        for id in listener_ids {
            if let Err(e) = self.stop(id).await {
                eprintln!("{e}");
            }
        }
    }

    async fn remove_listener(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let l = self
            .listeners
            .iter()
            .find(|l| *l.0 == listener_id)
            .ok_or_else(|| anyhow::anyhow!(format!("Listener {listener_id} not found")))?;

        if l.1.handle.is_some() {
            return Err(anyhow::anyhow!(format!(
                "Listener {listener_id} is runnning"
            )));
        }

        self.listeners.remove_entry(&listener_id);
        Ok(())
    }

    fn list(&self) -> &HashMap<ListenerId, ListenerInfo> {
        &self.listeners
    }
}

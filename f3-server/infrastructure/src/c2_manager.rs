use std::{collections::HashMap, sync::Arc};
use anyhow::anyhow;
use domain::{c2_manager::{C2Manager, ListenerProtocol}, listener::{Listener, ListenerId}};
use tokio::sync::Mutex;
use uuid::Uuid;


pub struct C2ManagerImpl {
    pub listeners: HashMap<ListenerId, Arc<Mutex<dyn Listener>>>,
}

impl C2ManagerImpl {
    pub fn new() -> Self {
        Self { listeners: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl C2Manager for C2ManagerImpl {
    fn add_listener(&mut self, listener: Arc<Mutex<dyn Listener>>) -> anyhow::Result<()>  {
        self.listeners.insert(ListenerId::new_v4(), listener);
        Ok(())
    }

    async fn start(&mut self,listener_id: ListenerId) -> anyhow::Result<()> {
        let listener = self
            .listeners
            .get_mut(&listener_id)
            .ok_or_else(|| anyhow::anyhow!("listener {listener_id} already running"))?
            .lock()
            .await;

        let token = listener.cancel_token.clone();
        let addr = listener.addr.clone();

        let handle = match listener.protocol {
            ListenerProtocol::Http => {
                tokio::spawn({
                    let token = token.clone();
                    async move {
                        if let Err(e) = listener.start().await {
                            log::error!("HTTP Listener error: {e}")
                        }
                    }
                })
            },
            _ => todo!(),
        };

        listener.join_handle = Some(handle);

        log::info!(
            "[+] HTTP listener {} started on {}",
            listener.name,
            listener.addr
        );
        Ok(())
    }

    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let handle = self.listeners
            .get_mut(&listener_id)
            .ok_or(anyhow!("listener {listener_id} not found"))?
            .join_handle
            .take()
            .ok_or(anyhow!("listener {listener_id} not runnning"))?;

        tokio::time::timeout(std::time::Duration::from_secs(10), handle)
            .await
            .map_err(|_| anyhow::anyhow!("listener {listener_id} did not stop within 10s"))?
            .map_err(|e| anyhow::anyhow!("listener task panicked: {e}"))?;

        log::info!("[*] Listener {listener_id} stopped");
        Ok(())
    }

    async fn stop_all(&mut self) -> anyhow::Result<()> {
        let ids: Vec<Uuid> = self.listeners.keys().copied().collect();
        for id in ids {
            self.stop(id).await?;
        }
        Ok(())
    }

    fn remove_listener(&mut self,listener_id: ListenerId) -> anyhow::Result<()>  {
        self.listeners.remove(&listener_id);
        Ok(())
    }

    fn list_listener(&self) ->  &HashMap<ListenerId,Listener>  {
        &self.listeners
    }
}

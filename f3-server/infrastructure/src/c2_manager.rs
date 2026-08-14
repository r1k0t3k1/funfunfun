use std::collections::HashMap;
use futures::future::join_all;
use domain::{c2_manager::{C2Manager, ListenerProtocol}, listener::{Listener, ListenerId}};

use crate::listener;

pub struct C2ManagerImpl {
    pub listeners: HashMap<ListenerId, Listener>,
}

#[async_trait::async_trait]
impl C2Manager for C2ManagerImpl {
    fn add_listener(&mut self, listener: Listener) -> anyhow::Result<()>  {
        self.listeners.insert(ListenerId::new_v4(), listener);
        Ok(())
    }

    async fn start(&mut self,listener_id: ListenerId) -> anyhow::Result<()> {
        let listener = self
            .listeners
            .get_mut(&listener_id)
            .ok_or_else(|| anyhow::anyhow!("listener {listener_id} already running"))?;

        let token = listener.cancel_token.clone();
        let addr = listener.addr.clone();

        let handle = match listener.protocol {
            ListenerProtocol::Http => {
                tokio::spawn({
                    let token = token.clone();
                    async move {
                        if let Err(e) = listener::http::serve(listener_id, addr, notifier, command_queue, token).await {
                            log::error!("HTTP Listener error: {e}")
                        }
                    }
                })
            },
            _ => todo!(),
        };

        listener.handle = handle;
        log::info!(
            "[+] HTTP listener {} started on {}",
            listener.name,
            listener.addr
        );
        Ok(())
    }

    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let Some(l) = self.listeners.get_mut(&listener_id).take() else {
            return Ok(());
        };
        let handle = l
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
        let fut =  self.listeners.iter().map(|(&id, _)| async move {
            self.stop(id)
        });
        join_all(fut).await;
    }

    fn remove_listener(&mut self,listener_id: ListenerId) -> anyhow::Result<()>  {
        self.listeners.remove(&listener_id);
        Ok(())
    }

    fn list_listener(&mut self) ->  &HashMap<ListenerId,Listener>  {
        &self.listeners
    }
}

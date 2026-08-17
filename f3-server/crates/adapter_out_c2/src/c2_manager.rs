use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use anyhow::anyhow;
use application::port::outbound::c2_manager::C2Manager;
use application::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};
use application::port::outbound::listener::{self, ListenerPort};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::listener::http::HttpListener;

pub struct C2ManagerImpl {
    pub listeners: HashMap<ListenerId, Arc<Mutex<dyn ListenerPort>>>,
}

impl C2ManagerImpl {
    pub fn new() -> Self {
        Self { listeners: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl C2Manager for C2ManagerImpl {
    fn add_listener(&mut self, name: String, addr: SocketAddr, protocol: ListenerProtocol) -> anyhow::Result<ListenerModel>  {
        match protocol {
            ListenerProtocol::Http => {
                let listener = HttpListener::new(name.clone(), addr, protocol.clone());
                let listener_model = Into::<ListenerModel>::into(&listener);
                self.listeners.insert(listener.id, Arc::new(Mutex::new(listener)));
                Ok(listener_model)
            },
            _ => todo!(),
        }
    }

    async fn start(&mut self,listener_id: ListenerId) -> anyhow::Result<()> {
        let listener_arc = self
            .listeners
            .get(&listener_id)
            .ok_or(anyhow!("Listener {listener_id} not found"))?;
       
        let mut listener = listener_arc.lock().await;

        let name = listener.name();
        let addr = listener.addr();

        let handle = match listener.protocol() {
            ListenerProtocol::Http => {
                tokio::spawn({
                    let listener_arc = listener_arc.clone();
                    async move {
                        let mut listener = listener_arc.lock().await;
                        if let Err(e) = listener.start().await {
                            log::error!("HTTP Listener error: {e}")
                        }
                    }
                })
            },
            _ => todo!(),
        };

        listener.set_join_handle(handle);

        log::info!(
            "[+] HTTP listener {} started on {}",
            name,
            addr,
        );
        Ok(())
    }

    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let mut listener = self.listeners
            .get_mut(&listener_id)
            .ok_or(anyhow!("listener {listener_id} not found"))?
            .lock()
            .await;

        listener.stop().await?;

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
        let _ = self.listeners.remove(&listener_id);
        Ok(())
    }

    async fn list_listener(&self) ->  Vec<ListenerModel>  {
        let mut lm = vec![];
        for l in self.listeners.values() {
            lm.push(l.lock().await.listener_model());
        }
        lm
    }
}

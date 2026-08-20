use anyhow::anyhow;
use application::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};
use application::outbound::c2_manager::C2Manager;
use application::outbound::listener::ListenerPort;
use std::sync::Arc;
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::listener::http::HttpListener;

pub struct C2ManagerImpl {
    pub listeners: HashMap<ListenerId, Arc<Mutex<dyn ListenerPort>>>,
}

impl C2ManagerImpl {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
}

#[async_trait::async_trait]
impl C2Manager for C2ManagerImpl {
    async fn add_listener(
        &mut self,
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
    ) -> anyhow::Result<ListenerModel> {
        match protocol {
            ListenerProtocol::Http => {
                let listener = HttpListener::new(name.clone(), addr, protocol.clone());
                let listener_model = Into::<ListenerModel>::into(&listener);

                for l in self.listeners.values() {
                    if l.lock().await.addr() == addr {
                        return Err(anyhow!("Address already in use."));
                    }
                }

                self.listeners
                    .insert(listener.id, Arc::new(Mutex::new(listener)));
                Ok(listener_model)
            }
            _ => todo!(),
        }
    }

    async fn start(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let listener_arc = self
            .listeners
            .get(&listener_id)
            .ok_or(anyhow!("Listener {listener_id} not found"))?;

        let protocol = { listener_arc.lock().await.protocol() };

        match protocol {
            ListenerProtocol::Http => HttpListener::spawn_server(listener_arc.clone()).await?,
            _ => todo!(),
        };
        Ok(())
    }

    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let mut listener = self
            .listeners
            .get_mut(&listener_id)
            .ok_or(anyhow!("listener {listener_id} not found"))?
            .lock()
            .await;

        listener.stop().await?;

        Ok(())
    }

    async fn stop_all(&mut self) -> anyhow::Result<()> {
        let ids: Vec<Uuid> = self.listeners.keys().copied().collect();
        for id in ids {
            self.stop(id).await?;
        }
        Ok(())
    }

    fn remove_listener(&mut self, listener_id: ListenerId) -> anyhow::Result<()> {
        let _ = self.listeners.remove(&listener_id);
        Ok(())
    }

    async fn list_listener(&self) -> Vec<ListenerModel> {
        let mut lm = vec![];
        for l in self.listeners.values() {
            lm.push(l.lock().await.listener_model());
        }
        lm
    }
}

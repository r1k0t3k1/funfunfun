use std::{
    collections::HashMap
};

use crate::listener::{ListenerId, Listener};

pub enum ListenerProtocol {
    Tcp,
    Http,
    Https,
}

#[async_trait::async_trait]
pub trait C2Manager: Send + Sync {
    fn add_listener(
        &mut self,
        listener: impl Listener,
    ) -> anyhow::Result<()>;
    
    async fn start(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop_all(&mut self) -> anyhow::Result<()>;
    fn remove_listener(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    fn list_listener(&self) -> &HashMap<ListenerId, impl Listener>;
}

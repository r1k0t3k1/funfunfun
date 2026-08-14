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
pub trait C2Manager {
    fn add_listener(
        &mut self,
        listener: Listener,
    ) -> anyhow::Result<()>;
    
    async fn start(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    async fn stop_all(&mut self);
    fn remove_listener(&mut self, listener_id: ListenerId) -> anyhow::Result<()>;
    fn list_listener(&mut self) -> &HashMap<ListenerId, Listener>;
}

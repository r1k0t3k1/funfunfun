use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};

use domain::{
    agent::Agent, command::{CommandReceiver, CommandSender}, listener::ListenerManager
};
use tokio::sync::Mutex;

use crate::error::UsecaseError;

#[derive(Clone)]
pub struct ListenerUsecase {
    command_tx: Arc<dyn CommandSender>,
    command_rx: Arc<dyn CommandReceiver>,
    listener_manager: Arc<Mutex<dyn ListenerManager>>,
}

impl ListenerUsecase {
    pub fn new(
        command_tx: Arc<dyn CommandSender>,
        command_rx: Arc<dyn CommandReceiver>,
        listener_manager: Arc<Mutex<dyn ListenerManager>>,
    ) -> Self {
        Self {
            command_tx,
            command_rx,
            listener_manager,
        }
    }

    pub async fn list_listeners(&self) -> Vec<(String, SocketAddr)> {
        let listener_manager = self.listener_manager.lock().await;
        listener_manager
            .list()
            .iter()
            .map(|(n, l)| (n.clone(), l.addr))
            .collect()
    }

    pub async fn create_listener(
        &self,
        listener_type: String,
        lhost: String,
        lport: u16,
    ) -> Result<(), UsecaseError> {
        let host = lhost
            .parse::<Ipv4Addr>()
            .map_err(|e| UsecaseError::Validation(format!("{}: {}", "lhost".to_string(), e)))?;

        let addr = SocketAddr::new(IpAddr::V4(host), lport);
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        
        spawn_worker(rx, self.clone(), handle_checkin_request);

        self.listener_manager
            .lock()
            .await
            .add_listener(listener_type, addr, tx)
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn start_listener(&self, listener_id: String) -> Result<(), UsecaseError> {
        self.listener_manager
            .lock()
            .await
            .start_http(listener_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn stop_listener(&self, listener_id: String) -> Result<(), UsecaseError> {
        self.listener_manager
            .lock()
            .await
            .stop(listener_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn remove_listener(&self, listener_id: String) -> Result<(), UsecaseError> {
        self.listener_manager
            .lock()
            .await
            .remove_listener(listener_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    //pub async fn checkin_agent_to_listener(&self, agent: Agent) -> Result<(), UsecaseError> {
    //    self.listener_manager
    //        .lock()
    //        .await
    //}
}

fn spawn_worker<F, Fut, C>(mut rx: tokio::sync::mpsc::UnboundedReceiver<(String, String)>, ctx: C, f: F)
where
    C: Clone + Send + 'static,
    F: Fn((String, String), C) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            f(req, ctx.clone()).await;
        }
    });
}

async fn handle_checkin_request(checkin_info: (String, String), lu: ListenerUsecase) {
    log::info!("[+] Checkin request received on listener id {}. shared secret is {}", checkin_info.0, checkin_info.1);
    //lu.listener_manager.
}

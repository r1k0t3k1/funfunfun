use std::sync::Arc;

use domain::{
    agent::AgentEvent, c2_manager::C2Manager, listener::{Listener, ListenerId}, model::packet_model::CheckinResponse
};
use tokio::sync::Mutex;

use crate::error::UsecaseError;

#[derive(Clone)]
pub struct ListenerUsecase {
    c2_manager: Arc<Mutex<dyn C2Manager>>,
}

impl ListenerUsecase {
    pub fn new(
        c2_manager: Arc<Mutex<dyn C2Manager>>,
    ) -> Self {
        Self {
            c2_manager,
        }
    }

    pub async fn list_listeners(&self) -> Vec<String> {
        let c2_manager = self.c2_manager.lock().await;
        c2_manager.list_listener().keys().map(|k| k.to_string()).collect()
    }

    pub async fn create_listener(
        &self,
        listener_type: String,
        lhost: String,
        lport: u16,
    ) -> Result<(), UsecaseError> {
        self.c2_manager.lock().await.add_listener(Listener::new())
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn start_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError> {
        self.c2_manager
            .lock()
            .await
            .start(listener_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError> {
        self.c2_manager
            .lock()
            .await
            .stop(listener_id)
            .await
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

    pub async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), UsecaseError> {
        self.c2_manager
            .lock()
            .await
            .remove_listener(listener_id)
            .map_err(|e| UsecaseError::Unexpected(e.into()))
    }

}

fn spawn_worker<F, Fut, C>(mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<AgentEvent>>, ctx: C, f: F)
where
    C: Clone + Send + 'static,
    F: Fn(Vec<AgentEvent>, C) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            f(req, ctx.clone()).await;
        }
    });
}

async fn handle_request(agent_events: Vec<AgentEvent>, lu: ListenerUsecase) {
    log::info!("[+] Request received. event count {}", agent_events.len());
    for evt in agent_events {
        match evt {
            AgentEvent::Checkin { agent_public_key, response_sender } => {
                log::info!("[+] Checkin public key: {agent_public_key:?}");
                let res = CheckinResponse::new();
                response_sender.send(res).unwrap();
            },
            AgentEvent::CheckinComplete { agent_info, response_sender } => log::info!("[+] AgentInfo: {agent_info}"),
        }
    }
    //lu.listener_manager.
}

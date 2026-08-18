use crate::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    port::{
        inbound::{error::ListenerUsecaseError, listener_usecase::ListenerUsecase},
        outbound::c2_manager::C2Manager,
    },
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct ListenerService {
    c2_manager: Arc<Mutex<dyn C2Manager>>,
}

#[async_trait::async_trait]
impl ListenerUsecase for ListenerService {
    async fn list_listeners(&self) -> Vec<ListenerModel> {
        let c2_manager = self.c2_manager.lock().await;
        c2_manager.list_listener().await
    }
    async fn create_listener(
        &self,
        name: String,
        lhost: String,
        lport: u16,
        protocol: ListenerProtocol,
    ) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        let ipv4_addr = lhost.parse()
            .map_err(|e| ListenerUsecaseError::InvalidAddress)?;
        let addr = std::net::IpAddr::V4(ipv4_addr);

        let socket_addr = SocketAddr::new(addr, lport);
        c2_manager.add_listener(name, socket_addr, protocol)
            .await
            .map_err(|e| ListenerUsecaseError::AddressAlreadyInUse)?;
        Ok(())
    }

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager.start(listener_id)
            .await
            .map_err(|e| ListenerUsecaseError::FailedToStart)?;
        Ok(())
    }

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager.stop(listener_id)
            .await
            .map_err(|e| ListenerUsecaseError::FailedToStop)?;
        Ok(())
    }

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        let mut c2_manager = self.c2_manager.lock().await;
        c2_manager.remove_listener(listener_id)
            .map_err(|e| ListenerUsecaseError::FailedToRemove)?;
        Ok(())
    }
}

impl ListenerService {
    pub fn new(c2_manager: Arc<Mutex<dyn C2Manager>>) -> Self {
        Self { c2_manager }
    }
}

//fn spawn_worker<F, Fut, C>(mut rx: tokio::sync::mpsc::UnboundedReceiver<Vec<AgentEvent>>, ctx: C, f: F)
//where
//    C: Clone + Send + 'static,
//    F: Fn(Vec<AgentEvent>, C) -> Fut + Send + 'static,
//    Fut: Future<Output = ()> + Send,
//{
//    tokio::spawn(async move {
//        while let Some(req) = rx.recv().await {
//            f(req, ctx.clone()).await;
//        }
//    });
//}
//
//async fn handle_request(agent_events: Vec<AgentEvent>, lu: ListenerUsecase) {
//    log::info!("[+] Request received. event count {}", agent_events.len());
//    for evt in agent_events {
//        match evt {
//            AgentEvent::Checkin { agent_public_key, response_sender } => {
//                log::info!("[+] Checkin public key: {agent_public_key:?}");
//                let res = CheckinResponse::new();
//                response_sender.send(res).unwrap();
//            },
//            AgentEvent::CheckinComplete { agent_info, response_sender } => log::info!("[+] AgentInfo: {agent_info}"),
//        }
//    }
//    //lu.listener_manager.
//}

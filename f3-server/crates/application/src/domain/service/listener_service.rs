use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{domain::model::listener_model::{ListenerModel, ListenerId}, port::{inbound::{error::ListenerUsecaseError, listener_usecase::ListenerUsecase}, outbound::c2_manager::C2Manager}};

#[derive(Clone)]
pub struct ListenerService {
    c2_manager: Arc<Mutex<dyn C2Manager>>,
}

#[async_trait::async_trait]
impl ListenerUsecase for ListenerService {
    async fn list_listeners(&self) -> Vec<ListenerModel> {
        let m = self.c2_manager.lock().await;
        m.list_listener().await
    }
    async fn create_listener(&self, listener_type: String, lhost: String, lport: u16) -> Result<(), ListenerUsecaseError> {
        todo!()
    }

    async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
    }

    async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
    }

    async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
    }
}

impl ListenerService {
    pub fn new(
        c2_manager: Arc<Mutex<dyn C2Manager>>,
    ) -> Self {
        Self {
            c2_manager,
        }
    }

    pub async fn create_listener(
        &self,
        listener_type: String,
        lhost: String,
        lport: u16,
    ) -> Result<(), ListenerUsecaseError> {
        //self.c2_manager.lock().await.add_listener(Listener::new())
        //    .map_err(|e| UsecaseError::Unexpected(e.into()))
        todo!()
    }

    pub async fn start_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
        //self.c2_manager
        //    .lock()
        //    .await
        //    .start(listener_id)
        //    .await
        //    .map_err(|e| ListenerUsecaseError::Unexpected(e.into()))
    }

    pub async fn stop_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
        //self.c2_manager
        //    .lock()
        //    .await
        //    .stop(listener_id)
        //    .await
        //    .map_err(|e| ListenerUsecaseError::Unexpected(e.into()))
    }

    pub async fn remove_listener(&self, listener_id: ListenerId) -> Result<(), ListenerUsecaseError> {
        todo!()
        //self.c2_manager
        //    .lock()
        //    .await
        //    .remove_listener(listener_id)
        //    .map_err(|e| ListenerUsecaseError::Unexpected(e.into()))
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

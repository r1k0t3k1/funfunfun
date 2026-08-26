use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use application::{domain::model::listener_model::{ListenerModel, ListenerProtocol}, outbound::{agent::AgentId, listener::ListenerPort}};
use tokio::sync::{Mutex, mpsc};
use uuid::Uuid;

use crate::{actor::agent_actor::AgentHandle, c2_inner_message::C2InnerMessage, listener::http::HttpListener};

pub struct ListenerActor {
    receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
    agent_handles: HashMap<AgentId, AgentHandle>,
    listener: Arc<Mutex<dyn ListenerPort>>,
}

impl ListenerActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
        listener: Arc<Mutex<dyn ListenerPort>>,
    ) -> Self {
        let agent_handles = HashMap::new();
        Self { receiver, agent_handles, listener }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: C2InnerMessage) {
        match msg {
            C2InnerMessage::StartListener { listener_id, reply } => {
                let _ = reply.send(self.listener.lock().await.start());
            },
            C2InnerMessage::StopListener { listener_id, reply } => {
                let _ = reply.send(self.listener.lock().await.stop().await);
            },
            _ => {},
        }
    }
}

pub struct ListenerHandle {
    pub sender: mpsc::UnboundedSender<C2InnerMessage>,
    pub model: ListenerModel,
}

impl ListenerHandle {
    pub fn new(id: Uuid, name: String, addr: SocketAddr, protocol: ListenerProtocol, c2_manager_sender: mpsc::UnboundedSender<C2InnerMessage>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let listener = Arc::new(Mutex::new(HttpListener::new(name.clone(), addr, protocol.clone(), c2_manager_sender.clone()))); // TODO
        let mut actor = ListenerActor::new(receiver, listener);
        tokio::spawn(async move { actor.run().await });
        let model = ListenerModel::new(id, name, addr, protocol);
        Self { sender, model }
    }
}

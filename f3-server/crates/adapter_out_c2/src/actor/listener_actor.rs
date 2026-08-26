use std::{collections::HashMap, net::SocketAddr};

use application::{domain::model::listener_model::{ListenerModel, ListenerProtocol}, outbound::{agent::AgentId, listener::ListenerPort}};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{actor::agent_actor::AgentHandle, c2_inner_message::C2InnerMessage, listener::http::HttpListener};

pub struct ListenerActor {
    receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
    agent_handles: HashMap<AgentId, AgentHandle>,
    listener: Box<dyn ListenerPort>
}

impl ListenerActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
        listener: Box<dyn ListenerPort>,
    ) -> Self {
        let agent_handles = HashMap::new();
        Self { receiver, agent_handles, listener }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: C2InnerMessage) {
        match msg {
            C2InnerMessage::ListListener { reply } => todo!(),
            C2InnerMessage::AddListener { name, addr, protocol, reply } => todo!(),
            C2InnerMessage::StartListener { listener_id, reply } => todo!(),
            C2InnerMessage::StopListener { listener_id, reply } => todo!(),
            C2InnerMessage::RemoveListener { listener_id, reply } => todo!(),
        }
    }
}

pub struct ListenerHandle {
    pub sender: mpsc::UnboundedSender<C2InnerMessage>,
    pub model: ListenerModel,
}

impl ListenerHandle {
    pub fn new(id: Uuid, name: String, addr: SocketAddr, protocol: ListenerProtocol) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let listener = Box::new(HttpListener::new(name.clone(), addr, protocol.clone())); // TODO
        let mut actor = ListenerActor::new(receiver, listener);
        tokio::spawn(async move { actor.run().await });
        let model = ListenerModel::new(id, name, addr, protocol);
        Self { sender, model }
    }
}

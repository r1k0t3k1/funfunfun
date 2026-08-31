use std::net::SocketAddr;

use application::domain::model::listener_model::{ListenerModel, ListenerProtocol};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{c2_message::{C2Message, ListenerMessage}, listener::{http::HttpListener, listener::ListenerPort}};

pub struct ListenerActor {
    receiver: mpsc::UnboundedReceiver<ListenerMessage>,
    listener: Box<dyn ListenerPort>,
}

impl ListenerActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<ListenerMessage>,
        listener: Box<dyn ListenerPort>,
    ) -> Self {
        Self { receiver, listener }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: ListenerMessage) {
        match msg {
            ListenerMessage::Start { reply } => {
                let _ = reply.send(self.listener.start());
            },

            ListenerMessage::Stop { reply } => {
                let _ = reply.send(self.listener.stop().await);
            },

            ListenerMessage::Query { reply } => {
                let _ = reply.send(Ok(self.listener.listener_model()));
            },
        }
    }
}

pub struct ListenerHandle {
    pub sender: mpsc::UnboundedSender<ListenerMessage>,
    pub model: ListenerModel,
}

impl ListenerHandle {
    pub fn new(id: Uuid, name: String, addr: SocketAddr, protocol: ListenerProtocol, c2_manager_sender: mpsc::UnboundedSender<C2Message>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let listener = Box::new(HttpListener::new(id, name.clone(), addr, protocol.clone(), c2_manager_sender.clone()));
        let mut actor = ListenerActor::new(receiver, listener);
        tokio::spawn(async move { actor.run().await });
        let model = ListenerModel::new(id, name, addr, protocol);
        Self { sender, model }
    }
}

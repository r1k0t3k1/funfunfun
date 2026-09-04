use application::domain::model::listener_model::ListenerModel;
use tokio::sync::mpsc;

use crate::{c2_message::{C2Message, ListenerMessage}, listener::{http::HttpListener, listener::Listener}};

pub struct ListenerActor {
    receiver: mpsc::UnboundedReceiver<ListenerMessage>,
    listener: Box<dyn Listener>,
}

impl ListenerActor {
    pub fn new(
        receiver: mpsc::UnboundedReceiver<ListenerMessage>,
        listener: Box<dyn Listener>,
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
        }
    }
}

pub struct ListenerHandle {
    pub sender: mpsc::UnboundedSender<ListenerMessage>,
}

impl ListenerHandle {
    pub fn new(model: ListenerModel, c2_manager_sender: mpsc::UnboundedSender<C2Message>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        let listener = match model.config {
            application::domain::model::listener_model::ListenerConfig::Http {..} => {
                Box::new(HttpListener::new(model, c2_manager_sender))
            },
            application::domain::model::listener_model::ListenerConfig::Tcp {  } => todo!(),
            application::domain::model::listener_model::ListenerConfig::Dns {  } => todo!(),
        };

        let mut actor = ListenerActor::new(receiver, listener);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }
}

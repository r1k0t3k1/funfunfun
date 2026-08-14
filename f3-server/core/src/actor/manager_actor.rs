use std::collections::HashMap;

use domain::agent::AgentId;
use tokio::sync::mpsc;

use crate::{actor::session_actor::SessionActor, message::agent_message::AgentMessage};

#[derive(Debug)]
pub struct ManagerActor {
    pub receiver: mpsc::UnboundedReceiver<AgentMessage>,  // listenerからの通知受信用
    pub sessions: HashMap<AgentId, SessionActor>,
}

impl ManagerActor {
    pub fn new(receiver: mpsc::UnboundedReceiver<AgentMessage>) -> Self {
        Self { receiver, sessions: HashMap::new() }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
           self.handle_message(msg); 
        }
    }

    fn handle_message(&mut self, msg: AgentMessage) {
        match msg {
            AgentMessage::CheckinRequest() => {},
            AgentMessage::CheckinCompleteRequest() => {},
        }
    }
}

#[derive(Debug, Clone)]
pub struct ManagerHandle {
    pub sender: mpsc::UnboundedSender<AgentMessage>,  // ManagerActorへの送信用 
}

impl ManagerHandle {
    fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = ManagerActor::new(receiver);

        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }
}

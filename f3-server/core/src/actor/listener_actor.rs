use tokio::sync::mpsc;

use crate::{actor::manager_actor::ManagerHandle, message::agent_message::AgentMessage};

#[derive(Debug)]
pub struct ListenerActor {
    pub receiver: mpsc::UnboundedReceiver<AgentMessage>,
    pub manager_handle: ManagerHandle,
}

impl ListenerActor {
    pub fn new(receiver: mpsc::UnboundedReceiver<AgentMessage>, manager_handle: ManagerHandle) -> Self {
        Self { receiver, manager_handle }
    }

    pub async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    } 

    fn handle_message(&mut self, msg: AgentMessage) {
        match msg {
           AgentMessage::CheckinRequest() => (), // TODO manager_handle経由でManagerActorに通知
           AgentMessage::CheckinCompleteRequest() => (), // TODO
        }
    }
}


#[derive(Debug, Clone)]
pub struct ListenerHandle {
    pub sender: mpsc::UnboundedSender<AgentMessage>,
}

impl ListenerHandle {
    pub fn new(manager_handle: ManagerHandle) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = ListenerActor::new(receiver, manager_handle);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }
}

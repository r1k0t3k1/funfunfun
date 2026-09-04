use application::domain::model::agent_model::AgentModel;
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::c2_message::{AgentMessage, C2Message};

pub struct AgentActor {
    agent: AgentModel,
    sender: mpsc::UnboundedSender<C2Message>,
    receiver: mpsc::UnboundedReceiver<AgentMessage>,
    task_queue: Vec<String>, // TODO
}

impl AgentActor {
    pub fn new(agent: AgentModel, sender: mpsc::UnboundedSender<C2Message>, receiver: mpsc::UnboundedReceiver<AgentMessage>) -> Self {
        Self {
            agent,
            sender,
            receiver,
            task_queue: vec![] 
        }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: AgentMessage) {
        match msg {
            AgentMessage::CheckinComplete => {
                log::info!("Agent checkin {}", self.agent.id);
            }
        }
    }
}

pub struct AgentHandle {
    pub sender: mpsc::UnboundedSender<AgentMessage>,
}

impl AgentHandle {
    pub fn new(model: AgentModel, c2_manager_sender: UnboundedSender<C2Message>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = AgentActor::new(model, c2_manager_sender, receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }

    }
}

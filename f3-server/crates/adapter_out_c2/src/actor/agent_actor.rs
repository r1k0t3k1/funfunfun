use application::domain::model::{agent_model::{AgentModel, AgentStatus}, id::{AgentId, ListenerId}};
use rand::Rng;
use tokio::sync::mpsc::{self, UnboundedSender};
use x25519_dalek::{PublicKey, StaticSecret, x25519};

use crate::c2_message::{AgentMessage, C2Message};

pub struct AgentActor {
    id: AgentId,
    parent_listener_id: ListenerId,
    status: AgentStatus,
    session_pubkey: [u8;32],
    shared_secret: [u8;32],
    sender: mpsc::UnboundedSender<C2Message>,
    receiver: mpsc::UnboundedReceiver<AgentMessage>,
    task_queue: Vec<String>, // TODO
}

impl Into<AgentModel> for &mut AgentActor {
    fn into(self) -> AgentModel {
        AgentModel { 
            id: self.id.clone(),
            listener_id: self.parent_listener_id.clone(),
            status: self.status.clone(),
            session_pubkey: self.session_pubkey,
            shared_secret: self.shared_secret,
        }
    }
}

impl AgentActor {
    pub fn new(id: AgentId, parent_listener_id: ListenerId, received_pubkey: [u8;32], sender: mpsc::UnboundedSender<C2Message>, receiver: mpsc::UnboundedReceiver<AgentMessage>) -> Self {
        let mut secret_bytes = [0_u8; 32];
        rand::rng().fill_bytes(&mut secret_bytes);
        let secret = StaticSecret::from(secret_bytes);
        let session_pubkey = PublicKey::from(&secret).to_bytes();
        let shared_secret = x25519(secret.to_bytes(), received_pubkey);

        log::info!("Generated session_secret: {:?}", secret.to_bytes());
        log::info!("Generated session_pubkey: {:?}", session_pubkey);
        log::info!("Generated shared_secret: {:?}", shared_secret);

        Self {
            id,
            parent_listener_id,
            status: AgentStatus::CheckinProcessStarted, 
            session_pubkey,
            shared_secret,
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
            AgentMessage::Query { reply } => {
                let _ = reply.send(Ok(self.into()));
            },
            AgentMessage::CheckinComplete => {
                log::info!("Agent status updated to {:?}", self.status);
                self.status =  AgentStatus::CheckinProcessCompleted;
            }
        }
    }
}

pub struct AgentHandle {
    pub parent_listener_id: ListenerId,
    pub model: AgentModel,
    pub sender: mpsc::UnboundedSender<AgentMessage>,
}

impl AgentHandle {
    pub fn new(parent_listener_id: ListenerId, agent_id: AgentId, received_pubkey: [u8;32], c2_manager_sender: UnboundedSender<C2Message>) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = AgentActor::new(agent_id, parent_listener_id.clone(), received_pubkey, c2_manager_sender, receiver);
        let model = AgentModel {
            id: actor.id.clone(),
            listener_id: parent_listener_id.clone(),
            status: actor.status.clone(),
            session_pubkey: actor.session_pubkey.clone(),
            shared_secret: actor.shared_secret.clone(),
        };
        tokio::spawn(async move { actor.run().await });
        Self { parent_listener_id, model, sender }

    }
}

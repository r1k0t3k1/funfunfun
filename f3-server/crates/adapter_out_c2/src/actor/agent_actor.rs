use rand::Rng;
use tokio::sync::mpsc;
use x25519_dalek::{PublicKey, StaticSecret, x25519};

use crate::c2_message::AgentMessage;

#[derive(Debug)]
pub enum AgentStatus {
    CheckinProcessStarted,
    CheckinProcessCompleted,
}

pub struct AgentActor {
    receiver: mpsc::UnboundedReceiver<AgentMessage>,
    status: AgentStatus,
    pubkey: [u8; 32],
    secret: [u8; 32],
    shared_secret: [u8; 32],
    task_queue: Vec<String>, // TODO
}

impl AgentActor {
    pub fn new(receiver: mpsc::UnboundedReceiver<AgentMessage>) -> Self {
        let mut secret_bytes = [0_u8; 32];
        rand::rng().fill_bytes(&mut secret_bytes);
        let secret = StaticSecret::from(secret_bytes).to_bytes();

        let pubkey = PublicKey::from(secret).to_bytes();


        Self { receiver, status: AgentStatus::CheckinProcessStarted, pubkey, secret, shared_secret: [0_u8; 32], task_queue: vec![] }
    }

    pub fn set_shared_secret(&mut self, secret: [u8; 32]) {
        let session_pubkey = PublicKey::from(&session_secret_key);

        let shared_secret = x25519(session_secret_key.to_bytes(), agent_pubkey);
        self.shared_secret = secret;
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: AgentMessage) {
        match msg {
            AgentMessage::CheckinComplete => {
                self.status =  AgentStatus::CheckinProcessCompleted;
                log::info!("Agent status updated to {:?}", self.status);
            }
            AgentMessage::QuerySecret { reply } => {
                reply.send(Ok(self.shared_secret));
            },
        }
    }
}

pub struct AgentHandle {
    pub sender: mpsc::UnboundedSender<AgentMessage>,
}

impl AgentHandle {
    pub fn new(pubkey: [u8; 32]) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = AgentActor::new(receiver, pubkey);
        tokio::spawn(async move { actor.run().await });
        Self { sender }

    }
}

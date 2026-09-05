use application::domain::model::agent_model::AgentModel;
use application::domain::model::id::{AgentId, ListenerId};
use application::domain::model::listener_model::ListenerModel;
use application::outbound::agent_repository::AgentRepository;
use application::outbound::error::C2Error;
use application::outbound::listener_repository::ListenerRepository;
use rand::RngExt;
use x25519_dalek::{PublicKey, StaticSecret, x25519};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

use crate::actor::agent_actor::AgentHandle;
use crate::actor::listener_actor::ListenerHandle;
use crate::c2_message::{C2Message, ListenerMessage};

pub struct C2ManagerActor {
    sender: mpsc::UnboundedSender<C2Message>, // C2ManagerにListenerやAgentからメッセージを送信する用
    receiver: mpsc::UnboundedReceiver<C2Message>,
    listener_handles: HashMap<ListenerId, ListenerHandle>,
    agent_handles: HashMap<AgentId, AgentHandle>,
    listener_repository: Arc<dyn ListenerRepository>,
    agent_repository: Arc<dyn AgentRepository>,
}

impl C2ManagerActor {
    pub fn new(
        sender: mpsc::UnboundedSender<C2Message>,
        receiver: mpsc::UnboundedReceiver<C2Message>,
        listener_repository: Arc<dyn ListenerRepository>,
        agent_repository: Arc<dyn AgentRepository>,
    ) -> Self {
        Self { 
            sender, 
            receiver, 
            listener_handles: HashMap::new(), 
            agent_handles: HashMap::new(),
            listener_repository,
            agent_repository,
        }
    }

    async fn run(mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: C2Message) {
        match msg {
            C2Message::AddListener { listener, reply } => {
                let listener_id = ListenerId::new();
                let listener_handle = ListenerHandle::new(listener.clone(), self.sender.clone());
                self.listener_handles.insert(listener_id, listener_handle);
                let _ = reply.send(Ok(listener));
            },

            C2Message::RemoveListener { listener_id, reply } => {
                self.listener_handles.remove(&listener_id);
                let _ = reply.send(Ok(()));
            },

            C2Message::ListListeners { reply } => {
                let result = self.listener_repository.list()
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"));

                let _ = reply.send(result);
            },

            C2Message::ListAgent { listener_id, reply } => {
                let result = self.agent_repository
                    .list_by_listener_id(listener_id)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"));

                let _ = reply.send(result);
            },

            C2Message::QueryAgent { agent_id, reply } => {
                let result = self.agent_repository
                    .find_by_id(agent_id)
                    .await
                    .map_err(|e| anyhow::anyhow!("{e}"));

                let _ = reply.send(result);
            },

            C2Message::AddAgent { 
                    listener_id, 
                    process_id, 
                    thread_id, 
                    arch, 
                    is_admin, 
                    process_name, 
                    os, 
                    domain_name, 
                    computer_name, 
                    user_name,
                    reply,
                    received_pubkey 
            } => {
                let mut secret_bytes = [0_u8; 32];
                rand::rng().fill(&mut secret_bytes);
                let secret = StaticSecret::from(secret_bytes);
                let session_pubkey = PublicKey::from(&secret).to_bytes();
                let shared_secret = x25519(secret.to_bytes(), received_pubkey);

                let agent = self.agent_repository.insert(
                    listener_id, 
                    shared_secret,
                    process_id, 
                    thread_id, 
                    arch, 
                    is_admin, 
                    process_name, 
                    os, 
                    domain_name, 
                    computer_name, 
                    user_name
                )
                .await
                .map_err(|e| C2Error::Repository(e)).unwrap(); // TODO

                let agent_handle = AgentHandle::new(agent.clone(),  self.sender.clone());
                self.agent_handles.insert(agent.clone().id, agent_handle);

                let _ = reply.send(Ok((agent, session_pubkey)));
            },

            C2Message::ToListener { listener_id, msg } => {
                // listenerに転送するだけ
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    log::error!("listener {listener_id} not found, but message received.");
                    return ;
                };
                let _ = l.sender.send(msg);
            },

            C2Message::ToAgent { agent_id, msg } => {
                // agentに転送するだけ
                let Some(a) = self.agent_handles.get(&agent_id) else {
                    log::error!("agent {agent_id} not found, but ToAgent message received.");
                    return;
                };
                let _ = a.sender.send(msg);
            },
        }
    }
}

#[derive(Clone)]
pub struct C2ManagerHandle {
    sender: mpsc::UnboundedSender<C2Message>,
}

impl C2ManagerHandle {
    pub fn new(
        listener_repository: Arc<dyn ListenerRepository>,
        agent_repository: Arc<dyn AgentRepository>,
    ) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = C2ManagerActor::new(sender.clone(), receiver, listener_repository, agent_repository);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    pub async fn add_listener(
        &self, 
        listener: ListenerModel,
    ) -> anyhow::Result<ListenerModel> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::AddListener { listener, reply };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn list_listener(&self) -> anyhow::Result<Vec<ListenerModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::ListListeners { reply };
        let _ = self.sender.send(msg);

        rx.await?
    }

    pub async fn start_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::ToListener { 
            listener_id, 
            msg: ListenerMessage::Start { reply } 
        };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn stop_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::ToListener { 
            listener_id, 
            msg: ListenerMessage::Stop { reply } 
        };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn remove_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::RemoveListener { listener_id, reply };
        let _ = self.sender.send(msg);

        rx.await?
    }

    pub async fn list_agent(&self, listener_id: ListenerId) -> anyhow::Result<Vec<AgentModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::ListAgent { listener_id, reply };
        let _ = self.sender.send(msg);

        rx.await?
    }

    pub async fn get_agent(&self, agent_id: AgentId) -> anyhow::Result<Option<AgentModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::QueryAgent { agent_id, reply };
        let _ = self.sender.send(msg);

        rx.await?
    }
}

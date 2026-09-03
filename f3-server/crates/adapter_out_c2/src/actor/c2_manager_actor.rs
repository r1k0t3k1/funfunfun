use application::domain::model::agent_model::AgentModel;
use application::domain::model::id::{AgentId, ListenerId};
use application::domain::model::listener_model::{ListenerModel, ListenerProtocol};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::{mpsc, oneshot};

use crate::actor::agent_actor::AgentHandle;
use crate::actor::listener_actor::ListenerHandle;
use crate::c2_message::{AgentMessage, C2Message, ListenerMessage};

pub struct C2ManagerActor {
    sender: mpsc::UnboundedSender<C2Message>, // C2ManagerにListenerやAgentからメッセージを送信する用
    receiver: mpsc::UnboundedReceiver<C2Message>,
    listener_handles: HashMap<ListenerId, ListenerHandle>,
    agent_handles: HashMap<AgentId, AgentHandle>,
}

impl C2ManagerActor {
    pub fn new(
        sender: mpsc::UnboundedSender<C2Message>,
        receiver: mpsc::UnboundedReceiver<C2Message>,
    ) -> Self {
        Self { sender, receiver, listener_handles: HashMap::new(), agent_handles: HashMap::new() }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: C2Message) {
        match msg {
            C2Message::QueryListener { listener_id, reply } => {
                // SQLXでいい
                let model = self.listener_handles.get(&listener_id)
                    .map(|l| l.model.clone())
                    .ok_or(anyhow::anyhow!("Listener {listener_id} not found"));
                let _ = reply.send(model);
            },

            C2Message::ListListener { reply } => {
                // SQLXでいい
                let models: Vec<ListenerModel> = self.listener_handles
                    .values()
                    .into_iter()
                    .map(|l| l.model.clone())
                    .collect();
                let _ = reply.send(Ok(models));
            },

            C2Message::AddListener { name, addr, protocol, reply } => {
                let listener_id = ListenerId::new();
                let listener_handle = ListenerHandle::new(listener_id.clone(), name, addr, protocol, self.sender.clone());
                let model = listener_handle.model.clone();
                self.listener_handles.insert(listener_id, listener_handle);
                let _ = reply.send(Ok(model));
            },

            C2Message::RemoveListener { listener_id, reply } => {
                self.listener_handles.remove(&listener_id);
                let _ = reply.send(Ok(()));
            },

            C2Message::ListAgent { listener_id: _, reply } => {
                let _ = reply.send(Ok(self.agent_handles.values().map(|a| a.model.clone()).collect()));
            },

            C2Message::AddAgent { listener_id, reply, received_pubkey } => {
                let agent_id = AgentId::new();
                let agent_handle = AgentHandle::new(listener_id, agent_id.clone(), received_pubkey, self.sender.clone());
                self.agent_handles.insert(agent_id.clone(), agent_handle);
                log::info!("received pubkey: {:?}", received_pubkey);

                let agent_handle = self.agent_handles.get(&agent_id).unwrap();
                let msg = AgentMessage::Query { reply };
                let _ = agent_handle.sender.send(msg);
            },

            C2Message::QueryAgent { agent_id, reply } => {
                let Some(a) = self.agent_handles.get(&agent_id) else {
                    log::error!("agent {agent_id} not found, but QueryAgent message received.");
                    return;
                };
                let msg = AgentMessage::Query { reply };
                let _ = a.sender.send(msg);
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
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let mut actor = C2ManagerActor::new(sender.clone(), receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    pub async fn add_listener(
        &self, 
        name: String, 
        addr: SocketAddr, 
        protocol: ListenerProtocol, 
    ) -> anyhow::Result<ListenerModel> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::AddListener { name, addr, protocol, reply };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn list_listener(&self) -> anyhow::Result<Vec<ListenerModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::ListListener { reply };
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

    pub async fn get_agent(&self, agent_id: AgentId) -> anyhow::Result<AgentModel> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::QueryAgent { agent_id, reply };
        let _ = self.sender.send(msg);

        rx.await?
    }
}

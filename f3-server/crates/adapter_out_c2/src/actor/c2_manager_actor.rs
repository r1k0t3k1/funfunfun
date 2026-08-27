use anyhow::anyhow;
use application::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::actor::listener_actor::ListenerHandle;
use crate::c2_message::{AgentMessage, C2Message, ListenerMessage};

pub struct C2ManagerActor {
    sender: mpsc::UnboundedSender<C2Message>, // C2ManagerにListenerからメッセージを送信する用
    receiver: mpsc::UnboundedReceiver<C2Message>,
    listener_handles: HashMap<ListenerId, ListenerHandle>,
}

impl C2ManagerActor {
    pub fn new(
        sender: mpsc::UnboundedSender<C2Message>,
        receiver: mpsc::UnboundedReceiver<C2Message>,
    ) -> Self {
        Self { sender, receiver, listener_handles: HashMap::new() }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: C2Message) {
        match msg {
            C2Message::Listener(msg) => self.handle_listener_message(msg),
            C2Message::Agent(msg) => self.handle_agent_message(msg),
        }
    }

    fn handle_listener_message(&mut self, msg: ListenerMessage) {
        match msg {
            ListenerMessage::List { reply } => {
                let listeners = self.listener_handles
                    .values()
                    .into_iter()
                    .map(|l| l.model.clone())
                    .collect();

                let _ = reply.send(Ok(listeners));
            },
            ListenerMessage::Add { name, addr, protocol, reply } => {
                let id = Uuid::new_v4();
                let listener_handle = ListenerHandle::new(id, name.clone(), addr, protocol.clone(), self.sender.clone());
                self.listener_handles.insert(id, listener_handle);
                let _ = reply.send(Ok(ListenerModel::new(id, name, addr, protocol)));
            },
            ListenerMessage::Start { listener_id, reply } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    let _ = reply.send(Err(anyhow!("Listener not found: {listener_id}")));
                    return;
                };
                let msg = ListenerMessage::Start { listener_id, reply };
                let _ = l.sender.send(msg);
            },
            ListenerMessage::Stop { listener_id, reply } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    let _ = reply.send(Err(anyhow!("Listener not found: {listener_id}")));
                    return;
                };
                let msg = ListenerMessage::Stop { listener_id, reply };
                let _ = l.sender.send(msg);
            },
            ListenerMessage::Remove { listener_id, reply } => {
                let _ = self.listener_handles.remove(&listener_id);
                let _ = reply.send(Ok(()));
            },
            ListenerMessage::AgentCheckinReceived { listener_id, agent_id, agent_pubkey } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    return;
                };
                let _ = l.sender.send(ListenerMessage::CheckinAgent { agent_id, agent_pubkey });
            },
            ListenerMessage::AgentCheckinCompleted { listener_id, agent_id } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    return;
                };
                let _ = l.sender.send(ListenerMessage::CompleteCheckinAgent { listener_id, agent_id });
            },
            ListenerMessage::QuerySecret { listener_id, agent_id, reply } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    return;
                };
                let _ = l.sender.send(ListenerMessage::Query { listener_id, agent_id, reply });

            },
            ListenerMessage::CheckinAgent { agent_id, agent_pubkey } => unreachable!(),
            ListenerMessage::CompleteCheckinAgent { listener_id, agent_id } => unreachable!(),
            ListenerMessage::Query { listener_id, agent_id, reply } => todo!(),
        }
    }

    fn handle_agent_message(&mut self, msg: AgentMessage) {
        todo!()
    }
}

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
        let msg = C2Message::Listener(ListenerMessage::Add { name, addr, protocol, reply });
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn list_listener(&self) -> anyhow::Result<Vec<ListenerModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::Listener(ListenerMessage::List { reply });
        let _ = self.sender.send(msg);

        rx.await?
    }

    pub async fn start_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::Listener(ListenerMessage::Start { listener_id, reply });
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn stop_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::Listener(ListenerMessage::Stop { listener_id, reply });
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn remove_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2Message::Listener(ListenerMessage::Remove { listener_id, reply });
        let _ = self.sender.send(msg);

        rx.await?
    }
}

use anyhow::anyhow;
use application::domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol};
use std::{collections::HashMap, net::SocketAddr};
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::actor::listener_actor::ListenerHandle;
use crate::c2_inner_message::C2InnerMessage;

pub struct C2ManagerActor {
    sender: mpsc::UnboundedSender<C2InnerMessage>, // C2ManagerにListenerからメッセージを送信する用
    receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
    listener_handles: HashMap<ListenerId, ListenerHandle>,
}

impl C2ManagerActor {
    pub fn new(
        sender: mpsc::UnboundedSender<C2InnerMessage>,
        receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
    ) -> Self {
        Self { sender, receiver, listener_handles: HashMap::new() }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }

    fn handle_message(&mut self, msg: C2InnerMessage) {
        match msg {
            C2InnerMessage::ListListener { reply } => {
                let listeners = self.listener_handles
                    .values()
                    .into_iter()
                    .map(|l| l.model.clone())
                    .collect();

                let _ = reply.send(Ok(listeners));
            },
            C2InnerMessage::AddListener { 
                name, 
                addr, 
                protocol, 
                reply 
            } => {
                let id = Uuid::new_v4();
                let listener_handle = ListenerHandle::new(id, name.clone(), addr, protocol.clone(), self.sender.clone());
                self.listener_handles.insert(id, listener_handle);
                let _ = reply.send(Ok(ListenerModel::new(id, name, addr, protocol)));
            },
            C2InnerMessage::StartListener { listener_id, reply } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    let _ = reply.send(Err(anyhow!("Listener not found: {listener_id}")));
                    return;
                };
                let msg = C2InnerMessage::StartListener { listener_id, reply };
                let _ = l.sender.send(msg);
            },
            C2InnerMessage::StopListener { listener_id, reply } => {
                let Some(l) = self.listener_handles.get(&listener_id) else {
                    let _ = reply.send(Err(anyhow!("Listener not found: {listener_id}")));
                    return;
                };
                let msg = C2InnerMessage::StopListener { listener_id, reply };
                let _ = l.sender.send(msg);
            },
            C2InnerMessage::RemoveListener { listener_id, reply } => {
                let _ = self.listener_handles.remove(&listener_id);
                let _ = reply.send(Ok(()));
            },
            C2InnerMessage::ListenerRequestReceived => { log::info!("listner requesst received") },
        }
    }
}

pub struct C2ManagerHandle {
    sender: mpsc::UnboundedSender<C2InnerMessage>,
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
        let msg = C2InnerMessage::AddListener { name, addr, protocol, reply };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn list_listener(&self) -> anyhow::Result<Vec<ListenerModel>> {
        let (reply, rx) = oneshot::channel();
        let msg = C2InnerMessage::ListListener { reply };
        let _ = self.sender.send(msg);

        rx.await?
    }

    pub async fn start_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2InnerMessage::StartListener { listener_id, reply };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn stop_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        let (reply, rx) = oneshot::channel();
        let msg = C2InnerMessage::StopListener { listener_id, reply };
        let _ = self.sender.send(msg);
        
        rx.await?
    }

    pub async fn remove_listener(&self, listener_id: ListenerId) -> anyhow::Result<()> {
        //let (reply, rx) = oneshot::channel();
        //let msg = C2InnerMessage::StopListener { listener_id, reply };
        //let _ = self.sender.send(msg);
        //let _ = rx.await?;

        let (reply, rx) = oneshot::channel();
        let msg = C2InnerMessage::RemoveListener { listener_id, reply };
        let _ = self.sender.send(msg);

        rx.await?
    }
}

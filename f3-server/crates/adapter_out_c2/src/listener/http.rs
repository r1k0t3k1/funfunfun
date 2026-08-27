use actix_web::{App, HttpResponse, HttpServer, Responder, dev::ServerHandle, http::StatusCode, web};
use application::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    outbound::{
        agent::{Agent, AgentId},
    },
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{sync::mpsc::{self, UnboundedSender}, task::JoinHandle};
use uuid::Uuid;

use crate::{c2_message::{C2Message, ListenerMessage}, listener::listener::ListenerPort};

pub struct HttpListener {
    pub id: Uuid,
    pub name: String,
    pub addr: SocketAddr,
    pub protocol: ListenerProtocol,
    pub handle: Option<ServerHandle>,
    pub server_task: Option<JoinHandle<anyhow::Result<()>>>,
    pub sender: mpsc::UnboundedSender<C2Message>,
}

impl Drop for HttpListener {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            tokio::spawn(handle.stop(true));
        }
    }
}

impl HttpListener {
    pub fn new(name: String, addr: SocketAddr, protocol: ListenerProtocol, sender: mpsc::UnboundedSender<C2Message>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            addr,
            protocol,
            handle: None,
            server_task: None,
            sender,
        }
    }

    pub fn spawn(&mut self) -> anyhow::Result<()> {
        let sender = self.sender.clone();
        let server = HttpServer::new(move || { 
            App::new()
                .app_data(web::Data::new(sender.clone()))
                .route("/favicon.ico", web::post().to(dispatch))
            })
            .workers(1)
            .bind(self.addr)?
            .shutdown_timeout(10)
            .disable_signals()
            .run();
        
        self.handle = Some(server.handle());
        self.server_task = Some(tokio::spawn(async move { server.await.map_err(anyhow::Error::from) }));
        Ok(())
    }

    pub async fn stop(&mut self) -> anyhow::Result<()> { 
        let handle = self.handle.take()
            .ok_or_else(|| anyhow::anyhow!("Listener not started"))?;

        let _ = tokio::spawn(tokio::time::timeout(Duration::from_secs(5), handle.stop(true))).await;
        Ok(())
    }
}

async fn dispatch(sender: web::Data<UnboundedSender<C2Message>>) -> impl Responder {
    let _ = sender.send(C2Message::Listener(ListenerMessage::ListenerRequestReceived));
    HttpResponse::new(StatusCode::TEMPORARY_REDIRECT)
}

impl Into<ListenerModel> for &HttpListener {
    fn into(self) -> ListenerModel {
        ListenerModel {
            id: self.id,
            name: self.name.clone(),
            addr: self.addr,
            protocol: self.protocol.clone(),
        }
    }
}

#[async_trait::async_trait]
impl ListenerPort for HttpListener {
    fn id(&self) -> ListenerId {
        self.id
    }
    fn name(&self) -> String {
        self.name.to_string()
    }
    fn addr(&self) -> SocketAddr {
        self.addr
    }
    fn protocol(&self) -> ListenerProtocol {
        self.protocol.clone()
    }
    fn listener_model(&self) -> ListenerModel {
        ListenerModel {
            id: self.id(),
            name: self.name(),
            addr: self.addr(),
            protocol: self.protocol(),
        }
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.spawn()
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        self.stop().await
    }

    fn list_agents(&self) -> Vec<Agent> {
        todo!()
    }

    fn add_agents(&mut self, agent: Agent) -> anyhow::Result<()> {
        todo!()
    }

    fn remove_agent(&mut self, agent_id: AgentId) -> anyhow::Result<()> {
        todo!()
    }

    fn remove_all_agent(&mut self) {
        todo!()
    }
}


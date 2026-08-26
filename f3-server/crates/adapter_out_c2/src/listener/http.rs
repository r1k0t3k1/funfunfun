use actix_web::{App, HttpResponse, HttpServer, Responder, dev::ServerHandle, http::StatusCode, web};
use anyhow::anyhow;
use application::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    outbound::{
        agent::{Agent, AgentId},
        listener::ListenerPort,
    },
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::mpsc::{self, UnboundedSender};
use uuid::Uuid;

use crate::c2_inner_message::C2InnerMessage;

pub struct HttpListener {
    pub id: Uuid,
    pub name: String,
    pub addr: SocketAddr,
    pub protocol: ListenerProtocol,
    pub handle: Option<ServerHandle>,
    pub sender: mpsc::UnboundedSender<C2InnerMessage>,
}

impl Drop for HttpListener {
    fn drop(&mut self) {
    }
}

impl HttpListener {
    pub fn new(name: String, addr: SocketAddr, protocol: ListenerProtocol, sender: mpsc::UnboundedSender<C2InnerMessage>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            addr,
            protocol,
            handle: None,
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
            .bind(self.addr)?
            .shutdown_timeout(10)
            .disable_signals()
            .run();
        
        let handle = server.handle();
        let _server_task = tokio::spawn(server);
        self.handle = Some(handle);
        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> { 
        let handle = self.handle.clone()
            .ok_or_else(|| anyhow!("Listener not started"))?;

        tokio::spawn(tokio::time::timeout(Duration::from_secs(5), handle.stop(true)));
        Ok(())
    }
}

async fn dispatch(sender: web::Data<UnboundedSender<C2InnerMessage>>) -> impl Responder {
    log::info!("test");
    let _ = sender.send(C2InnerMessage::ListenerRequestReceived);
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

    fn stop(&mut self) -> anyhow::Result<()> {
        self.stop()
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


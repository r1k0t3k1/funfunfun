use std::collections::VecDeque;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::pin;
use std::sync::Arc;

use domain::agent::AgentEvent;
use domain::listener::ListenerId;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use http_body_util::{BodyExt, Full, Limited};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;

use crate::packet::Packet;

const MAX_BODY: usize = 1024 * 4096; // 4 MiB

pub async fn serve(
    listener_id: ListenerId,
    addr: SocketAddr,
    inbound_tx: UnboundedSender<Vec<AgentEvent>>,
    command_queue: Arc<Mutex<VecDeque<String>>>,
    token: CancellationToken,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    log::info!("HTTP Listener listening on http://{addr} ...");

    let http = http1::Builder::new();
    let graceful = GracefulShutdown::new();
    let state = inbound_tx;

    let mut shutdown = pin!(token.cancelled());

    loop {
        tokio::select! {
            accepted = listener.accept() => {
                let (stream, _) = match accepted {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("accept error: {e}");
                        continue;
                    }
                };

                let io = TokioIo::new(stream);
                let state = state.clone();
                let queue = command_queue.clone();
                let listener_id = listener_id.clone();

                let svc = service_fn(move |req| {
                    let state = state.clone();
                    let queue = queue.clone();
                    let listener_id = listener_id.clone();
                    async move { handle_request(req, listener_id, state, queue).await }
                });


                let conn = http.serve_connection(io, svc);
                let fut = graceful.watch(conn);

                tokio::spawn(async move {
                    if let Err(e) = fut.await {
                        log::error!("connection error: {e}");
                    }
                });
            }
            _ = &mut shutdown => {
                log::info!("HTTP listener on {addr}: shutdown signal received");
                drop(listener);
                break;
            }
        }
    }
    Ok(())
}

async fn handle_request(
    req: Request<Incoming>,
    listener_id: ListenerId,
    state: UnboundedSender<Vec<AgentEvent>>,
    command_queue: Arc<Mutex<VecDeque<String>>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/favicon.ico") => {
            let body = Limited::new(req.into_body(), MAX_BODY);
            let bytes = match body.collect().await {
                Ok(b) => b.to_bytes().to_vec(),
                Err(_) => vec![],
            };
            
            let packet: Packet = serde_cbor::from_slice(&bytes).unwrap(); // TODO
            let agent_events = packet.try_into().unwrap(); // TODO
            state.send(agent_events).unwrap();
            
            // response packet作成
            let response = Packet::new(vec![]);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Full::new(Bytes::from(serde_cbor::to_vec(&response).unwrap()))) // TODO
                .unwrap()
            )
        },
        _ => Ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::new()))
                .unwrap()
            ),
    }
}

use std::collections::VecDeque;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::pin::pin;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::sync::CancellationToken;

use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::server::graceful::GracefulShutdown;
use base64::engine::{general_purpose, Engine as _};
use x25519_dalek::{EphemeralSecret, PublicKey};

pub async fn serve(
    listener_id: String,
    addr: SocketAddr,
    inbound_tx: UnboundedSender<(String, String)>,
    command_queue: Arc<Mutex<VecDeque<String>>>,
    token: CancellationToken,
) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&addr).await?;
    //let make_svc = service_fn(handle_request);

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
    listener_id: String,
    state: UnboundedSender<(String, String)>,
    command_queue: Arc<Mutex<VecDeque<String>>>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/checkin") => {
            match req.headers().get("Authorization") {
                Some(authorization) => {
                    let agent_public_bytes = general_purpose::STANDARD.decode(authorization.to_str().unwrap()).unwrap();
                    let agent_public_32bytes: [u8;32] = agent_public_bytes.try_into().unwrap();
                    let agent_public = PublicKey::from(agent_public_32bytes);

                    let listener_secret = EphemeralSecret::random();
                    let listener_public = PublicKey::from(&listener_secret);

                    let shared_secret = listener_secret.diffie_hellman(&agent_public);
                    let encoded_shared_secret = general_purpose::STANDARD.encode(shared_secret.as_bytes());
                    log::info!("shared secret: {encoded_shared_secret}");
                    state.send((listener_id, encoded_shared_secret)).unwrap(); // リクエスト受信を通知
                
                    let encoded_listener_public = general_purpose::STANDARD.encode(listener_public.as_bytes());
                    return Ok(Response::new(encoded_listener_public.into()));
                },
                None => {
                    let mut queue = command_queue.lock().await;
                    let res = queue
                        .pop_front()
                        .or(Some("no more commands".to_string()))
                        .unwrap();
                    return Ok(Response::new(res.into()));
                },
            }
        }
        _ => Ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::new()))
                .unwrap()
            ),
    }
}

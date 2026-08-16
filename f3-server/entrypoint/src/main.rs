use adapter_in_web::state::AppState;
use adapter_in_web::server::run;
use env_logger;
use actix_web::web;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = AppState::new(auth_usecase, listener_usecase);
    run(app_state)?
}

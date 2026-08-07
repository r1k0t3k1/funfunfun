use std::env;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http, middleware::Logger, web};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod apidocs;
mod config;
mod dto;
mod error;
mod handler;
mod middleware;
mod route;
mod state;

use config::DatabaseConfig;
use state::AppState;

use crate::route::configure_route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = web::Data::new(AppState::new(DatabaseConfig::new().connect_database_with()));

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder
        .set_private_key_file("app/src/resource/certificate/key.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_file("app/src/resource/certificate/cert.pem", SslFiletype::PEM)
        .unwrap();

    let port = env::var("PORT").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:1420")
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".rust-lang.org"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(http::header::CONTENT_TYPE)
            .allowed_header(http::header::ACCEPT)
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .wrap(cors)
            .configure(configure_route(state.clone()))
    })
    .bind_openssl(format!("0.0.0.0:{port}"), builder)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, http::StatusCode, test};

    use super::*;

    #[actix_web::test]
    async fn test_health_check_db() {
    }
}

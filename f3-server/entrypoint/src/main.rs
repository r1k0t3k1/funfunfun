use std::sync::Arc;

use adapter_in_web::server::run;
use adapter_in_web::state::AppState;
use adapter_out_c2::actor::c2_manager_actor::C2ManagerHandle;
use adapter_out_c2::agent_adapter::{self, AgentAdapter};
use adapter_out_c2::listener_adapter::ListenerAdapter;
use adapter_out_persistence::password_hasher_impl::Argon2PasswordHasher;
use adapter_out_persistence::repository::operator_repository_impl::OperatorRepositoryImpl;
use adapter_out_persistence::repository::session_repository_impl::SessionRepositoryImpl;
use application::domain::service::agent_service::AgentService;
use application::domain::service::auth_service::AuthService;
use application::domain::service::listener_service::ListenerService;

use application::domain::service::operator_service::OperatorService;
use env_logger;
use sqlx::PgPool;
use sqlx::postgres::PgConnectOptions;
use tokio::sync::Mutex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let options = PgConnectOptions::new()
        .host("localhost".into())
        .port(5432)
        .username("f3".into())
        .password("funfunfun".into())
        .database("f3".into());

    let connection = PgPool::connect_lazy_with(options);
    let operator_repository_impl = Arc::new(OperatorRepositoryImpl::new(connection.clone()));
    let session_repository_impl = Arc::new(SessionRepositoryImpl::new(connection));
    let password_hasher = Arc::new(Argon2PasswordHasher::new());

    let auth_service = Arc::new(AuthService::new(
        operator_repository_impl.clone(),
        session_repository_impl.clone(),
        password_hasher.clone(),
    ));
    
    let c2_manager_handle = C2ManagerHandle::new();
    let listner_adapter = Arc::new(Mutex::new(ListenerAdapter::new(c2_manager_handle.clone())));
    let agent_adapter = Arc::new(Mutex::new(AgentAdapter::new(c2_manager_handle.clone())));

    let listener_service = Arc::new(ListenerService::new(listner_adapter));
    let operator_service = Arc::new(OperatorService::new(operator_repository_impl));
    let agent_service = Arc::new(AgentService::new(agent_adapter));


    let app_state = AppState::new(auth_service, listener_service, operator_service, agent_service);
    run(app_state).await
}

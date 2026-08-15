use std::sync::Arc;

use infrastructure::c2_manager::C2ManagerImpl;
use infrastructure::repository::operator_repository_impl::OperatorRepositoryImpl;
use infrastructure::repository::session_repository_impl::SessionRepositoryImpl;
use sqlx::PgPool;

use tokio::sync::Mutex;
use usecase::auth_usecase::AuthUsecase;
use usecase::listener_usecase::ListenerUsecase;

#[derive(Clone)]
pub struct AppState {
    pub db_connection: PgPool,
    pub auth_usecase: AuthUsecase,
    pub listener_usecase: ListenerUsecase,
}

impl AppState {
    pub fn new(connection: PgPool) -> Self {
        let listener_manager = Arc::new(Mutex::new(C2ManagerImpl::new()));

        Self {
            db_connection: connection.clone(), // repository間でトランザクション貼りたいときように渡す
            auth_usecase: AuthUsecase::new(
                Arc::new(OperatorRepositoryImpl::new(connection.clone())),
                Arc::new(SessionRepositoryImpl::new(connection.clone())),
            ),
            listener_usecase: ListenerUsecase::new(
                listener_manager,
            ),
        }
    }
}

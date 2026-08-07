use std::sync::Arc;

use infrastructure::command::{MpscCommandReceiver, MpscCommandSender};
use infrastructure::listener::ListenerManagerImpl;
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
        let (shutdown_tx, _) = tokio::sync::mpsc::unbounded_channel();
        let listener_manager = Arc::new(Mutex::new(ListenerManagerImpl::new(shutdown_tx)));

        let (command_tx, command_rx) = tokio::sync::mpsc::unbounded_channel();
        let command_sender = Arc::new(MpscCommandSender { tx: command_tx });
        let command_receiver = Arc::new(MpscCommandReceiver { rx: command_rx });

        Self {
            db_connection: connection.clone(), // repository間でトランザクション貼りたいときように渡す
            auth_usecase: AuthUsecase::new(
                Arc::new(OperatorRepositoryImpl::new(connection.clone())),
                Arc::new(SessionRepositoryImpl::new(connection.clone())),
            ),
            listener_usecase: ListenerUsecase::new(
                command_sender,
                command_receiver,
                listener_manager,
            ),
        }
    }
}

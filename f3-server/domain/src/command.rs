use tokio::sync::mpsc::error::SendError;

pub enum Command {
    OS(String),
}

#[async_trait::async_trait]
pub trait CommandSender: Send + Sync {
    async fn send(&self, command: Command) -> Result<(), SendError<Command>>;
}

#[async_trait::async_trait]
pub trait CommandReceiver: Send + Sync {
    async fn receive(&mut self) -> Option<Command>;
}

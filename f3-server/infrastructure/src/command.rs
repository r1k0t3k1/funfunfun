use domain::command::{Command, CommandReceiver, CommandSender};
use tokio::sync::mpsc::{Receiver, UnboundedReceiver, UnboundedSender, error::SendError};

pub struct MpscCommandSender {
    pub tx: UnboundedSender<Command>,
}

#[async_trait::async_trait]
impl CommandSender for MpscCommandSender {
    async fn send(&self, command: Command) -> Result<(), SendError<Command>> {
        self.tx.send(command)
    }
}

pub struct MpscCommandReceiver {
    pub rx: UnboundedReceiver<Command>,
}

#[async_trait::async_trait]
impl CommandReceiver for MpscCommandReceiver {
    async fn receive(&mut self) -> Option<Command> {
        self.rx.recv().await
    }
}

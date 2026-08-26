use tokio::sync::mpsc;

use crate::c2_inner_message::C2InnerMessage;

pub struct AgentActor {
    receiver: mpsc::UnboundedReceiver<C2InnerMessage>,
    task_queue: Vec<String>, // TODO
}

pub struct AgentHandle {
    sender: mpsc::UnboundedSender<C2InnerMessage>,
}

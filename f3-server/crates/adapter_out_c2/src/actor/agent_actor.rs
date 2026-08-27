use tokio::sync::mpsc;

use crate::c2_message::AgentMessage;

pub struct AgentActor {
    receiver: mpsc::UnboundedReceiver<AgentMessage>,
    task_queue: Vec<String>, // TODO
}

pub struct AgentHandle {
    sender: mpsc::UnboundedSender<AgentMessage>,
}

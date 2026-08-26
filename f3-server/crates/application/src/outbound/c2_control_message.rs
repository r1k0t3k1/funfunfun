use std::net::SocketAddr;

use uuid::Uuid;

use crate::domain::model::listener_model::ListenerProtocol;

pub enum C2ControllMessage {
    AddListener {
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol,
    },
    StartListener {
        listener_id: Uuid,
    },
    StopListener {
        listener_id: Uuid,
    },
    RemoveListener {
        listener_id: Uuid,
    },
}

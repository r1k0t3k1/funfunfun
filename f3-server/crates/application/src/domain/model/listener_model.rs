use std::net::SocketAddr;

use uuid::Uuid;


pub type ListenerId = Uuid;

#[derive(Clone)]
pub enum ListenerProtocol {
    Tcp,
    Http,
    Https,
}

impl std::fmt::Display for ListenerProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenerProtocol::Tcp => f.write_str("TCP"),
            ListenerProtocol::Http => f.write_str("HTTP"),
            ListenerProtocol::Https => f.write_str("HTTPS"),
        }
    }
}

#[derive(Clone)]
pub struct ListenerModel {
    pub id: ListenerId,
    pub name: String,
    pub addr: SocketAddr,
    pub protocol: ListenerProtocol
}

impl ListenerModel {
    pub fn new(
        id: ListenerId,
        name: String,
        addr: SocketAddr,
        protocol: ListenerProtocol
    ) -> Self {
        Self { id, name, addr, protocol }
    }
}

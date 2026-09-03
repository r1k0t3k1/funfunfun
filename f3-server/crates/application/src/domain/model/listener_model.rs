use std::net::SocketAddr;

use crate::domain::model::id::ListenerId;

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

impl TryFrom<String> for ListenerProtocol {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "TCP" => Ok(Self::Tcp),
            "HTTP" => Ok(Self::Http),
            "HTTPS" => Ok(Self::Https),
            _ => Err(anyhow::anyhow!("Invalid protocol")),
        }
    }
}

#[derive(Clone)]
pub struct ListenerModel {
    pub id: ListenerId,
    pub name: String,
    pub addr: SocketAddr,
    pub protocol: ListenerProtocol,
}

impl ListenerModel {
    pub fn new(id: ListenerId, name: String, addr: SocketAddr, protocol: ListenerProtocol) -> Self {
        Self {
            id,
            name,
            addr,
            protocol,
        }
    }
}

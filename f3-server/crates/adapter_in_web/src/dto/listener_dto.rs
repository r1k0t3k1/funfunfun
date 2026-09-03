use application::domain::model::listener_model::ListenerModel;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateListenerRequest {
    pub name: String,
    pub lhost: String,
    pub lport: u16,
    pub protocol: ListenerType,
}

#[derive(Deserialize, ToSchema)]
pub struct StartListenerRequest {
    pub listener_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct StopListenerRequest {
    pub listener_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RemoveListenerRequest {
    pub listener_id: String,
}

#[derive(Deserialize, ToSchema)]
pub enum ListenerType {
    TCP,
    HTTP,
    HTTPS,
}

impl std::fmt::Display for ListenerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenerType::TCP => f.write_str("TCP"),
            ListenerType::HTTP => f.write_str("HTTP"),
            ListenerType::HTTPS => f.write_str("HTTPS"),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct ListenerResponse {
    pub id: String,
    pub name: String,
    pub addr: String,
    pub protocol: String,
}

impl From<ListenerModel> for ListenerResponse {
    fn from(value: ListenerModel) -> Self {
        Self { 
            id: value.id.to_string(),
            name: value.name,
            addr: value.addr.to_string(),
            protocol: value.protocol.to_string(),
        }
    }
}

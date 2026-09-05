use application::domain::model::listener_model::{ListenerConfig, ListenerModel};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateListenerRequest {
    pub name: String,
    pub lhost: String,
    pub lport: u16,
    pub config: ListenerConfigRequest,
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

#[derive(Deserialize, ToSchema, Clone)]
#[serde(tag = "protocol")]
pub enum ListenerConfigRequest {
    Http {
        path: String,
        user_agent: String,
        host_header: String,
        http_method: String,
        is_ssl: bool,
    },
    Tcp {}, // TODO
    Dns {}, // TODO
}

impl Into<ListenerConfig> for ListenerConfigRequest {
    fn into(self) -> ListenerConfig {
        match self {
            ListenerConfigRequest::Http { path, user_agent, host_header, http_method, is_ssl } => {
                ListenerConfig::Http { path, user_agent, host_header, http_method, is_ssl }
            },
            ListenerConfigRequest::Tcp {  } => ListenerConfig::Tcp {  },
            ListenerConfigRequest::Dns {  } => ListenerConfig::Dns {  },
        }
    }
}


#[derive(Serialize, ToSchema)]
pub struct ListenerResponse {
    pub id: String,
    pub name: String,
    pub lhost: String,
    pub lport: u16,
    pub config: ListenerConfigResponse,  
}

#[derive(Serialize, ToSchema)]
#[serde(tag = "protocol")]
pub enum ListenerConfigResponse {
    Http {
        path: String,
        user_agent: String,
        host_header: String,
        http_method: String,
        is_ssl: bool,
    },
    Tcp {}, // TODO
    Dns {}, // TODO
}

impl From<ListenerConfig> for ListenerConfigResponse {
    fn from(value: ListenerConfig) -> Self {
        match value {
            ListenerConfig::Http { path, user_agent, host_header, http_method, is_ssl } => {
                Self::Http { path, user_agent, host_header, http_method, is_ssl }
            },
            ListenerConfig::Tcp {  } => Self::Tcp {  },
            ListenerConfig::Dns {  } => Self::Dns {  },
        }
    }
}

impl From<ListenerModel> for ListenerResponse {
    fn from(value: ListenerModel) -> Self {
        Self { 
            id: value.id.to_string(),
            name: value.name,
            lhost: value.lhost,
            lport: value.lport,
            config: value.config.into(),
        }
    }
}

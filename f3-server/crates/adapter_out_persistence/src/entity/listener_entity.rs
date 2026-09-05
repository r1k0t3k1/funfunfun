use application::{domain::model::listener_model::{ListenerConfig, ListenerModel}, outbound::error::RepositoryError};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::{Uuid, chrono}};

#[derive(Clone, Debug, FromRow)]
pub struct ListenerEntity {
    pub id: Uuid,
    pub name: String,
    pub lhost: String,
    pub lport: i32,           // Domainモデルに詰め替えるときにu16に変換
    pub is_running: bool,
    pub checkin_key: Vec<u8>, // Domainモデルに詰め替えるときに[u8;32]に変換
    pub config: String,  // RepositoryでStringからListenerConfigEntityへ変換
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "protocol", rename_all = "snake_case")]
pub enum ListenerConfigEntity {
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

impl Into<ListenerConfig> for ListenerConfigEntity {
    fn into(self) -> ListenerConfig {
        match self {
            ListenerConfigEntity::Http { path, user_agent, host_header, http_method, is_ssl } => {
                ListenerConfig::Http { path, user_agent, host_header, http_method, is_ssl }
            },
            ListenerConfigEntity::Tcp {  } => { ListenerConfig::Tcp {  } },
            ListenerConfigEntity::Dns {  } => { ListenerConfig::Dns {  } },
        }
    }
}

impl From<ListenerConfig> for ListenerConfigEntity {
    fn from(value: ListenerConfig) -> Self {
        match value {
            ListenerConfig::Http { path, user_agent, host_header, http_method, is_ssl } => {
                ListenerConfigEntity::Http { path, user_agent, host_header, http_method, is_ssl }
            },
            ListenerConfig::Tcp {  } => ListenerConfigEntity::Tcp {  },
            ListenerConfig::Dns {  } => ListenerConfigEntity::Dns {  },
        }
    }
}

impl TryInto<ListenerModel> for ListenerEntity {
    type Error = RepositoryError;

    fn try_into(self) -> Result<ListenerModel, Self::Error> {
        let config: ListenerConfigEntity = serde_json::from_str(&self.config)
            .map_err(|e| RepositoryError::FailedToDesirialize { detail: e.to_string() })?;
       
        let checkin_key: [u8; 32] = self.checkin_key.try_into()
            .map_err(|v: Vec<u8>| RepositoryError::InvalidKey{ detail: format!("got {}", v.len()) })?;

        Ok(ListenerModel {
            id: self.id.into(),
            name: self.name,
            lhost: self.lhost,
            lport: self.lport as u16,
            is_running: self.is_running,
            checkin_key,
            config: config.into(),
        })
    }
}

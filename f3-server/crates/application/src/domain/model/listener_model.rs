use crate::domain::model::id::ListenerId;

// プロトコル固有の設定はListenerConfigに隠蔽
#[derive(Clone)]
pub struct ListenerModel {
    pub id: ListenerId,
    pub name: String,
    pub lhost: String,
    pub lport: u16,
    pub is_running: bool,
    pub checkin_key: [u8; 32], // Domainモデルに詰め替えるときに[u8;32]に変換
    pub config: ListenerConfig,
}

#[derive(Clone)]
pub enum ListenerConfig {
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

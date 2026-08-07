use std::time::SystemTime;

#[derive(Clone)]
pub struct Session {
    pub session_id: String,
    pub operator_id: String,
    pub expire_at: SystemTime,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        if SystemTime::now() > self.expire_at {
            return true;
        }
        false
    }
}

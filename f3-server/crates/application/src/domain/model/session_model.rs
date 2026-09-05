use std::time::SystemTime;

use crate::domain::model::id::{OperatorId, SessionId};

#[derive(Clone)]
pub struct SessionModel {
    pub id: SessionId,
    pub operator_id: OperatorId,
    pub expire_at: SystemTime,
}

impl SessionModel {
    pub fn is_expired(&self) -> bool {
        if SystemTime::now() > self.expire_at {
            return true;
        }
        false
    }
}

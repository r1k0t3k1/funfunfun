use application::domain::model::session_model::Session;
use sqlx::{postgres::types::PgHstore, types::chrono};

#[derive(Clone)]
pub struct SessionEntity {
    pub session_id: String,
    pub operator_id: String,
    pub expire_at: chrono::DateTime<chrono::Utc>,
    pub attribute: Option<PgHstore>,
}

impl Into<Session> for SessionEntity {
    fn into(self) -> Session {
        Session {
            session_id: self.session_id,
            operator_id: self.operator_id,
            expire_at: self.expire_at.into(),
        }
    }
}

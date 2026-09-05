use application::domain::model::session_model::SessionModel;
use sqlx::{postgres::types::PgHstore, types::{Uuid, chrono}};

#[derive(Clone)]
pub struct SessionEntity {
    pub id: Uuid,
    pub operator_id: Uuid,
    pub expire_at: chrono::DateTime<chrono::Utc>,
    pub attribute: Option<PgHstore>,
}

impl Into<SessionModel> for SessionEntity {
    fn into(self) -> SessionModel {
        SessionModel {
            id: self.id.into(),
            operator_id: self.operator_id.into(),
            expire_at: self.expire_at.into(),
        }
    }
}

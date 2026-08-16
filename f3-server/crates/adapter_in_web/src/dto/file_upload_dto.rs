use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UploadFile {
    pub filename: String,
    pub data: String,
}

use actix_web::{HttpResponse, post, web};
use base64::{Engine, prelude::BASE64_STANDARD};
use std::{fs::File, io::Write, path::Path};

use crate::{dto::file_upload_dto::UploadFile, error::ApiError};

#[utoipa::path(
    context_path = "/after-login",
    responses(
        (status = 200, description = "アップロード成功"),
        (status = 400, description = "アップロード失敗")
    )
)]
#[post("/upload")]
pub async fn upload_file(upload_file: web::Json<UploadFile>) -> Result<HttpResponse, ApiError> {
    let filename = Path::new(&upload_file.filename)
        .file_name()
        //.ok_or_else(|| ApiError::BadRequest("filename".to_string()))?;
        .ok_or_else(|| ApiError::BadRequest)?;

    let file = BASE64_STANDARD
        .decode(upload_file.data.clone())
        //.map_err(|e| ApiError::Validation(e.to_string()))?;
        .map_err(|e| ApiError::BadRequest)?;

    let upload_path = Path::new("resource/download").join(filename);

    let mut new_file = File::create(upload_path)
        //.map_err(|e| ApiError::Unexpected(e.into()))?;
        .map_err(|e| ApiError::InternelServerError)?;

    new_file
        .write_all(&file)
        //.map_err(|e| ApiError::InternelServerError(e.into()))?;
        .map_err(|e| ApiError::InternelServerError)?;

    Ok(HttpResponse::Ok().finish())
}

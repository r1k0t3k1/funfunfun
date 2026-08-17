use crate::dto::auth_dto::AuthenticatedResponse;
use crate::state::AppState;
use crate::{dto::operator_dto::OperatorCredential, error::ApiError};
use actix_web::{HttpRequest, HttpResponse, post, web};

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200, description = "ログイン成功"),
        (status = 401, description = "認証情報が正しくない")
    )
)]
#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    cred: web::Json<OperatorCredential>,
) -> Result<HttpResponse, ApiError> {
    let session = state
        .auth_usecase
        .authenticate_operator(cred.username.clone(), cred.password.clone())
        .await?;

    let res = HttpResponse::Ok().json(AuthenticatedResponse {
        access_token: session.session_id,
    });

    Ok(res)
}

#[utoipa::path(
    context_path = "/auth",
    responses(
        (status = 200, description = "ログアウト成功（未認証状態で実行しても成功扱い）"),
    )
)]
#[post("/logout")]
pub async fn logout(
    state: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let session_id = req.cookie("session_id");

    if session_id.is_none() {
        return Ok(HttpResponse::Ok().finish());
    }

    state
        .auth_usecase
        .logout(session_id.unwrap().to_string())
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| e.into())
}

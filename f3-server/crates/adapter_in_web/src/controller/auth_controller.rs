use crate::dto::auth_dto::{AuthenticateRequest, AuthenticatedResponse};
use crate::response::ResponseBody;
use crate::state::AppState;
use crate::error::ApiError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, Responder, post, web};
use application::domain::model::id::SessionId;
use uuid::Uuid;

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
    cred: web::Json<AuthenticateRequest>,
) -> Result<impl Responder, ApiError> {
    let session = state
        .auth_usecase
        .authenticate_operator(&cred.operator_name, &cred.password)
        .await?;

    Ok(ResponseBody::ok(StatusCode::OK, AuthenticatedResponse {
        access_token: session.id.to_string(),
    }))
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
) -> Result<impl Responder, ApiError> {
    let Some(session_id) = req.cookie("session_id") else {
        return Ok(ResponseBody::ok(StatusCode::OK, ())); // セッションなくてもとりあえずOK返す
    };

    // パース失敗してもとりあえずOK返す
    let Ok(uuid) = Uuid::try_parse(&session_id.value()) else {
        return Ok(ResponseBody::ok(StatusCode::OK, ()));
    };

    let session_id = SessionId::from(uuid);

    // ログアウト失敗してもとりあえずOK返す
    let _ = state
        .auth_usecase
        .logout(session_id)
        .await;

    Ok(ResponseBody::ok(StatusCode::OK, ()))
}

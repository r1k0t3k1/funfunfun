use crate::dto::operator_dto::OperatorCredential;
use crate::{error::AppError, state::AppState};
use actix_web::{
    HttpRequest, HttpResponse,
    cookie::{CookieBuilder, SameSite},
    post, web,
};

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
) -> Result<HttpResponse, AppError> {
    let session = state
        .auth_usecase
        .authenticate_operator(cred.username.clone(), cred.password.clone())
        .await?;

    let res = HttpResponse::Ok()
        .cookie(
            CookieBuilder::new("sessionid", session.session_id)
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(SameSite::None)
                .finish(),
        )
        .finish();

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
) -> Result<HttpResponse, AppError> {
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

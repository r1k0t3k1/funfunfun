use actix_web::{Responder, get, http::StatusCode, post, web};
use uuid::Uuid;

use crate::{
    dto::listener_dto::{
        CreateListenerRequest, ListenerResponse, RemoveListenerRequest, StartListenerRequest, StopListenerRequest
    }, error::ApiError, response::ResponseBody, state::AppState
};

#[utoipa::path(
    context_path = "/listener",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = Vec<ListenerResponse>, description = "Listenerの一覧"),
        (status = 500, description = "ハンドリングできない異常")
    )
)]
#[get("/list")]
pub async fn list_listeners(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let response_json = state
        .listener_usecase
        .list_listeners()
        .await
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::NotFound
        })?
        .iter()
        .map(|l| l.clone().into())
        .collect::<Vec<ListenerResponse>>();

    Ok(ResponseBody::ok(StatusCode::OK, response_json))
}

#[utoipa::path(
    context_path = "/listener",
    security(
        ("bearer_auth" = ["Admin", "Write"])
    ),
    responses(
        (status = 200, description = "Listenerの作成に成功"),
        (status = 500, description = "Listenerの作成に失敗")
    )
)]
#[post("/create")]
pub async fn create_listener(
    state: web::Data<AppState>,
    listener_data: web::Json<CreateListenerRequest>,
) -> Result<impl Responder, ApiError> {
    state
        .listener_usecase
        .create_listener(
            listener_data.name.to_string(),
            listener_data.lhost.to_string(),
            listener_data.lport,
            listener_data.config.clone().into(),
        )
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, ListenerResponse::from(result)))
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::InternelServerError
        })
}

#[utoipa::path(
    context_path = "/listener",
    security(
        ("bearer_auth" = ["Admin", "Write"])
    ),
    responses(
        (status = 200, description = "Listenerが正常に起動"),
        (status = 500, description = "Listenerの起動に失敗")
    )
)]
#[post("/start")]
pub async fn start_listener(
    state: web::Data<AppState>,
    req: web::Json<StartListenerRequest>,
) -> Result<impl Responder, ApiError> {
    let listener_id = req
        .listener_id
        .parse::<Uuid>()
        .map_err(|_| ApiError::BadRequest {detail: "Failed to parse listener id".to_string()})
        .map(|id| id.into())?;

    state
        .listener_usecase
        .start_listener(listener_id)
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, result))
        .map_err(|e| ApiError::BadRequest {detail: format!("Failed to start listener. inner error: {e}")})
}

#[utoipa::path(
    context_path = "/listener",
    security(
        ("bearer_auth" = ["Admin", "Write"])
    ),
    responses(
        (status = 200, description = "Listenerが正常に停止"),
        (status = 500, description = "Listenerの停止に失敗")
    )
)]
#[post("/stop")]
pub async fn stop_listener(
    state: web::Data<AppState>,
    req: web::Json<StopListenerRequest>,
) -> Result<impl Responder, ApiError> {
    let listener_id = req
        .listener_id
        .parse::<Uuid>()
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::BadRequest {detail: "Failed to parse listener id".to_string()}
        })
        .map(|id| id.into())?;

    state
        .listener_usecase
        .stop_listener(listener_id)
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, result))
        .map_err(|e| ApiError::BadRequest {detail: format!("Failed to stop listener. inner error: {e}")})
}

#[utoipa::path(
    context_path = "/listener",
    security(
        ("bearer_auth" = ["Admin", "Write"])
    ),
    responses(
        (status = 200, description = "Listenerの削除に成功"),
        (status = 500, description = "Listenerの削除に失敗")
    )
)]
#[post("/remove")]
pub async fn remove_listener(
    state: web::Data<AppState>,
    req: web::Json<RemoveListenerRequest>,
) -> Result<impl Responder, ApiError> {
    let listener_id = req
        .listener_id
        .parse::<Uuid>()
        .map_err(|_| ApiError::BadRequest {detail: "Failed to parse listener id".to_string()})
        .map(|id| id.into())?;

    state
        .listener_usecase
        .remove_listener(listener_id)
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, result))
        .map_err(|e| ApiError::BadRequest {detail: format!("Failed to remove listener. inner error: {e}")})
}

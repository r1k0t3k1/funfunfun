use actix_web::{HttpResponse, get, post, web};

use crate::{
    dto::listener_dto::{
        CreateListenerRequest, ListListenerResponse, RemoveListenerRequest, StartListenerRequest,
        StopListenerRequest,
    },
    error::AppError,
    state::AppState,
};

#[utoipa::path(
    context_path = "/listener",
    responses(
        (status = 200, description = "Listenerの一覧"),
        (status = 500, description = "ハンドリングできない異常")
    )
)]
#[get("/list")]
pub async fn list_listeners(state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let response_json = state
        .listener_usecase
        .list_listeners()
        .await
        .iter()
        .map(|l| ListListenerResponse {
            name: l.to_string(),
            addr: l.to_string(),
        })
        .collect::<Vec<ListListenerResponse>>();

    Ok(HttpResponse::Ok().json(response_json))
}

#[utoipa::path(
    context_path = "/listener",
    responses(
        (status = 200, description = "Listenerの作成に成功"),
        (status = 500, description = "Listenerの作成に失敗")
    )
)]
#[post("/create")]
pub async fn create_listener(
    state: web::Data<AppState>,
    listener_data: web::Json<CreateListenerRequest>,
) -> Result<HttpResponse, AppError> {
    state
        .listener_usecase
        .create_listener(
            listener_data.listener_type.to_string(),
            listener_data.lhost.clone(),
            listener_data.lport,
        )
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| AppError::UsecaseError(e.into()))
}

#[utoipa::path(
    context_path = "/listener",
    responses(
        (status = 200, description = "Listenerが正常に起動"),
        (status = 500, description = "Listenerの起動に失敗")
    )
)]
#[post("/start")]
pub async fn start_listener(
    state: web::Data<AppState>,
    req: web::Json<StartListenerRequest>,
) -> Result<HttpResponse, AppError> {
    let listener_id = req.listener_id.parse::<uuid::Uuid>()
        .map_err(|_| AppError::Validation("ListenerId".to_string()))?;
    state
        .listener_usecase
        .start_listener(listener_id)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| AppError::UsecaseError(e.into()))
}

#[utoipa::path(
    context_path = "/listener",
    responses(
        (status = 200, description = "Listenerが正常に停止"),
        (status = 500, description = "Listenerの停止に失敗")
    )
)]
#[post("/stop")]
pub async fn stop_listener(
    state: web::Data<AppState>,
    req: web::Json<StopListenerRequest>,
) -> Result<HttpResponse, AppError> {
    let listener_id = req.listener_id.parse::<uuid::Uuid>()
        .map_err(|_| AppError::Validation("ListenerId".to_string()))?;
    state
        .listener_usecase
        .stop_listener(listener_id)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| AppError::UsecaseError(e.into()))
}

#[utoipa::path(
    context_path = "/listener",
    responses(
        (status = 200, description = "Listenerの削除に成功"),
        (status = 500, description = "Listenerの削除に失敗")
    )
)]
#[post("/remove")]
pub async fn remove_listener(
    state: web::Data<AppState>,
    req: web::Json<RemoveListenerRequest>,
) -> Result<HttpResponse, AppError> {
    let listener_id = req.listener_id.parse::<uuid::Uuid>()
        .map_err(|_| AppError::Validation("ListenerId".to_string()))?;
    state
        .listener_usecase
        .remove_listener(listener_id)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| AppError::UsecaseError(e.into()))
}

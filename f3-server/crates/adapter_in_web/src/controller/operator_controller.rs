use crate::dto::operator_dto::{
    GetOperatorRequest, OperatorResponse, ToggleOperatorStatusRequest, UpdatePasswordRequest
};
use crate::error::ApiError;
use crate::response::ResponseBody;
use crate::state::AppState;
use actix_web::http::StatusCode;
use actix_web::{Responder, get, post, web};
use application::domain::model::id::OperatorId;
use application::domain::model::operator_model::OperatorModel;
use uuid::Uuid;

#[utoipa::path(
    context_path = "/operator",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "オペレータ一覧"),
    )
)]
#[get("/list")]
pub async fn list_operators(state: web::Data<AppState>) -> Result<impl Responder, ApiError> {
    let operators: Vec<OperatorResponse> = state
        .operator_usecase
        .list_operators()
        .await
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::InternelServerError
        })?
        .iter()
        .map(|o| Into::<OperatorResponse>::into(o.clone()))
        .collect();

    Ok(ResponseBody::ok(StatusCode::OK, operators))
}

#[utoipa::path(
    context_path = "/operator",
    security(
        ("bearer_auth" = [])
    ),
    params(GetOperatorRequest),
    responses(
        (status = 200, description = "オペレータ詳細情報"),
    )
)]
#[get("/get")]
pub async fn get_operator(
    state: web::Data<AppState>,
    operator: web::Query<GetOperatorRequest>,
) -> Result<impl Responder, ApiError> {
    let Ok(uuid) = Uuid::try_parse(&operator.operator_id) else {
        return Err(ApiError::BadRequest { detail: "Invalid operator id".to_string() });
    };

    let operator_id = OperatorId::from(uuid);

    let operator = state
        .operator_usecase
        .get_operator(operator_id)
        .await
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::InternelServerError
        })?
        .ok_or_else(|| ApiError::NotFound)
        .map(|o| Into::<OperatorResponse>::into(o))?;
    
    Ok(ResponseBody::ok(StatusCode::OK, operator))
}

#[utoipa::path(
    context_path = "/operator",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "パスワード変更完了"),
    )
)]
#[post("/update_password")]
pub async fn update_password(
    state: web::Data<AppState>,
    auth_operator: web::ReqData<OperatorModel>,
    request: web::Json<UpdatePasswordRequest>,
) -> Result<impl Responder, ApiError> {
    let operator_id = auth_operator.id.clone();
    let current_password = request.current_password.clone();
    let new_password = request.new_password.clone();

    state
        .operator_usecase
        .change_password(operator_id, current_password, new_password)
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, result))
        .map_err(|e| ApiError::BadRequest {detail: format!("Failed to update password. inner error: {e}")})
}

#[utoipa::path(
    context_path = "/operator",
    security(
        ("bearer_auth" = ["Admin"])
    ),
    responses(
        (status = 200, description = "有効化状態変更完了"),
    )
)]
#[post("/toggle_status")]
pub async fn toggle_operator_status(
    state: web::Data<AppState>,
    request: web::Json<ToggleOperatorStatusRequest>,
) -> Result<impl Responder, ApiError> {
    let Ok(uuid) = Uuid::try_parse(&request.operator_id) else {
        return Err(ApiError::BadRequest { detail: "Invalid operator id".to_string() });
    };

    let operator_id = OperatorId::from(uuid);

    state
        .operator_usecase
        .toggle_status(operator_id)
        .await
        .map(|result| ResponseBody::ok(StatusCode::OK, Into::<OperatorResponse>::into(result)))
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::InternelServerError
        })
}


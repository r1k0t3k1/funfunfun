use crate::dto::operator_dto::{
    AuthOperator, GetOperatorRequest, OperatorResponse, ToggleOperatorStatusRequest, UpdatePasswordRequest
};
use crate::error::ApiError;
use crate::state::AppState;
use actix_web::{HttpResponse, get, post, web};

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
pub async fn list_operators(state: web::Data<AppState>) -> Result<HttpResponse, ApiError> {
    let operators: Vec<OperatorResponse> = state
        .operator_usecase
        .list_operators()
        .await
        .map_err(|e| ApiError::BadRequest)?
        .iter()
        .map(|o| Into::<OperatorResponse>::into(o.clone()))
        .collect();

    Ok(HttpResponse::Ok().json(operators))
}

#[utoipa::path(
    context_path = "/operator",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "オペレータ詳細情報"),
    )
)]
#[get("/get")]
pub async fn get_operator(
    state: web::Data<AppState>,
    operator: web::Json<GetOperatorRequest>,
) -> Result<HttpResponse, ApiError> {
    let operator = state
        .operator_usecase
        .get_operator(operator.operator_id.clone())
        .await
        .map_err(|e| ApiError::BadRequest)?
        .ok_or_else(|| ApiError::InternelServerError)
        .map(|o| Into::<OperatorResponse>::into(o))?;

    Ok(HttpResponse::Ok().json(operator))
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
    auth_operator: web::ReqData<AuthOperator>,
    request: web::Json<UpdatePasswordRequest>,
) -> Result<HttpResponse, ApiError> {
    let operator_id = auth_operator.operator_id.clone();
    let current_password = request.current_password.clone();
    let new_password = request.new_password.clone();

    let res = state
        .operator_usecase
        .change_password(operator_id, current_password, new_password)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e| ApiError::BadRequest)?;

    Ok(res)
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
) -> Result<HttpResponse, ApiError> {
    let operator_id = request.operator_id.clone();

    let res = state
        .operator_usecase
        .toggle_status(operator_id)
        .await
        .map(|_| HttpResponse::Ok().finish())
        .map_err(|e|  { 
            log::error!("{e}");
            return ApiError::BadRequest;
        })?;

    Ok(res)
}


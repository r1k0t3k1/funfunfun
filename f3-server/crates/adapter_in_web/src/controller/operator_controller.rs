use crate::dto::auth_dto::AuthenticatedResponse;
use crate::dto::operator_dto::{GetOperatorRequest, OperatorResponse};
use crate::state::AppState;
use crate::error::ApiError;
use actix_web::{HttpResponse, post, web};

#[utoipa::path(
    context_path = "/operator",
    responses(
        (status = 200, description = "オペレータ一覧"),
    )
)]
#[post("/list")]
pub async fn list_operators(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let operators: Vec<OperatorResponse> = state.operator_usecase.list_operators()
        .await
        .map_err(|e| ApiError::BadRequest)?
        .iter()
        .map(|o| Into::<OperatorResponse>::into(o.clone()))
        .collect();

    Ok(HttpResponse::Ok().json(operators))
}

#[utoipa::path(
    context_path = "/operator",
    responses(
        (status = 200, description = "オペレータ詳細情報"),
    )
)]
#[post("/get")]
pub async fn get_operator(
    state: web::Data<AppState>,
    operator: web::Json<GetOperatorRequest>,
) -> Result<HttpResponse, ApiError> {
    let operator = state.operator_usecase.get_operator(operator.operator_id.clone())
        .await
        .map_err(|e| ApiError::BadRequest)?
        .ok_or_else(|| ApiError::InternelServerError)
        .map(|o| Into::<OperatorResponse>::into(o))?;

    Ok(HttpResponse::Ok().json(operator))
}

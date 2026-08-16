use actix_web::{HttpResponse, Responder, get};

use crate::dto::operator_dto::AuthOperator;

#[utoipa::path(
    context_path = "/required-role-admin",
    responses(
        (status = 200, description = "テスト用"),
    )
)]
#[get("/admin")]
pub async fn admin(operator: AuthOperator) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}!\npermission: {:?}",
        operator.name, operator.role
    ))
}

#[utoipa::path(
    context_path = "/required-role-write",
    responses(
        (status = 200, description = "テスト用"),
    )
)]
#[get("/write")]
pub async fn write(operator: AuthOperator) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}!\npermission: {:?}",
        operator.name, operator.role
    ))
}

#[utoipa::path(
    context_path = "/required-role-read",
    responses(
        (status = 200, description = "テスト用"),
    )
)]
#[get("/write")]
pub async fn read(operator: AuthOperator) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Hello {}!\npermission: {:?}",
        operator.name, operator.role
    ))
}

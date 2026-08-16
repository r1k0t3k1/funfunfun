use actix_web::{HttpResponse, Responder, get};

use crate::dto::operator_dto::AuthOperator;

#[utoipa::path(
    context_path = "/after-login",
    responses(
        (status = 200, description = "テスト用"),
    )
)]
#[get("/top")]
pub async fn after_login(operator: AuthOperator) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", operator.name))
}

use actix_web::{HttpResponse, Responder, get, web};

use crate::state::AppState;

#[utoipa::path(
    context_path = "/health",
    responses(
        (status = 200, description = "DBが正常に起動している"),
        (status = 500, description = "DBが起動していない、またはクエリの実行に失敗した")
    )
)]
#[get("/db")]
pub async fn health_check_db(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok() // TODO
}

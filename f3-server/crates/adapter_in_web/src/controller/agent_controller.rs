use actix_web::{Responder, get, http::StatusCode, web};
use application::domain::model::id::{AgentId, ListenerId};
use uuid::Uuid;

use crate::{
    dto::agent_dto::{AgentResponse, GetAgentRequest, ListAgentRequest}, error::ApiError, response::ResponseBody, state::AppState
};


#[utoipa::path(
    context_path = "/agent",
    security(
        ("bearer_auth" = [])
    ),
    params(ListAgentRequest),
    responses(
        (status = 200, body = Vec<AgentResponse>, description = "Listenerが管理するAgentの一覧"),
        (status = 500, description = "ハンドリングできない異常")
    )
)]
#[get("/list")]
pub async fn list_agents(state: web::Data<AppState>, agent_request: web::Query<ListAgentRequest>) -> Result<impl Responder, ApiError> {
    let listener_id: ListenerId = agent_request.listener_id.parse::<Uuid>()
        .map_err(|e| ApiError::BadRequest { detail: format!("Invalid Listener id: {e}") })
        .map(|id| id.into())?;

    let response_json: Vec<AgentResponse> = state
        .agent_usecase
        .list_agents(listener_id.clone())
        .await
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::NotFound
        })?
        .iter()
        .map(|a| Into::<AgentResponse>::into(a.clone()))
        .collect();

    Ok(ResponseBody::ok(StatusCode::OK, response_json))
}

#[utoipa::path(
    context_path = "/agent",
    security(
        ("bearer_auth" = [])
    ),
    params(GetAgentRequest),
    responses(
        (status = 200, body = AgentResponse, description = "Agent"),
        (status = 500, description = "ハンドリングできない異常")
    )
)]
#[get("/get")]
pub async fn get_agent(state: web::Data<AppState>, agent_request: web::Query<GetAgentRequest>) -> Result<impl Responder, ApiError> {
    let agent_id = agent_request.agent_id.parse::<Uuid>()
        .map_err(|e| ApiError::BadRequest { detail: format!("Invalid Agent id: {e}") })
        .map(|id| id.into())?;

    let response_json = state
        .agent_usecase
        .get_agent(agent_id)
        .await
        .map(|a| Into::<AgentResponse>::into(a.clone()))
        .map_err(|e| {
            log::warn!("{e}");
            ApiError::NotFound
        })?;

    Ok(ResponseBody::ok(StatusCode::OK, response_json))
}

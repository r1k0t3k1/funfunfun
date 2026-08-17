use std::{
    future::{Ready, ready},
    sync::Arc,
};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::LocalBoxFuture;

use crate::{dto::operator_dto::AuthOperator, error::ApiError, state::AppState};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthN;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthN
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthNMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthNMiddleware {
            service: Arc::new(service),
        }))
    }
}

pub struct AuthNMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthNMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        //let fut = self.service.call(req);
        let service = Arc::clone(&self.service);

        Box::pin(async move {
            let authorization_header = req
                .headers()
                .get("Authorization")
                .ok_or_else(|| ApiError::Unauthorized)?;

            let bearer_token = authorization_header.to_str()
                .map_err(|_| ApiError::Unauthorized)?;
            
            let re = regex::Regex::new(r"\s*Bearer\s*").unwrap();
            let session_token = re.replace_all(bearer_token, "").to_string();
            
            if session_token.len() != 64 {
                return Err(ApiError::Unauthorized.into());
            }

            let app_state = req
                .app_data::<web::Data<AppState>>()
                .clone()
                .ok_or_else(|| ApiError::InternelServerError)?;
                //.ok_or_else(|| ApiError::InternelServerError(anyhow::anyhow!("Usecase not registered")))?;

            let is_valid_session = app_state
                .auth_usecase
                .is_valid_session(session_token.clone())
                .await
                .map_err(|_| ApiError::Unauthorized)?;

            if !is_valid_session {
                return Err(ApiError::Unauthorized.into());
            }

            let operator = app_state
                .auth_usecase
                .get_operator_from_session(session_token)
                .await
                .map_err(|e| ApiError::InternelServerError)?
                //.map_err(|e| ApiError::UsecaseError(e))?
                .ok_or_else(|| ApiError::Unauthorized)?;

            req.extensions_mut().insert(AuthOperator::from(operator));
            service.call(req).await
        })
    }
}

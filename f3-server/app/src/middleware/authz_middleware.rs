use std::{
    future::{Ready, ready},
    sync::Arc,
};

use actix_web::{
    Error, HttpMessage,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures_util::future::LocalBoxFuture;

use crate::{
    dto::{operator_dto::AuthOperator, role_dto::Role},
    error::AppError,
};

#[derive(Clone)]
pub enum RoleRequirement {
    Is(Role),
    Any(Vec<RoleRequirement>),
    All(Vec<RoleRequirement>),
    Not(Box<RoleRequirement>),
}

impl RoleRequirement {
    pub fn check_permission(&self, operator: &AuthOperator) -> bool {
        match self {
            Self::Is(r) => operator.has_role(r),
            Self::Not(rs) => !rs.check_permission(operator),
            Self::Any(rs) => rs.iter().any(|r| r.check_permission(operator)),
            Self::All(rs) => rs.iter().all(|r| r.check_permission(operator)),
        }
    }
}

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthZ {
    role_required: RoleRequirement,
}

impl AuthZ {
    pub fn new(role_required: RoleRequirement) -> Self {
        Self { role_required }
    }
}

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthZ
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthZMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthZMiddleware {
            service: Arc::new(service),
            role_required: self.role_required.clone(),
        }))
    }
}

pub struct AuthZMiddleware<S> {
    service: Arc<S>,
    role_required: RoleRequirement,
}

impl<S, B> Service<ServiceRequest> for AuthZMiddleware<S>
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
        let service = Arc::clone(&self.service);
        let required_role = self.role_required.clone();

        Box::pin(async move {
            log::warn!("TEST");
            let operator = req
                .extensions()
                .get::<AuthOperator>()
                .ok_or_else(|| AppError::Unauthorized)?
                .clone();
            let has_permission = required_role.check_permission(&operator);

            if !has_permission {
                return Err(AppError::Forbidden.into());
            }

            service.call(req).await
        })
    }
}

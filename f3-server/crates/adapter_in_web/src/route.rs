use crate::{
    apidocs::swagger_ui, controller::{
        auth_controller::{login, logout},
        file_upload_controller::upload_file,
        health_controller::health_check_db,
        listener_controller::{
            create_listener, list_listeners, remove_listener, start_listener, stop_listener,
        },
        operator_controller::{get_operator, list_operators, toggle_operator_status},
    }, dto::role_dto::Role, middleware::{authn_middleware::AuthN, authz_middleware::{AuthZ, RoleRequirement}}, state::AppState
};
use actix_files::Files;
use actix_web::{Error, dev::ServiceRequest, web};
use actix_web_httpauth::{
    extractors::{
        AuthenticationError,
        basic::{self, BasicAuth},
    },
    middleware::HttpAuthentication,
};

pub fn configure_route(state: web::Data<AppState>) -> impl FnOnce(&mut web::ServiceConfig) {
    move |config: &mut web::ServiceConfig| {
        config
            .app_data(state)
            .service(web::scope("/health").service(health_check_db))
            .service(web::scope("/auth").service(login).service(logout))
            .service(web::scope("/after-login").wrap(AuthN).service(upload_file))
            .service(
                web::scope("/listener")
                    .wrap(AuthN)
                    .service(list_listeners)
                    .service(create_listener)
                    .service(start_listener)
                    .service(stop_listener)
                    .service(remove_listener),
            )
            .service(swagger_ui())
            .service(
                web::scope("/download")
                    .wrap(HttpAuthentication::basic(validator))
                    .service(
                        Files::new("/", "resource/download")
                            .show_files_listing()
                            .use_hidden_files(),
                    ),
            )
            .service(
                web::scope("/operator")
                    .wrap(AuthN)
                    //.wrap(AuthZ::new(RoleRequirement::Is(Role::Admin)))
                    .service(list_operators)
                    .service(get_operator)
                    .service(toggle_operator_status)
            );
    }
}

async fn validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    if credentials.user_id() == "f3" && credentials.password() == Some("funfunfun") {
        return Ok(req);
    }

    Err((
        AuthenticationError::from(basic::Config::default()).into(),
        req,
    ))
}

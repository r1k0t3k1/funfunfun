use crate::{
    apidocs::swagger_ui,
    dto::role_dto::Role::{Admin, Read, Write},
    handler::{
        after_login_handler::after_login,
        auth_handler::{login, logout},
        file_upload_handler::upload_file,
        health_handler::health_check_db,
        listener_handler::{
            create_listener, list_listeners, remove_listener, start_listener, stop_listener,
        },
        required_role_handler::{admin, read, write},
    },
    middleware::{
        authn_middleware::AuthN,
        authz_middleware::{AuthZ, RoleRequirement},
    },
    state::AppState,
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
            .service(
                web::scope("/after-login")
                    .wrap(AuthN)
                    .service(after_login)
                    .service(upload_file),
            )
            .service(
                web::scope("/listener")
                    .wrap(AuthN)
                    .service(list_listeners)
                    .service(create_listener)
                    .service(start_listener)
                    .service(stop_listener)
                    .service(remove_listener),
            )
            .service(
                web::scope("/required-role-admin")
                    .wrap(AuthZ::new(RoleRequirement::Is(Admin)))
                    .wrap(AuthN)
                    .service(admin),
            )
            .service(
                web::scope("/required-role-write")
                    .wrap(AuthZ::new(RoleRequirement::Is(Write)))
                    .wrap(AuthN)
                    .service(write),
            )
            .service(
                web::scope("/required-role-read")
                    .wrap(AuthZ::new(RoleRequirement::Is(Read)))
                    .wrap(AuthN)
                    .service(read),
            )
            .service(swagger_ui())
            .service(
                web::scope("/download")
                    .wrap(HttpAuthentication::basic(validator))
                    .service(
                        Files::new("/", "./app/src/resource/download")
                            .show_files_listing()
                            .use_hidden_files(),
                    ),
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

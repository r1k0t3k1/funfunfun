use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::handler::health_handler::health_check_db,
    crate::handler::auth_handler::login,
    crate::handler::auth_handler::logout,
    crate::handler::after_login_handler::after_login,
    crate::handler::listener_handler::list_listeners,
    crate::handler::listener_handler::create_listener,
    crate::handler::listener_handler::start_listener,
    crate::handler::listener_handler::stop_listener,
    crate::handler::listener_handler::remove_listener,
    crate::handler::required_role_handler::admin,
    crate::handler::required_role_handler::write,
    crate::handler::required_role_handler::read,
    crate::handler::file_upload_handler::upload_file,
))]
struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

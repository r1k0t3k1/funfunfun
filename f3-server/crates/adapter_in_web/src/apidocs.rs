use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(
    crate::controller::health_controller::health_check_db,
    crate::controller::auth_controller::login,
    crate::controller::auth_controller::logout,
    crate::controller::listener_controller::list_listeners,
    crate::controller::listener_controller::create_listener,
    crate::controller::listener_controller::start_listener,
    crate::controller::listener_controller::stop_listener,
    crate::controller::listener_controller::remove_listener,
    crate::controller::file_upload_controller::upload_file,
))]
struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

#[test]
fn export_openapi_json() {
    let json = ApiDoc::openapi().to_pretty_json().unwrap();
    std::fs::write("openapi.json", json).unwrap();
}

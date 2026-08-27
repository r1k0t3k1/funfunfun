use utoipa::{Modify, OpenApi, openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme}};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::controller::health_controller::health_check_db,
        crate::controller::auth_controller::login,
        crate::controller::auth_controller::logout,
        crate::controller::listener_controller::list_listeners,
        crate::controller::listener_controller::create_listener,
        crate::controller::listener_controller::start_listener,
        crate::controller::listener_controller::stop_listener,
        crate::controller::listener_controller::remove_listener,
        crate::controller::file_upload_controller::upload_file,
        crate::controller::operator_controller::list_operators,
        crate::controller::operator_controller::get_operator,
        crate::controller::operator_controller::toggle_operator_status,
    ),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .build()
            ),
        );
    }
}

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi())
}

#[test]
fn export_openapi_json() {
    let json = ApiDoc::openapi().to_pretty_json().unwrap();
    std::fs::write("openapi.json", json).unwrap();
}

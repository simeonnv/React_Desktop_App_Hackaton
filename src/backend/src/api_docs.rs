use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Req {
    pub username: String,
    pub password: String,
}

struct BearerAuthAddon;

impl Modify for BearerAuthAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    components(schemas(Req)),
    security(("bearer_auth" = [])),
    paths(

    ),
    modifiers(&BearerAuthAddon),
    // tags(
    //     (name = "Auth", description = "Authentication endpoints"),
    //     (name = "Users", description = "User management endpoints")
    // ),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;
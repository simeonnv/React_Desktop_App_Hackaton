use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use crate::routes;

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::auth::signup::signup,
        routes::auth::login::login
    ),
    tags(
        (name = "Auth", description = "Authentication endpoints"),
        // (name = "Users", description = "User management endpoints")
    ),
   
)]
pub struct ApiDoc;
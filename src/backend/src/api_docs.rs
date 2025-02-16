use serde::{Deserialize, Serialize};
use utoipa::OpenApi;
use utoipa::{Modify, ToSchema};

use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

#[derive(OpenApi)]
#[openapi(
    paths(

    ),
    // tags(
    //     (name = "Auth", description = "Authentication endpoints"),
    //     (name = "Users", description = "User management endpoints")
    // ),
   
)]
pub struct ApiDoc;
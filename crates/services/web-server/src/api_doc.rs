use utoipa::OpenApi;

use crate::controllers::auth::{self as Auth};
use crate::error::HandlerError;

#[derive(OpenApi)]
#[openapi(
	paths(
		Auth::login,
		Auth::logout,
		Auth::register,
	),
	components(schemas(
        HandlerError
    )),
	tags(
        (name = "Auth", description = "The authentication API"),
        (name = "Account", description = "The account API"),
    )
)]
pub(crate) struct ApiV1Doc;

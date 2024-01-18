use utoipa::OpenApi;

use crate::controllers::auth::{self as Auth, SignInError};
use crate::error::HandlerError;

#[derive(OpenApi)]
#[openapi(
	paths(
		Auth::signin,
		Auth::signout,
		Auth::signup,
	),
	components(schemas(
        HandlerError<SignInError>
    )),
	tags(
        (name = "Auth", description = "The authentication API"),
        (name = "Account", description = "The account API"),
    )
)]
pub(crate) struct ApiV1Doc;

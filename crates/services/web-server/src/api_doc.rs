use utoipa::OpenApi;

use crate::controllers::auth as Auth;

#[derive(OpenApi)]
#[openapi(
	paths(
		Auth::signin,
		Auth::signout,
		Auth::signup,
	),
	components(schemas()),
	tags(
        (name = "Auth", description = "The authentication API"),
        (name = "Account", description = "The account API"),
    )
)]
pub(crate) struct ApiV1Doc;

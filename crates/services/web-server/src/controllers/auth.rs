use axum::{
	http::StatusCode,
	routing::{patch, post},
	Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

pub fn routes() -> Router {
	Router::new()
		//.route("/signin", post(signin))
		.route("/signout", patch(signout))
		.route("/signup", post(signup))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(crate) struct SignInResponse {
    access_token: String,
    refresh_token: String
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(crate) enum SignInError {
    InternalServerError,
    InvalidCredentials,
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct SignInQuery {
	email: String,
	password: String,
}


#[utoipa::path(
    post,
    path = "/api/v1/auth/signin",
    params(
        SignInQuery
    ),
    responses(
        (status = StatusCode::OK, description = "Successful operation", body = [String]),
        (status = StatusCode::UNAUTHORIZED, description = "", body = [SignupError]),
    ),
)]
pub(crate) async fn signin(Json(body): Json<SignInQuery>) -> std::result::Result<SignInResponse, ()> {
	Ok(SignInResponse {
        access_token: "test".to_string(),
        refresh_token: "test".to_string()
    })
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct SignOutQuery {
	refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/signout",
    params(
       SignOutQuery 
    ),
    responses(
        (status = StatusCode::OK, description = "Successful operation"),
    ),
)]
pub(crate) async fn signout(Json(body): Json<SignOutQuery>) -> StatusCode {
	StatusCode::OK
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct SignUpQuery {
	email: String,
	password: String,
	username: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/signup",
    params(
        SignUpQuery
    ),
    responses(
        (status = StatusCode::OK, description = "Successful operation")
    ),
)]
pub(crate) async fn signup(Json(body): Json<SignUpQuery>) -> StatusCode {
	StatusCode::OK
}

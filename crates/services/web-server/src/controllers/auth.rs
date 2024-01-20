use axum::{
	http::StatusCode,
	routing::post,
	Json, Router, response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{error::HandlerResult, AppState};


pub async fn routes() -> Router<AppState> {
	Router::new()
        .route("/signin", post(login))
		.route("/signout", post(logout))
		.route("/signup", post(register))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(crate) struct LoginResponse {
    access_token: String,
    refresh_token: String
}

#[derive(Serialize, Deserialize, IntoParams)]
pub(crate) struct LoginQuery {
	email: String,
	password: String,
}


#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    params(
        LoginQuery
    ),
    responses(
        (status = 200, description = "Successful operation", body = [String]),
        (status = UNAUTHORIZED, description = "Couldn't login", body = [HandlerError]),
    ),
)]
pub(crate) async fn login(Json(body): Json<LoginQuery>) -> HandlerResult<impl IntoResponse> {
    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token: "test".to_string(),
            refresh_token: "test".to_string()
        })
    ))
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct LogoutQuery {
	refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    params(
       LogoutQuery 
    ),
    responses(
        (status = StatusCode::OK, description = "Successful operation"),
    ),
)]
pub(crate) async fn logout(Json(body): Json<LogoutQuery>) -> StatusCode {
	StatusCode::OK
}

#[derive(Deserialize, IntoParams)]
pub(crate) struct RegisterQuery {
	email: String,
	password: String,
	username: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    params(
        RegisterQuery
    ),
    responses(
        (status = StatusCode::OK, description = "Successful operation")
    ),
)]
pub(crate) async fn register(Json(body): Json<RegisterQuery>) -> HandlerResult<StatusCode> {
	Ok(StatusCode::OK)
}
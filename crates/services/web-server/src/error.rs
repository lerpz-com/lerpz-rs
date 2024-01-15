use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A type alias for `Result<T, HandlerError>`.
/// Used by handlers to return a response or an error.
pub(crate) type HandlerResult<T, D = (), E = anyhow::Error> =
	std::result::Result<T, HandlerError<D, E>>;

/// The error response returned to the user.
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub(crate) struct HandlerError<D = (), E = anyhow::Error>
where
	D: Serialize,
	E: Into<anyhow::Error>,
{
	/// HTTP status code for the error.
	#[serde(skip)]
	pub(crate) status_code: StatusCode,
	/// Error header.
	pub(crate) header: String,
	/// Error message.
	pub(crate) message: String,
	/// Other details about the error.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) detail: Option<D>,
	/// The acctual error, if needed.
	/// Should never be sent to the client.
	#[serde(skip)]
	pub(crate) error: Option<E>,
	/// Log id of the error send to the client.
	/// Used to identify server errors in the logs.
	/// This is private and should never be set manually.
	#[serde(skip_serializing_if = "Option::is_none")]
	log_id: Option<uuid::Uuid>,
}

/// Converts a `HandlerError` into a response.
/// Automatically logs server errors.
impl IntoResponse for HandlerError {
	fn into_response(mut self) -> Response {
		if self.status_code.is_server_error() {
			self.log_id = uuid::Uuid::new_v4().into();
			if let Some(error) = self.error.as_ref() {
				tracing::error!("({:?}) - SERVER ERROR: {:?}", self.log_id, self.error);
			} else {
				tracing::error!("({:?}) - SERVER ERROR: {:?}", self.log_id, self.message);
			}
		}

		Json(self).into_response()
	}
}

/// Turns any error into a `HandlerError`.
impl<E> From<E> for HandlerError
where
	E: Into<anyhow::Error>,
{
	fn from(value: E) -> Self {
		Self {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			header: String::from("Internal Server Error"),
			message: String::from("If this issue persists, please contact the administrator."),
			detail: None,
			error: Some(value.into()),
			log_id: None, // This will be set in `into_response`.
		}
	}
}

impl<D> HandlerError<D>
where
	D: Serialize,
{
	// Error constructor.
	pub(crate) fn new(status_code: StatusCode, header: &str, message: &str) -> Self {
		Self {
			status_code,
			header: String::from(header),
			message: String::from(message),
			detail: None,
			error: None,
			log_id: None,
		}
	}

	/// Adds a custom detail to the error.
	pub(crate) fn with_detail(mut self, detail: D) -> Self {
		self.detail = Some(detail);
		self
	}

	/// Adds an inner error to the response error.
	pub(crate) fn with_error<E>(mut self, error: E) -> Self
	where
		E: Into<anyhow::Error>,
	{
		self.error = Some(error.into());
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn log_internal_server_error() {
		let error = HandlerError::from(anyhow::Error::msg("test"));
		// `log_id` should only be set when turned into a response.
		assert!(error.log_id.is_none());
		assert!(error.error.is_some());

		let response = error.into_response();
		assert!(response.status().is_server_error());
	}
}

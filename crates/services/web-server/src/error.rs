use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A type alias for [`Result<T, HandlerError>`].
///
/// Used by handlers to return a response or an structured error.
pub(crate) type HandlerResult<T, D = ()> = std::result::Result<T, HandlerError<D>>;

/// Represents an error returned by an handler.
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub(crate) struct HandlerError<D>
where
	D: Serialize + Send + Sync,
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
	/// The acctual error that occured.
	///
	/// There might no be an error, in which case
	/// this field is set to `None`.
	///
	/// Should never be sent to the client for
	/// security reason. This is why it is skipped.
	#[serde(skip)]
	pub(crate) error: Option<anyhow::Error>,
	/// ID of the error send to the client.
	///
	/// Used to identify server errors in the logs.
	/// This is private and should never be set manually.
	#[serde(skip_serializing_if = "Option::is_none")]
	log_id: Option<uuid::Uuid>,
}

impl<D> IntoResponse for HandlerError<D>
where
	D: Serialize + Send + Sync,
{
	/// Converts a [`HandlerError`] into a [Response].
	///
	/// Automatically logs server errors.
	fn into_response(mut self) -> Response {
		if self.status_code.is_server_error() {
			self.log_id = Some(uuid::Uuid::new_v4());
			if let Some(error) = self.error.as_ref() {
				tracing::error!("({:?}) - SERVER ERROR: {}", self.log_id, error);
			} else {
				tracing::error!("({:?}) - SERVER ERROR: {:?}", self.log_id, self.message);
			}
		}

		(self.status_code, Json(self)).into_response()
	}
}

impl<E, D> From<E> for HandlerError<D>
where
	E: Into<anyhow::Error>,
	D: Serialize + Send + Sync,
{
	/// Turns any error into a [`HandlerError`].
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
	D: Serialize + Send + Sync,
{
	/// Create a new [`HandlerError`] with status code, header and message.
	///
	/// All optional fields are set to `None`.
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

	/// Adds a custom detail to the [`HandlerError`].
	pub(crate) fn with_detail<T>(mut self, detail: T) -> Self
	where
		T: Into<D>,
	{
		self.detail = Some(detail.into());
		self
	}

	/// Adds an error to the [`HandlerError`].
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

	#[derive(Serialize)]
	struct TestDetail(i32);

	#[derive(thiserror::Error, Debug)]
	enum TestError {
		#[error("test")]
		TestField,
	}

	#[test]
	fn log_internal_server_error() -> HandlerResult<()> {
		let error: HandlerError<TestDetail> = HandlerError::new(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error",
			"If this issue persists, please contact the administrator.",
		)
		.with_detail(TestDetail(401))
		.with_error(TestError::TestField);

		// `log_id` should only be set when turned into a response.
		assert!(error.log_id.is_none());
		assert!(error.error.is_some());

		let response = error.into_response();
		assert!(response.status().is_server_error());

		Ok(())
	}

	#[test]
	fn any_error_to_handler_error() -> HandlerResult<(), HandlerError<()>> {
		"123".parse::<i32>()?;
		Ok(())
	}

	fn any_error_to_handler_error_2() -> anyhow::Result<()> {
		"123".parse::<i32>()?;
		Ok(())
	}
}

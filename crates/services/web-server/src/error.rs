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
pub(crate) struct HandlerError<D = ()>
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
	/// There might no be an acctual error,
	/// in which case this field is set to `None`.
	///
	/// Should never be exposed to the
	/// client for security reasons.
	#[serde(skip)]
	pub(crate) inner: Option<anyhow::Error>,
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
	/// This automatically logs errors.
	/// This will also set the `log_id` field
	/// so the users can report errors to get support.
	fn into_response(mut self) -> Response {
		if let Some(error) = self.inner.as_ref() {
			self.log_id = Some(uuid::Uuid::new_v4());
			if self.status_code.is_server_error() {
				tracing::error!("({:?}) - SERVER ERROR: {}", self.log_id, error);
			} else {
				tracing::error!(
					"({:?}) - UNKOWN ERROR - {}: {}",
					self.log_id,
					self.header,
					self.message
				);
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
	///
	/// This assumes that the error is an server error.
	fn from(value: E) -> Self {
		Self {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			header: String::from("Internal Server Error"),
			message: String::from("If this issue persists, please contact the administrator."),
			detail: None,
			inner: Some(value.into()),
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
			inner: None,
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
		self.inner = Some(error.into());
		self
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Serialize)]
	struct TestDetail {
		test_detail: String,
	}

	#[derive(thiserror::Error, Debug)]
	enum TestError {
		#[error("This is a test error")]
		RandomError,
	}

	#[test]
	fn log_internal_server_error() -> HandlerResult<()> {
		let detail: TestDetail = TestDetail {
			test_detail: String::from("test"),
		};

		let inner_error = TestError::RandomError;

		let error: HandlerError<TestDetail> = HandlerError::new(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error",
			"If this issue persists, please contact the administrator.",
		)
		.with_detail(detail)
		.with_error(inner_error);

		// `log_id` should only be set when turned into a response.
		assert!(error.log_id.is_none());
		assert!(error.inner.is_some());

		let response = error.into_response();
		assert!(response.status().is_server_error());

		Ok(())
	}

	#[test]
	fn any_error_to_handler_error() {
		let example_handler = || -> HandlerResult<(), HandlerError<()>> {
			"abc".parse::<i32>()?;
			Ok(())
		};
		assert!(example_handler().is_err());

		let error = example_handler().unwrap_err();
		assert!(error.status_code.is_server_error());
		assert!(error.inner.is_some())
	}
}

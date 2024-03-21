use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A type alias for `Result<T, HandlerError>`.
///
/// Used by handlers to return a response or an structured error.
pub(crate) type HandlerResult<T, D = ()> = std::result::Result<T, HandlerError<D>>;

/// Represents an error returned by a handler.
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub(crate) struct HandlerError<D = ()>
where
	D: Serialize + Send + Sync + ToSchema<'static>,
{
	/// HTTP status code for the error.
	#[serde(skip)]
	pub(crate) status_code: StatusCode,
	/// The error header.
	///
	/// The headline for the error. A short an presercice
	/// explenation of the error.
	pub(crate) header: String,
	/// The error message.
	///
	/// A more detailed describtion of what wen't wrong
	/// or what to do next.
	pub(crate) message: String,
	/// Other details about the error.
	///
	/// Does not get send to the client if it's [`None`].
	/// The some variant should implement [`ToSchema`] so that
	/// an OpenAPI scema can be generated for the type.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub(crate) detail: Option<D>,
	/// The acctual error that occured.
	///
	/// There might no be an acctual error, in which case this
	/// field is [`None`]. Should never be exposed to the client
	/// for security reasons.
	///
	/// If this field contains an error, the log_id field should
	/// also be present, to identify the error in the logs.
	#[serde(skip)]
	pub(crate) inner: Option<anyhow::Error>,
	/// The log ID of the error send to the client.
	///
	/// This is automaticaly set when the response contains an error
	/// that should be tracked. This is not public, so that it never
	/// get's set manually.
	#[serde(skip_serializing_if = "Option::is_none")]
	log_id: Option<uuid::Uuid>,
}

impl<D> IntoResponse for HandlerError<D>
where
	D: Serialize + Send + Sync + ToSchema,
{
	/// Converts a [`HandlerError`] into a [`Response`].
	///
	/// This automatically logs errors to using [`tracing`]. This also
	/// sets the log_id field so that the error can be tracked.
	fn into_response(mut self) -> Response {
		if let Some(error) = self.inner.as_ref() {
			self.log_id = if None = self.log_id {
				Some(uuid::Uuid::new_v4())
			};

			let HandlerError {
				ref header,
				ref message,
				ref log_id,
				..
			} = self;

			if self.status_code.is_server_error() {
				tracing::error!("({log_id}) - SERVER ERROR: {error}");
			} else {
				tracing::error!("({log_id}) - ERROR - {header}: {message}");
			}
		}

		(self.status_code, Json(self)).into_response()
	}
}

impl<E, D> From<E> for HandlerError<D>
where
	E: Into<anyhow::Error>,
	D: Serialize + Send + Sync + ToSchema,
{
	/// Turns any error into a [`HandlerError`].
	///
	/// This assumes that the error is an internal server error.
	/// This will also set the error in the `inner` field.
	fn from(value: E) -> Self {
		Self {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			header: String::from("Something went wrong"),
			message: String::from("If this issue persists, please contact the administrator."),
			detail: None,
			inner: Some(value.into()),
			log_id: None, // This will be set in `into_response` if inner set.
		}
	}
}

impl<D> HandlerError<D>
where
	D: Serialize + Send + Sync + ToSchema,
{
	/// Create a new [`HandlerError`] with status code, header and message.
	///
	/// All optional fields are set to `None`.
	pub(crate) fn new(
		status_code: StatusCode,
		header: impl Into<String>,
		message: impl Into<Strinb>,
	) -> Self {
		Self {
			status_code,
			header: header.into(),
			message: message.into(),
			detail: None,
			inner: None,
			log_id: None,
		}
	}

	pub(crate) fn unautharized() -> Self {
		Self {
			status_code: StatusCode::UNAUTHORIZED,
			header: String::from("Unauthorized for resource"),
			message: String::from("You do not have permission to access this resouce."),
			detail: None,
			inner: None,
			log_id: None,
		}
	}

	/// Adds a custom detail to the [`HandlerError`].
	pub(crate) fn detail<T>(mut self, detail: T) -> Self
	where
		T: Into<D>,
	{
		self.detail = Some(detail.into());
		self
	}

	/// Adds an error to the [`HandlerError`].
	pub(crate) fn error<E>(mut self, error: E) -> Self
	where
		E: Into<anyhow::Error>,
	{
		self.inner = Some(error.into());
		self
	}

	/// Sets the `log_id` field for the [`HandlerError`].
	///
	/// The `log_id` field is automatically set when an error occurs
	/// that needs to be tracked. Changing this field might make it impossible
	/// to track the error.
	pub(crate) unsafe fn set_log_id(&mut self, log_id: uuid::Uuid) {
		self.log_id = Some(log_id);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Serialize, ToSchema)]
	struct Detail {
		test_detail: String,
	}

	#[derive(thiserror::Error, Debug, ToSchema)]
	enum Error {
		#[error("This is a test error")]
		RandomError,
	}

	#[test]
	fn log_internal_server_error() -> HandlerResult<()> {
		let detail: Detail = Detail {
			test_detail: String::from("test"),
		};

		let inner_error = Error::RandomError;

		let error: HandlerError<Detail> = HandlerError::new(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Internal Server Error",
			"If this issue persists, please contact the administrator.",
		)
		.detail(detail)
		.error(inner_error);

		// `log_id` should only be set when turned into a response.
		assert!(error.log_id.is_none());
		assert!(error.inner.is_some());

		let response = error.into_response();
		assert!(response.status().is_server_error());

		Ok(())
	}

	#[test]
	fn any_error_to_handler_error() {
		let example_handler = || -> HandlerResult<(), HandlerError<()>> { "abc".parse::<i32>() };
		assert!(example_handler().is_err());

		let error = example_handler().unwrap_err();
		assert!(error.status_code.is_server_error());
		assert!(error.inner.is_some())
	}
}

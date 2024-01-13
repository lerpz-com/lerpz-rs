use axum::{extract::Request, response::Response};
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
struct AuthLayer;

impl<S> Layer<S> for AuthLayer {
	type Service = AuthMiddleware<S>;

	fn layer(&self, inner: S) -> Self::Service {
		AuthMiddleware { inner }
	}
}

#[derive(Clone)]
struct AuthMiddleware<S> {
	inner: S,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
	S: Service<Request, Response = Response> + Send + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, request: Request) -> Self::Future {
		let future = self.inner.call(request);
		Box::pin(async move {
			let response: Response = future.await?;

			Ok(response)
		})
	}
}

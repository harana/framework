use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
    Router,
};

pub async fn auth_middleware(
    request: Request<Body>,
    next: Next,
) -> Response {
    tracing::debug!(
        method = %request.method(),
        uri = %request.uri(),
        "Processing request"
    );

    next.run(request).await
}

pub trait AuthRouterExt<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn require_jwt(self) -> Self;
    fn require_session(self) -> Self;
}

impl<S> AuthRouterExt<S> for Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn require_jwt(self) -> Self {
        self
    }

    fn require_session(self) -> Self {
        self
    }
}
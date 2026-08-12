use std::sync::Arc;

use axum::{
    body::Body,
    extract::Request,
    http::{Method, StatusCode},
    response::Response,
};
use tower::Layer;

#[derive(Clone)]
pub struct CsrfLayer {
    allowed_origins: Arc<Vec<String>>,
}

impl CsrfLayer {
    pub fn new(allowed_origins: Vec<String>) -> Self {
        Self {
            allowed_origins: Arc::new(allowed_origins),
        }
    }
}

impl<S> Layer<S> for CsrfLayer {
    type Service = CsrfService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CsrfService {
            inner,
            allowed_origins: self.allowed_origins.clone(),
        }
    }
}

#[derive(Clone)]
pub struct CsrfService<S> {
    inner: S,
    allowed_origins: Arc<Vec<String>>,
}

impl<S> tower::Service<Request<Body>> for CsrfService<S>
where
    S: tower::Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let method = req.method().clone();
        let headers: Vec<_> = req
            .headers()
            .get_all("origin")
            .iter()
            .filter_map(|v| v.to_str().ok().map(String::from))
            .collect();
        let referers: Vec<_> = req
            .headers()
            .get_all("referer")
            .iter()
            .filter_map(|v| v.to_str().ok().map(String::from))
            .collect();

        let allowed = self.allowed_origins.clone();
        let mut inner = self.inner.clone();

        let content_type = req
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        Box::pin(async move {
            if method == Method::GET || method == Method::HEAD || method == Method::OPTIONS {
                return inner.call(req).await;
            }

            if content_type.starts_with("application/json") {
                return inner.call(req).await;
            }

            let is_valid = if let Some(origin) = headers.first() {
                allowed.iter().any(|a| a == origin)
            } else if let Some(referer) = referers.first() {
                extract_origin(referer)
                    .as_deref()
                    .map_or(true, |ro| allowed.iter().any(|a| a == ro))
            } else {
                false
            };

            if is_valid {
                inner.call(req).await
            } else {
                tracing::warn!(
                    method = %method,
                    ?headers,
                    ?referers,
                    "CSRF check failed"
                );
                Ok(Response::builder()
                    .status(StatusCode::FORBIDDEN)
                    .body(axum::body::Body::from("forbidden"))
                    .unwrap())
            }
        })
    }
}

fn extract_origin(url: &str) -> Option<String> {
    let end = url.find("://")?;
    let scheme = &url[..end];
    let rest = &url[end + 3..];
    let host_end = rest.find('/').unwrap_or(rest.len());
    let host = &rest[..host_end];
    Some(format!("{scheme}://{host}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_origin() {
        assert_eq!(
            extract_origin("https://example.com/path?q=1"),
            Some("https://example.com".to_string())
        );
        assert_eq!(
            extract_origin("http://localhost:3000/dashboard"),
            Some("http://localhost:3000".to_string())
        );
        assert_eq!(extract_origin("not-a-url"), None);
    }
}

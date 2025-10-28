mod path_middleware;
mod request_id;
mod token_verify;
use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::Request,
    http::{Method, StatusCode, Uri},
    middleware::from_fn,
    BoxError, Router,
};
use path_middleware::path_middleware;
use request_id::set_request_id;
use std::time::Duration;
use token_verify::token_show_verify;
use tower::{buffer::BufferLayer, limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    compression::CompressionLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use tower_http::timeout::TimeoutError;
use tower_http::timeout::TimeoutLayer;
use tracing::Span;
use tracing::{error, Level};

use crate::error::AppError;
const REQUEST_ID_HEADER: &str = "x-request-id";
const REQUEST_SHOW_TOKEN: &str = "show-token";
const REQUEST_SHOW_PASSWD: &str = "HJDIUVNQ23131d1ff1&8*";

pub fn set_layer(app: Router) -> Router {
    // print!("{:?}", ap());
    app.layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_request(|request: &Request<Body>, _span: &Span| {
                        tracing::info!("started {} {}", request.method(), request.uri().path())
                    })
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Millis),
                    )
                    .on_failure(|error, _latency: Duration, _span: &Span| {
                        tracing::error!("Request processing failed. Error: {}", error);
                    }),
            )
            .layer(from_fn(set_request_id))
            .layer(from_fn(token_show_verify))
            .layer(from_fn(path_middleware))
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(500)))
            .layer(BufferLayer::new(100))
            // 并发请求数实施限制
            .layer(ConcurrencyLimitLayer::new(10))
            .layer(CompressionLayer::new().gzip(true).br(true).deflate(true)),
    )
}

async fn handle_timeout_error(
    method: Method,
    uri: Uri,
    err: BoxError,
) -> Result<StatusCode, AppError> {
    error!(
        "request error {:?}",
        format!("`{method} {uri}` failed with {err}")
    );
    if err.is::<TimeoutError>() {
        Ok(StatusCode::REQUEST_TIMEOUT)
    } else {
        Ok(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

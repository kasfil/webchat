use axum::Router;
use http::header;
use tower_http::{
  compression::CompressionLayer, cors::CorsLayer, propagate_header::PropagateHeaderLayer,
  sensitive_headers::SetSensitiveHeadersLayer, trace,
};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::logger;
use crate::models;
use crate::routes;

pub async fn create_app() -> Router {
  #[derive(OpenApi)]
  #[openapi(
    paths(routes::status::get_status),
    components(schemas(routes::status::Status))
  )]
  struct ApiDoc;

  logger::setup();

  models::sync_indexes()
    .await
    .expect("Failed to sync database indexes");

  Router::new()
    .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
    .merge(routes::status::create_route())
    .merge(routes::user::create_route())
    .merge(Router::new().nest(
      "/v1",
      // All public v1 routes will be nested here.
      Router::new().merge(routes::cat::create_route()),
    ))
    // High level logging of requests and responses
    .layer(
      trace::TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
        .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
    )
    // Mark the `Authorization` request header as sensitive so it doesn't
    // show in logs.
    .layer(SetSensitiveHeadersLayer::new(std::iter::once(
      header::AUTHORIZATION,
    )))
    // Compress responses
    .layer(CompressionLayer::new())
    // Propagate `X-Request-Id`s from requests to responses
    .layer(PropagateHeaderLayer::new(header::HeaderName::from_static(
      "x-request-id",
    )))
    // CORS configuration. This should probably be more restrictive in
    // production.
    .layer(CorsLayer::permissive())
}

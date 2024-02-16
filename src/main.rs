use std::sync::Arc;

use axum::{body::Body, http::Request, middleware, routing::get, Router};
use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use bigchaindb_token::{
    config::Config,
    database::DatabaseConnPool,
    doc::ApiDoc,
    fallback::handler_404,
    middleware::print_request_response,
    modules::{token, wallet},
    state::AppState,
};

#[tokio::main]
async fn main() {
    // env
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let bigchain_url =
        std::env::var("BIGCHAINDB_URL").unwrap_or("http://localhost:9984/api/v1/".to_string());

    // tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "dreg=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // init state
    let db_conn_pool = DatabaseConnPool::new(&db_url).await;
    let config = Config::new(db_url, bigchain_url);
    let app_state = Arc::new(AppState::new(config, db_conn_pool));

    // app
    let app = Router::new()
        // root
        .route("/", get(|| async { "Hello, world!" }))
        // modules' routes
        .merge(wallet::routes(app_state.clone()))
        .merge(token::routes(app_state.clone()))
        // middleware layers
        .layer(middleware::from_fn(print_request_response))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                info_span!(
                    "http_request",
                    method = ?request.method(),
                    uri = ?request.uri(),
                    version = ?request.version(),
                    headers = ?request.headers(),
                )
            }),
        )
        // doc
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"));

    let app = app.fallback(handler_404);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

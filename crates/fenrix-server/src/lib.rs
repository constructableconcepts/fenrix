use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::post,
    Router,
};
use fenrix_core::ServerFn;
use serde_json::Value;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;

/// The main configuration for the Fenrix server.
#[derive(Clone)]
pub struct ServerConfig {
    pub addr: SocketAddr,
    pub assets_path: PathBuf,
    pub server_functions: Arc<HashMap<String, ServerFn>>,
}

/// Starts the Fenrix server.
pub async fn start_server(config: ServerConfig) {
    let app = Router::new()
        .route("/api/:name", post(handle_api))
        .nest_service("/", ServeDir::new(config.assets_path.clone()))
        .with_state(config.clone());

    info!("Starting server at http://{}", config.addr);
    info!(
        "Serving static assets from: {}",
        config.assets_path.display()
    );

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// The API handler that dynamically dispatches to the correct server function.
async fn handle_api(
    Path(name): Path<String>,
    State(config): State<ServerConfig>,
    Json(args): Json<Value>,
) -> (StatusCode, Json<Value>) {
    info!("Received API call for function: {}", name);

    if let Some(func) = config.server_functions.get(&name) {
        match func(args).await {
            Ok(result) => (StatusCode::OK, Json(result)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e })),
            ),
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Server function '{}' not found", name) })),
        )
    }
}
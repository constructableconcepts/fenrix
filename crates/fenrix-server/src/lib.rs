use axum::{
    extract::Path,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tracing::info;

/// The main configuration for the Fenrix server.
#[derive(Clone, Debug)]
pub struct ServerConfig {
    /// The IP address and port to bind the server to.
    pub addr: SocketAddr,
    /// The path to the directory containing the client-side assets (e.g., HTML, JS, WASM).
    pub assets_path: PathBuf,
}

/// Starts the Fenrix server.
///
/// This function initializes an `axum` server to host the application. It serves the
/// static client-side files and provides API endpoints for server functions.
///
/// # Arguments
///
/// * `config` - A `ServerConfig` struct containing the server's configuration.
///
/// # Panics
///
/// This function will panic if the server fails to start.
pub async fn start_server(config: ServerConfig) {
    let app = Router::new()
        // API route for server functions.
        .route("/api/:name", post(handle_api))
        // Route to serve static assets.
        .nest_service("/", ServeDir::new(config.assets_path.clone()));

    info!("Starting server at http://{}", config.addr);
    info!(
        "Serving static assets from: {}",
        config.assets_path.display()
    );

    let listener = tokio::net::TcpListener::bind(config.addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// A placeholder handler for server function API calls.
///
/// This function will eventually be replaced by a more robust system that
/// dynamically registers and calls server functions.
async fn handle_api(Path(name): Path<String>) -> (StatusCode, Json<Value>) {
    info!("Received API call for function: {}", name);
    let response = json!({
        "status": "success",
        "message": format!("Called server function: '{}'", name),
    });
    (StatusCode::OK, Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    // Helper to get an available port.
    fn get_available_port() -> u16 {
        std::net::TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port()
    }

    #[tokio::test]
    async fn server_starts_and_responds_to_api_call() {
        let port = get_available_port();
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        // Create a temporary directory for assets.
        let temp_dir = tempfile::tempdir().unwrap();
        let assets_path = temp_dir.path().to_path_buf();
        std::fs::File::create(assets_path.join("index.html")).unwrap();

        let config = ServerConfig { addr, assets_path };

        // Run the server in the background.
        let server_handle = tokio::spawn(start_server(config));

        // Give the server a moment to start.
        tokio::time::sleep(Duration::from_millis(100)).await;

        // Test the API endpoint.
        let client = reqwest::Client::new();
        let res = client
            .post(format!("http://{}/api/test_function", addr))
            .send()
            .await
            .expect("Failed to send request");

        assert_eq!(res.status(), reqwest::StatusCode::OK);

        let body: Value = res.json().await.expect("Failed to parse JSON response");
        assert_eq!(
            body,
            json!({
                "status": "success",
                "message": "Called server function: 'test_function'",
            })
        );

        // Test the static file serving.
        let res_static = client
            .get(format!("http://{}/index.html", addr))
            .send()
            .await
            .expect("Failed to request static file");

        assert_eq!(res_static.status(), reqwest::StatusCode::OK);

        // Shut down the server.
        server_handle.abort();
        let _ = timeout(Duration::from_secs(1), server_handle).await;
    }
}
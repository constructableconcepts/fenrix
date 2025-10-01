use fenrix_core::ServerFn;
use fenrix_server::{start_server, ServerConfig};
use server_function_example::{get_user_from_db, User};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut server_functions = HashMap::new();

    let get_user_from_db_wrapper: ServerFn = Arc::new(|args| {
        Box::pin(async move {
            let (id,): (u32,) =
                serde_json::from_value(args).map_err(|e| e.to_string())?;
            let result = get_user_from_db(id).await;
            serde_json::to_value(result).map_err(|e| e.to_string())
        })
    });
    server_functions.insert(
        "get_user_from_db".to_string(),
        get_user_from_db_wrapper,
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let assets_path = std::env::current_dir().unwrap();

    let config = ServerConfig {
        addr,
        assets_path,
        server_functions: Arc::new(server_functions),
    };

    println!("Starting server for server-function-example...");
    start_server(config).await;
}
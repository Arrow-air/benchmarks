//! Benchmark Server for the axum-graphql stack
//! axum-rest stack

use axum::{
    routing::get,
    Router,
    Server,
    handler::Handler,
};
use common::{get_bytes_100, get_bytes_1000};

/// Responds to client with 100 bytes
///
/// # Arguments
///
/// # Examples
///
/// ```
/// let app = Router::new()
/// .route("/100", get(respond_bytes_100));
/// ```
pub async fn respond_bytes_100() -> &'static str {
    get_bytes_100()
}

/// Responds to client with 1000 bytes
///
/// # Arguments
///
/// # Examples
///
/// ```
/// let app = Router::new()
/// .route("/1000", get(respond_bytes_1000));
/// ```
pub async fn respond_bytes_1000() -> &'static str {
    get_bytes_1000()
}

/// Responds a NOT_FOUND status and error string
///
/// # Arguments
///
/// # Examples
///
/// ```
/// let app = Router::new()
///         .fallback(not_found.into_service());
/// ```
pub async fn not_found(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
///
/// # Arguments
///
/// # Examples
///
/// ```
/// Server::bind(&"0.0.0.0:8000".parse().unwrap())
/// .serve(app.into_make_service())
/// .with_graceful_shutdown(shutdown_signal())
/// .await
/// .unwrap();
/// ```
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown! oh yeahhhhh");
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback(
            not_found.into_service()
        )
        .route("/100", get(respond_bytes_100))
        .route("/1000", get(respond_bytes_1000))
    ;

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
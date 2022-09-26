//! Benchmark Server for the axum-graphql stack
//! axum-graphql stack

use async_graphql_axum::GraphQLSubscription;
use axum::{
    handler::Handler,
    response::IntoResponse,
    Router, Server,
};
use common_graphql::*;

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
pub async fn not_found(uri: axum::http::Uri) -> impl IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .finish();

    let app = Router::new()
        .fallback(not_found.into_service())
        .route("/api", GraphQLSubscription::new(schema.clone()));

    println!("Try Me: {}/api", addr);

    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// use async_graphql::{
//     http::{playground_source, GraphQLPlaygroundConfig},
//     Schema,
// };
// use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    // extract::Extension,
    // response::{IntoResponse},
    routing::get,
    Router,
    Server,
    handler::Handler,
    // http::{StatusCode, Uri},
    body::StreamBody,
    http::{StatusCode},
    response::{IntoResponse},
};

use tokio_util::io::ReaderStream;
use std::{str};

// use books::{BooksSchema, MutationRoot, QueryRoot, Storage, SubscriptionRoot};

// async fn graphql_handler(schema: Extension<BooksSchema>, req: GraphQLRequest) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// async fn graphql_playground() -> impl IntoResponse {
//     response::Html(playground_source(
//         GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
//     ))
// }

pub async fn get_bytes_100() -> impl IntoResponse {
    static PATH: &str = env!("BYTES_100_PATH", "BYTES_100_PATH is not set in config.toml");
    let file = match tokio::fs::File::open(
        PATH
    ).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    Ok(StreamBody::new(stream))
}

pub async fn get_bytes_1000() -> impl IntoResponse {
    static PATH: &str = env!("BYTES_1000_PATH", "BYTES_1000_PATH is not set in config.toml");
    let file = match tokio::fs::File::open(
        PATH
    ).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    Ok(StreamBody::new(stream))
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown! oh yeahhhhh");
}

#[tokio::main]
async fn main() {
    // let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
    //     .data(Storage::default())
    //     .finish();

    let app = Router::new()
        .fallback(
            fallback.into_service()
        )
        // .route("/", get(graphql_playground).post(graphql_handler))
        // .route("/requests", GraphQLSubscription::new(schema.clone()))
        .route("/100", get(get_bytes_100))
        .route("/1000", get(get_bytes_1000))
        // .layer(Extension(schema)
    ;

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
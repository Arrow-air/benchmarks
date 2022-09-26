//! Benchmark Server for the actix-rest stack.
//! actix-rest stack.

use actix_web::{web, App, HttpResponse, HttpServer};
use common::{get_bytes_100, get_bytes_1000};
use common_rest::*;

/// Responds to client with 100 bytes.
pub async fn respond_bytes_100() -> &'static str {
    get_bytes_100()
}

/// Responds to client with 1000 bytes.
pub async fn respond_bytes_1000() -> &'static str {
    get_bytes_1000()
}

/// Responds a GET request with a list of flights.
pub async fn fetch_flights() -> HttpResponse {
    HttpResponse::Ok().json(get_flights())
}

/// Responds a POST request with a boolean value.
///
/// Expects a JSON body conforming to the [`FlightInput`] struct.
pub async fn request_flight(flight: web::Json<FlightInput>) -> HttpResponse {
    print!("{:?}", flight);
    HttpResponse::Ok().json(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "localhost";
    let port = 8000;
    let server = HttpServer::new(|| {
        App::new()
            .route("/fetch-flights", web::get().to(fetch_flights))
            .route("/request-flight", web::post().to(request_flight))
            .route("/100", web::get().to(respond_bytes_100))
            .route("/1000", web::get().to(respond_bytes_1000))
            .default_service(web::to(HttpResponse::NotFound))
    });

    println!("Try Me: http://{}:{}", addr, port);

    server
        .disable_signals()
        .bind((addr, port))?
        .run()
        .await
}

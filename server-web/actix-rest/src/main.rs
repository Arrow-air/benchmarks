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
pub async fn request_flights(flight: web::Json<FlightInput>) -> HttpResponse {
    print!("{:?}", flight);
    HttpResponse::Ok().json(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/fetch", web::get().to(fetch_flights))
            .route("/request-flight", web::post().to(request_flights))
            .route("/100", web::get().to(respond_bytes_100))
            .route("/1000", web::get().to(respond_bytes_1000))
            .default_service(web::to(|| HttpResponse::NotFound()))
    });

    println!("Playground: http://localhost:8000");

    server
        .disable_signals()
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}

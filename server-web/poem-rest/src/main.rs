//! Poem Server Example for Benchmark
//! poem-openapi

use chrono::NaiveDateTime;
use common::{get_bytes_100, get_bytes_1000};
use common_rest::get_flights;
use poem::{listener::TcpListener, Result, Route};
use poem_openapi::{
    payload::{Json, PlainText},
    ApiRequest, Object, OpenApi, OpenApiService,
};

/// Represents a flight input.
#[derive(Debug, Object)]
pub struct FlightInput {
    #[oai(validator(max_length = 64))]
    port_depart: String,
    #[oai(validator(max_length = 64))]
    port_arrive: String,
    utc_arrive_by: NaiveDateTime,
    private_charter: bool,
}

#[derive(ApiRequest)]
enum RequestFlight {
    CreateByJSON(Json<FlightInput>),
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/100", method = "get")]
    async fn respond_bytes_100(&self) -> PlainText<&'static str> {
        PlainText(get_bytes_100())
    }

    #[oai(path = "/1000", method = "get")]
    async fn respond_bytes_1000(&self) -> PlainText<&'static str> {
        PlainText(get_bytes_1000())
    }

    #[oai(path = "/request-flight", method = "post")]
    async fn request_flight(&self, _req: RequestFlight) -> Result<Json<bool>> {
        Ok(Json(true))
    }

    #[oai(path = "/fetch-flights", method = "get")]
    async fn fetch_flights(&self) -> Result<Json<String>> {
        Ok(Json(serde_json::to_string(&get_flights()).unwrap()))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let addr = "http://localhost:8000/";
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server(addr);
    let app = Route::new().nest("/", api_service);

    println!("Try Me: {}", addr);
    poem::Server::new(TcpListener::bind(addr))
        .run(app)
        .await
}

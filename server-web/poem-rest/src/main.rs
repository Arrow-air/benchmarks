//! Poem Server Example for Benchmark
//! poem-openapi

use poem::{listener::TcpListener, Route, Result};
use poem_openapi::{
    payload::{PlainText, Json},
    Object,
    ApiRequest,
    OpenApi,
    OpenApiService};
use common::{get_bytes_100, get_bytes_1000};
use common_rest::get_flights;
use chrono::NaiveDateTime;

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
    CreateByJSON(Json<FlightInput>)
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
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:8000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    println!("Live! Visit http://localhost:8000");
    poem::Server::new(TcpListener::bind("127.0.0.1:8000"))
        .run(app)
        .await
}
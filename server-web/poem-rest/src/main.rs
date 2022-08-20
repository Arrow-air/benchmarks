//! Poem Server Example for Benchmark
//! poem-openapi

use poem::{listener::TcpListener, Route, Result};
use poem_openapi::{payload::{PlainText, Json}, OpenApi, OpenApiService};
use common::{get_bytes_100, get_bytes_1000};
use common_rest::{Flight, get_flights};

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

    #[oai(path = "/request", method = "post")]
    async fn request_flight(&self) -> Result<Json<bool>> {
        Ok(Json(true))
    }

    // #[oai(path = "/fetch", method = "get")]
    // async fn fetch_flights(&self) -> Result<Json<impl Serialize>> {
    //     Ok(Json(serialize(get_flights()))
    // }  
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/api", api_service).nest("/", ui);

    println!("Live! Visit http://localhost:3000");
    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}
//! Shared functions used for examples for all frameworks

use axum::http::StatusCode;
use chrono::NaiveDate;
use common_rest::FlightInput;
use hyper::{Body, Client, Method, Request};
use std::env;
use std::time::Duration;

/// Provide endpoint url to use
pub fn get_endpoint() -> String {
    // Set default hostname and port
    let mut hostname = "localhost".to_string();
    let mut port = "8080".to_string();

    // Allow override of hostname and port through env vars
    if env::var("HOSTNAME").is_ok() {
        if let Ok(v) = env::var("HOSTNAME") {
            hostname = v
        }
    }
    if env::var("DOCKER_PORT").is_ok() {
        if let Ok(v) = env::var("DOCKER_PORT") {
            port = v
        }
    }

    format!("http://{}:{}", hostname, port)
}

/// Lightly exercise the following endpoints:
/// - /100
/// - /1000
/// - /fetch-flights
/// - /create-flight
pub async fn test_rest_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let url: &str = &get_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        url
    );

    let mut ok = true;
    let client = Client::builder()
        .pool_idle_timeout(Duration::from_secs(10))
        .build_http();

    // GET /100
    let mut uri = format!("{}/{}", url, "100");
    let mut req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    let mut resp = client.request(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK);
    ok &= resp.status() == StatusCode::OK;

    // GET /1000
    uri = format!("{}/{}", url, "1000");
    req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    resp = client.request(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK);
    ok &= resp.status() == StatusCode::OK;

    // GET /fetch-flights
    uri = format!("{}/{}", url, "fetch-flights");
    req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    resp = client.request(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK);
    ok &= resp.status() == StatusCode::OK;

    // POST /create-flight
    let data = FlightInput {
        port_depart: "vertiport-1".to_string(),
        port_arrive: "vertiport-2".to_string(),
        utc_arrive_by: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        private_charter: true,
    };
    let data_str = serde_json::to_string(&data).unwrap();
    uri = format!("{}/{}", url, "create-flight");
    req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(data_str))
        .unwrap();

    resp = client.request(req).await.unwrap();
    assert!(resp.status() == StatusCode::OK);
    ok &= resp.status() == StatusCode::OK;

    if ok {
        println!("\u{1F942} All endpoints responded!");
    }

    Ok(())
}

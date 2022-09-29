//! Endpoints Example
//! `make run` in a separate window before executing this example

use common_example;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // URL from another docker container on a network bridge
    let url: &str = "http://arrow-benchmarks-actix-rest-run:8000";
    // URL from host
    // let url: &str = "http://localhost:8080"; // from localhost

    common_example::test_rest_endpoints(url).await
}

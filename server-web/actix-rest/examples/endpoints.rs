//! Endpoints Example
//! `make run` in a separate window before executing this example

use common_example;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common_example::test_rest_endpoints().await
}

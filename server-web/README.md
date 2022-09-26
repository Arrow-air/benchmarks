# Web Framework Benchmark

## Instructions

`server-web` is a Rust workspace, meaning you can run `cargo build/run -p <project>` from this directory.
- For example, `cargo run -p axum-graphql`.

Inspired by [this benchmark](https://github.com/piaoger/webframework-bench).

Benchmarking the following tasks:
1. Get Flight Information
    - id: String
    - departure_port: String
    - arrival_port: String
    - utc_departure: NaiveDateTime
    - utc_arrival: NaiveDateTime
    - private_charter: bool
2. Request a flight with the following arguments:
    - departure_port: String
    - arrival_port: String
    - utc_arrive_by: NaiveDateTime
    - private_charter: bool
    - Just return `true` on this
2. 100 Bytes
3. 1000 Bytes
4. Fallback (request invalid location)

Optimized by:
- Speed (`opt-level = 3`)
- Size (`opt-level = "z"`)

## Candidates

Stacks |  Test
--- | ---
[axum-rest](./axum-rest/README.md) |
[axum-graphql](./axum-graphql/README.md) |
[actix-rest](./actix-rest/README.md) |
[actix-graphql](./actix-graphql/README.md) |
[hyper-grpc (Petshop)](./hyper-grpc/README.md) |
[poem-rest (poem-openapi)](./poem-rest/README.md) |

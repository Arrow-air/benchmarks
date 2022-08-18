# Web Framework Benchmark

## Instructions

`server-web` is a Rust workspace, meaning you can run `cargo build/run -p <project>` from this directory.
- For example, `cargo run -p axum-graphql`.

Based off of [this benchmark](https://github.com/piaoger/webframework-bench).

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

## Candidates

Stacks |  Test
--- | ---
[axum-rest](./axum-rest/README.md) |
[axum-graphql](./axum-graphql/README.md) |
[actix-rest](./actix-rest/README.md) |
[actix-graphql](./actix-graphql/README.md) |
[hyper-grpc (Petshop)](./hyper-grpc/README.md) |
[poem-rest (poem-openapi)](./poem-rest/README.md) |

## Optimized for Speed


```toml
# Cargo.toml
[profile.release]
opt-level = 3
```

```bash
cargo build --release
```

### Binary Size

Framework | Memory (Bytes) | Cache (Bytes)
--- | --- | ---


### Memory

Framework | Memory (MiB) | Cache (MiB)
--- | --- | ---

### Throughput

Framework | Transfer/sec (MiB) | Requests/sec
--- | --- | ---


## Optimized for Size

```toml
# Cargo.toml
[profile.release]
opt-level = "z" # or "s"
```

```bash
cargo build --release
```

### Binary Size

Framework | Memory (Bytes) | Cache (Bytes)
--- | --- | ---

### Memory

Framework | Memory (MiB) | Cache (MiB)
--- | --- | ---

### Throughput

Framework | Transfer/sec (MiB) | Requests/sec
--- | --- | ---


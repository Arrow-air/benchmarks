# Web Framework Benchmark

## Build & Test

To build and run the Docker container for each framework:
```bash
# Build
cd server-web/axum-rest/ # or other framework
make docker-build # includes cargo build --release

# Run Server
make docker-run # server starts immediately
docker ps # confirm the container is running
```

Run the integration test (`endpoints.rs`) for a sanity check:
```bash
make example
```

To kill the container:
```bash
# Kill container
make docker-stop
```

## Required Endpoints

Required Endpoint | Arguments | Return Value
--- | --- | ---
GET /fetch-flights | None | **id:** String<BR>**port_depart:** String<BR>**port_arrive:** String<BR>**utc_depart:** NaiveDateTime<BR>**utc_arrive:** NaiveDateTime<BR>**private_charter:** bool
POST /create-flight | **port_depart:** String<br>**port_arrive:** String<br>**utc_arrive_by:** NaiveDateTime<br>**private_charter:** bool | true
GET /100 | None | 100 Arbitrary Bytes
GET /1000 | None | 1000 Arbitrary Bytes


Optimized by:
- Speed (`opt-level = 3`)
- Size (`opt-level = "z"`)

## Required Files in `server-web/<framework>/`

`examples/endpoints.rs`
- These are executed with `make example`
- For a sanity check; lightly exercises each endpoint
- See `server-web/axum-rest/tests/endpoints.rs` for example
- The `Cargo.toml` should have an `[[example]]` entry with `name = "endpoints"`

`Makefile` & `.makerc`:
- See `server-web/axum-rest/Makefile` and `server-web/axum-rest/.makerc` for examples

## Candidates

Stacks |  Test
--- | ---
[axum-rest](./axum-rest/README.md) |
[axum-graphql](./axum-graphql/README.md) |
[actix-rest](./actix-rest/README.md) |
[actix-graphql](./actix-graphql/README.md) |
[hyper-grpc (Petshop)](./hyper-grpc/README.md) |
[poem-rest (poem-openapi)](./poem-rest/README.md) |

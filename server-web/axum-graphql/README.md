# axum + graphql

## Stack

| Library | Version | Description
| --- | --- | --- |
| axum | [0.5.15](https://docs.rs/axum/0.5.15/axum/) | `tokio` for async runtime and utilities<br>`hyper` for http server<br>`tower` for  middleware and utilities<br>Includes `tonic` (gRPC)
| async-graphql | 4.0.9 | |

## Try It 
```bash
$ curl -i -H 'Content-Type: application/json' -X POST \
   -d '{"query": "query {fetch_flights() {}}"}' \
   http://localhost:8000/api
```

## Resources
- [Joel Parker Henderson - Axum Tutorial](https://github.com/joelparkerhenderson/demo-rust-axum)
- [Roman Kudryashov - GraphQL in Rust](https://romankudryashov.com/blog/2020/12/graphql-rust/)

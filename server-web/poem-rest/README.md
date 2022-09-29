# poem + REST
## Stack

| Library | Version | Description
| --- | --- | --- |
| poem | 1.3.40 | 
| poem-openapi | 2.0.10 |

## Try It

1. Start the server
```bash
$ cargo run
   Compiling poem-rest v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 3.89s
     Running `target/debug/poem-rest`
Try Me: http://127.0.0.1:8000
```

2. Use `curl` to send a request
```bash
curl -X 'POST' \
'http://127.0.0.1:8000/create-flight' \
-H 'accept: application/json' \
-H 'Content-Type: application/json' \
-d '{
  "port_depart": "string",
  "port_arrive": "string",
  "utc_arrive_by": "2022-08-18T23:59:57",
  "private_charter": true
}'
```

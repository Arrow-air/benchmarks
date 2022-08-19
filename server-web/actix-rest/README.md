# actix + REST

## Stack

| Library   | Version                                             | Description                        |
| --------- | --------------------------------------------------- | ---------------------------------- |
| actix-web | [4.1.0](https://docs.rs/actix-web/4.1.0/actix_web/) | Actix is a web framework for Rust. |

## Try It

There are many way to interact with REST APIs. The most native way is to
use the `curl` command in your terminal. But you are welcome to use
tools like [Postman](https://www.postman.com/) to test the server.

1. Start the server

```bash
$ cargo run
```

2. In your terminal, make a `GET` request for a list of flights

```bash
$ curl http://localhost:8000/
```

It should print an array of flights (only have 1 in our example):

```json
[
  {
    "id": "EARTH-NAMERICA-FN-0123456789",
    "port_depart": "EARTH-NAMERICA-10",
    "port_arrive": "EARTH-NAMERICA-11",
    "utc_depart": "2022-08-18T23:59:57",
    "utc_arrive": "2022-08-19T00:22:10",
    "private_charter": false
  }
]
```

3. Try to make a `POST` request to the same URL

```bash
$ curl -X POST http://localhost:8000/requests
```

It should print true:

```json
true
```

## Resources

- [Official Actix Tutorial](https://actix.rs/docs/getting-started/)

# Web Framework Benchmark

## Instructions

`server-web` is a Rust workspace, meaning you can run `cargo build/run -p <project>` from this directory.
- For example, `cargo run -p axum-graphql`.

Based off of [this benchmark](https://github.com/piaoger/webframework-bench).

1. fallback
2. shutdown signal



## Candidates

Stacks |  Test
--- | ---
[axum-openapi](./axum-openapi/README.md) |
[axum-graphql](./axum-graphql/README.md) |

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
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

### Memory

Framework | Memory (MiB) | Cache (MiB)
--- | --- | ---
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

### Throughput

Framework | Transfer/sec (MiB) | Requests/sec
--- | --- | ---
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

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
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

### Memory

Framework | Memory (MiB) | Cache (MiB)
--- | --- | ---
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

### Throughput

Framework | Transfer/sec (MiB) | Requests/sec
--- | --- | ---
axum, async-graphql| |
hyper, openapi | |
actix-web, openapi | | 
poem, openapi | |

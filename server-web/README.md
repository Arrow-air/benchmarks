# Web Framework Benchmark

Based off of [this benchmark](https://github.com/piaoger/webframework-bench).

1. fallback
2. shutdown signal

## Candidates

Library | Version
--- | --- 
axum, async-graphql | v4.1.0
hyper, openapi | v0.5.15
actix-web, openapi | v1.3.37

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

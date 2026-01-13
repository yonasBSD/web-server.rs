# web-server.rs

![Licenses](https://github.com/yonasBSD/web-server.rs/actions/workflows/licenses.yaml/badge.svg)
![Linting](https://github.com/yonasBSD/web-server.rs/actions/workflows/lint.yaml/badge.svg)
![Testing](https://github.com/yonasBSD/web-server.rs/actions/workflows/test-with-coverage.yaml/badge.svg)
![Packaging](https://github.com/yonasBSD/web-server.rs/actions/workflows/release-packaging.yaml/badge.svg)
![Cross-Build](https://github.com/yonasBSD/web-server.rs/actions/workflows/cross-build.yaml/badge.svg)

![Security Audit](https://github.com/yonasBSD/web-server.rs/actions/workflows/security.yaml/badge.svg)
![Scorecard Audit](https://github.com/yonasBSD/web-server.rs/actions/workflows/scorecard.yaml/badge.svg)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_web-server.rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=yonasBSD_web-server.rs)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_web-server.rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=yonasBSD_web-server.rs)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=yonasBSD_web-server.rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=yonasBSD_web-server.rs)
<!--[![codecov](https://codecov.io/gh/yonasBSD/web-server.rs/branch/main/graph/badge.svg?token=SLIHSUWHT2)](https://codecov.io/gh/yonasBSD/web-server.rs)-->
<!--[![ghcr.io](https://img.shields.io/badge/ghcr.io-download-blue)](https://github.com/yonasBSD/web-server.rs/pkgs/container/web-server.rs)-->
<!--[![Docker Pulls](https://img.shields.io/docker/pulls/web-server.rs/example.svg)](https://hub.docker.com/r/web-server.rs/example)-->
<!--[![Quay.io](https://img.shields.io/badge/Quay.io-download-blue)](https://quay.io/repository/web-server.rs/example)-->

![GitHub last commit](https://img.shields.io/github/last-commit/yonasBSD/web-server.rs)
[![Dependency Status](https://deps.rs/repo/github/yonasBSD/web-server.rs/status.svg)](https://deps.rs/repo/github/yonasBSD/web-server.rs)
![Rust](https://img.shields.io/badge/Built%20With-Rust-orange?logo=rust)
[![GitHub Release](https://img.shields.io/github/release/yonasBSD/web-server.rs.svg)](https://github.com/yonasBSD/web-server.rs/releases/latest)
[![License](https://img.shields.io/github/license/yonasBSD/web-server.rs.svg)](https://github.com/yonasBSD/web-server.rs/blob/main/LICENSE.txt)
<!--[![Matrix Chat](https://img.shields.io/matrix/vaultwarden:matrix.org.svg?logo=matrix)](https://matrix.to/#/#vaultwarden:matrix.org)-->

High performance, general purpose web server.

## Features
- based on high performance Axum web framework
- uses Tower middleware
- Tako web framework optimizations
- built-in OpenAPI support (Swagger UI and Scalar)


## Example

```rust
//! OpenAPI documentation example for Tako.
//!
//! This example demonstrates how to use route-level OpenAPI metadata
//! to generate an OpenAPI specification from your Tako routes.
//!
//! Run with: cargo run --example openapi

use anyhow::Result;
use web_server_rs::prelude::*;
use web_server_rs::*;

async fn hello() -> Response {
    http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::from("Hello World"))
        .unwrap()
}

async fn health() -> impl Responder {
    Json(json!({ "status": "healthy" }))
}

#[tokio::main]
async fn main() -> Result<()> {
    serve(
        vec![
            Route {
                method: Method::GET,
                path: "/",
                handler: handler!(hello),
                operation_id: "hello",
                summary: "Hello endpoint",
                description: None,
                tag: "example",
                response_code: 200,
                response_desc: "OK",
            },
            Route {
                method: Method::GET,
                path: "/health",
                handler: handler!(health),
                operation_id: "health",
                summary: "health endpoint",
                description: None,
                tag: "example",
                response_code: 200,
                response_desc: "OK",
            },
        ],
        "0.0.0.0",
        3000,
        "My Example",
        "0.1.0",
        ServerConfig::new(),
    )
    .await
}
```

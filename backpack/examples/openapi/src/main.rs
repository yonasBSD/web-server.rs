//! OpenAPI documentation example for Tako.
//!
//! This example demonstrates how to use route-level OpenAPI metadata
//! to generate an OpenAPI specification from your Tako routes.
//!
//! Run with: cargo run --example openapi

use anyhow::Result;
use web_server_rs::{prelude::*, *};

async fn hello() -> Response {
    http::Response::builder()
        .status(http::StatusCode::OK)
        .body(Body::from("Hello World"))
        .unwrap()
}

async fn health() -> impl Responder {
    Json!({ "status": "healthy" })
}

#[tokio::main]
async fn main() -> Result<()> {
    let config: ServerConfig = ServerConfig {
        routes: vec![
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
        address: "0.0.0.0",
        port: 3000,
        title: "My Example",
        ..Default::default()
    };

    serve(config).await
}

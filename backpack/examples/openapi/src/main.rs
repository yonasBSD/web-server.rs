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

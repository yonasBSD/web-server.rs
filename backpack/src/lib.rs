//! OpenAPI documentation example for Tako.
//!
//! This example demonstrates how to use route-level OpenAPI metadata
//! to generate an OpenAPI specification from your Tako routes.
//!
//! Run with: cargo run --example openapi

mod banner;

// Re-export for users of your crate
pub use tako::Method;

pub mod prelude {
    pub use serde_json::json;
    pub use tako::Method;
    pub use tako::body::TakoBody as Body;
    pub use tako::extractors::json::Json;
    pub use tako::responder::Responder;
    pub use tako::types::{Request, Response};
}

use anyhow::Result;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tako::openapi::ui::Scalar;
use tako::openapi::vespera::{Info, VesperaOpenApiJson, generate_openapi_from_routes};
use tako::{
    handler::Handler,
    responder::Responder,
    router::Router,
    types::{Request, Response},
};
use tokio::net::TcpListener;

pub struct ServerConfig {
    pub desc: Option<String>,
    pub summary: Option<String>,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            desc: None,
            summary: None,
        }
    }
}

/// Type-erased async future returning a full HTTP response.
pub type BoxFutureResponse = Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

/// A handler wrapper that stores any function or closure
/// that takes a Request and returns a Future producing a Response.
#[derive(Clone)]
pub struct FnHandler {
    func: Arc<dyn Fn(Request) -> BoxFutureResponse + Send + Sync + 'static>,
}

impl FnHandler {
    pub fn new<F>(func: F) -> Self
    where
        F: Fn(Request) -> BoxFutureResponse + Send + Sync + 'static,
    {
        Self {
            func: Arc::new(func),
        }
    }
}

impl Handler<Request> for FnHandler {
    type Future = BoxFutureResponse;

    fn call(self, req: Request) -> Self::Future {
        (self.func)(req)
    }
}

pub fn handler_fn<F, Fut, R>(f: F) -> FnHandler
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = R> + Send + 'static,
    R: Responder + 'static,
{
    let f = std::sync::Arc::new(f);

    FnHandler::new(move |_req| {
        let f = f.clone(); // clone the Arc, not the closure
        Box::pin(async move {
            let resp = f().await;
            resp.into_response()
        })
    })
}

#[macro_export]
macro_rules! handler {
    ($arg:expr) => {
        handler_fn($arg)
    };
}

/// A route definition with OpenAPI metadata.
pub struct Route {
    pub method: Method,
    pub path: &'static str,
    pub handler: FnHandler,
    pub operation_id: &'static str,
    pub summary: &'static str,
    pub description: Option<&'static str>,
    pub tag: &'static str,
    pub response_code: u16,
    pub response_desc: &'static str,
}

/// Apply all routes to the router.
pub fn setup_router(router: &mut Router, routes: &[Route]) {
    for r in routes {
        router
            .route(r.method.clone(), r.path, r.handler.clone())
            .operation_id(r.operation_id)
            .summary(r.summary)
            .description(r.description.unwrap_or(""))
            .tag(r.tag)
            .response(r.response_code, r.response_desc);
    }
}

/// Full server + OpenAPI setup.
pub async fn serve(
    routes: Vec<Route>,
    address: &'static str,
    port: u64,
    title: &'static str,
    version: &'static str,
    config: ServerConfig,
) -> Result<()> {
    let bind = format!("{address}:{port}");
    let listener = TcpListener::bind(&bind).await?;

    // OpenAPI metadata
    let info = Info {
        title: title.to_string(),
        version: version.to_string(),
        description: config.desc,
        terms_of_service: None,
        contact: None,
        license: None,
        summary: config.summary,
    };

    // Build router
    let mut router = Router::new();
    setup_router(&mut router, &routes);

    // Generate OpenAPI spec
    let spec = generate_openapi_from_routes(&router, info);

    // Serve OpenAPI JSON
    router.route(Method::GET, "/openapi.json", {
        let spec = spec.clone();
        move |_: Request| async move { VesperaOpenApiJson(spec.clone()) }
    });

    // Serve Scalar UI
    router.route(Method::GET, "/docs", move |_: Request| async move {
        Scalar::new("/openapi.json").title(format!("{title} - Scalar"))
    });

    banner::print(banner::BannerConfig {
        name: "http server",
        version: env!("CARGO_PKG_VERSION"),
        tagline: "High performance, minimalist Rust web service",
        addr: "0.0.0.0:3000",
    });

    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    let label_width = 18; // enough to align the longest label
    println!(
        "{:<label_width$} {GREEN}http://{bind}{RESET}",
        "Server running at",
        label_width = label_width
    );
    println!(
        "{:<label_width$} {GREEN}http://{bind}/openapi.json{RESET}",
        "OpenAPI spec:",
        label_width = label_width
    );
    println!(
        "{:<label_width$} {GREEN}http://{bind}/docs{RESET}\n",
        "Scalar UI:",
        label_width = label_width
    );

    tako::serve(listener, router).await;

    Ok(())
}

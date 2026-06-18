use std::{env, error::Error, net::SocketAddr, time::Instant};

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    started_at: Instant,
    version: &'static str,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    uptime: u64,
}

#[derive(Clone, Serialize)]
struct UseCaseResponse {
    name: &'static str,
    slug: &'static str,
    summary: &'static str,
    status: &'static str,
    stack: Vec<&'static str>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = AppState {
        started_at: Instant::now(),
        version: env!("CARGO_PKG_VERSION"),
    };

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/use-cases", get(use_cases))
        .route("/api/projects", get(use_cases))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = env::var("QUERY_IN_BACKEND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3001".to_owned())
        .parse::<SocketAddr>()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Query In backend listening on http://{addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: state.version,
        uptime: state.started_at.elapsed().as_secs(),
    })
}

async fn use_cases() -> Json<Vec<UseCaseResponse>> {
    Json(use_case_catalog())
}

fn use_case_catalog() -> Vec<UseCaseResponse> {
    vec![
        UseCaseResponse {
            name: "Operations Review",
            slug: "query-in",
            summary: "Inspect exports from billing, support, product, or CRM tools without uploading sensitive files into another SaaS system.",
            status: "Ready",
            stack: vec!["Private CSVs", "SQL", "Schema inference", "Export"],
        },
        UseCaseResponse {
            name: "Pre-Warehouse Triage",
            slug: "local-query-engine",
            summary: "Validate file shape, inspect nulls, and answer quick questions before deciding whether data belongs in the warehouse.",
            status: "Ready",
            stack: vec![
                "Schema checks",
                "Local compute",
                "Query history",
                "Results table",
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::use_case_catalog;

    #[test]
    fn use_case_catalog_contains_query_in() {
        let use_cases = use_case_catalog();

        assert!(use_cases.iter().any(|use_case| use_case.slug == "query-in"));
        assert!(use_cases.iter().all(|use_case| !use_case.stack.is_empty()));
    }
}

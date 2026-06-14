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
struct ProjectResponse {
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
        .route("/api/projects", get(projects))
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

async fn projects() -> Json<Vec<ProjectResponse>> {
    Json(project_catalog())
}

fn project_catalog() -> Vec<ProjectResponse> {
    vec![
        ProjectResponse {
            name: "Operations Review",
            slug: "query-in",
            summary: "Inspect exports from billing, support, product, or CRM tools without uploading sensitive files into another SaaS system.",
            status: "Ready",
            stack: vec!["Private CSVs", "SQL", "Schema inference", "Export"],
        },
        ProjectResponse {
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
    use super::project_catalog;

    #[test]
    fn project_catalog_contains_query_in() {
        let projects = project_catalog();

        assert!(projects.iter().any(|project| project.slug == "query-in"));
        assert!(projects.iter().all(|project| !project.stack.is_empty()));
    }
}

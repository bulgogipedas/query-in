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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = AppState {
        started_at: Instant::now(),
        version: env!("CARGO_PKG_VERSION"),
    };

    let app = Router::new()
        .route("/api/health", get(health))
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

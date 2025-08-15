use crate::options::StartOptions;
use crate::{db, import};
use anyhow::anyhow;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router, serve};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::db::AssetRepository;

/// The state of the HTTP server that is accessible to all HTTP handlers.
#[derive(Clone)]
pub struct ServerState {
    pub asset_repository: AssetRepository,
}

impl ServerState {
    /// Creates a new instance of the server state.
    pub fn new(asset_repository: AssetRepository) -> Self {
        Self { asset_repository }
    }
}


pub async fn start_server(start_options: &StartOptions) -> anyhow::Result<()> {
    let asset_repository =
        db::AssetRepository::new(&start_options.database_path, 5, Duration::from_secs(10))?;

    asset_repository.setup()?;

    let listener = TcpListener::bind(&start_options.address).await?;

    let app = Router::new()
        .route("/import", post(import_assets))
        .layer((
            TraceLayer::new_for_http(),
            // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
            // requests don't hang forever.
            TimeoutLayer::new(Duration::from_secs(300)),
        ))
        .with_state(ServerState::new(asset_repository));

    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn import_assets(
    State(server_state): State<ServerState>,
    // Ideally we would check whether we already imported these assets to support idempotency.
    // One option would be to hash the assets with a cryptographically secure hash and store the hashes in the database.
    // Another options would be to allow the caller to pass in an idempotency key header.
    Json(assets): Json<Vec<import::Asset>>,
) -> Response {
    let (good, bad): (Vec<_>, Vec<_>) = assets
        .into_iter()
        .map(import::Asset::try_into) // This is performing validation.
        .enumerate()
        .map(|(index, result)| {
            result.map_err(|e| anyhow!("Failed to convert asset at index {}: {}", index, e))
        })
        .partition(Result::is_ok);

    if !bad.is_empty() {
        let errors = bad
            .into_iter()
            .filter_map(Result::err)
            .map(|e: anyhow::Error| e.to_string())
            .collect::<Vec<_>>();
        return (StatusCode::BAD_REQUEST, Json(errors)).into_response();
    }

    let assets: Vec<db::Asset> = good.into_iter().filter_map(Result::ok).collect();

    match server_state.asset_repository.insert(assets) {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(e) => {
            // at this point, we have already validated the assets, so this is likely a database error.
            tracing::error!(%e, "Failed to import assets");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// Represents a request to get the balance of a specific asset as of a certain date.
#[derive(Debug, Serialize, Deserialize)]
struct BalanceRequest {
    asset: String,
    as_of: DateTime<Utc>,
}

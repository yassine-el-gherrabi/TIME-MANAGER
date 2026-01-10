//! Prometheus metrics endpoint handler

use axum::{extract::State, http::StatusCode, response::IntoResponse};
use crate::config::AppState;

/// GET /metrics
///
/// Returns Prometheus-formatted metrics for scraping
pub async fn get_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let metrics = state.metrics_service.render();
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4; charset=utf-8")],
        metrics,
    )
}

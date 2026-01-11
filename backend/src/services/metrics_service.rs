//! SQL and application metrics service using Prometheus
//!
//! Provides metrics for:
//! - SQL query duration (histogram)
//! - SQL query count (counter by operation type)
//! - HTTP request metrics
//! - Application health metrics

use metrics::{counter, describe_counter, describe_histogram, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::time::Instant;

/// Metrics service for application observability
pub struct MetricsService {
    handle: PrometheusHandle,
}

impl MetricsService {
    /// Initialize the metrics service with Prometheus exporter
    pub fn new() -> Self {
        // Build the Prometheus exporter
        let handle = PrometheusBuilder::new()
            .install_recorder()
            .expect("Failed to install Prometheus recorder");

        // Describe SQL metrics
        describe_histogram!(
            "sql_query_duration_seconds",
            "Duration of SQL queries in seconds"
        );
        describe_counter!("sql_query_total", "Total number of SQL queries executed");
        describe_counter!("sql_query_errors_total", "Total number of SQL query errors");

        // Describe HTTP metrics
        describe_histogram!(
            "http_request_duration_seconds",
            "Duration of HTTP requests in seconds"
        );
        describe_counter!("http_requests_total", "Total number of HTTP requests");

        // Describe application metrics
        describe_counter!(
            "auth_login_attempts_total",
            "Total number of login attempts"
        );
        describe_counter!(
            "auth_login_success_total",
            "Total number of successful logins"
        );
        describe_counter!(
            "auth_login_failure_total",
            "Total number of failed logins"
        );

        Self { handle }
    }

    /// Get the Prometheus metrics output for scraping
    pub fn render(&self) -> String {
        self.handle.render()
    }
}

impl Default for MetricsService {
    fn default() -> Self {
        Self::new()
    }
}

/// SQL query timer for measuring query duration
pub struct SqlQueryTimer {
    start: Instant,
    operation: &'static str,
    table: &'static str,
}

impl SqlQueryTimer {
    /// Start timing a SQL query with static string labels
    pub fn new(operation: &'static str, table: &'static str) -> Self {
        Self {
            start: Instant::now(),
            operation,
            table,
        }
    }

    /// Record the query as successful
    pub fn success(self) {
        let duration = self.start.elapsed().as_secs_f64();
        let labels = [
            ("operation", self.operation),
            ("table", self.table),
            ("status", "success"),
        ];

        histogram!("sql_query_duration_seconds", &labels).record(duration);
        counter!("sql_query_total", &labels).increment(1);
    }

    /// Record the query as failed
    pub fn error(self) {
        let duration = self.start.elapsed().as_secs_f64();
        let labels = [
            ("operation", self.operation),
            ("table", self.table),
            ("status", "error"),
        ];

        histogram!("sql_query_duration_seconds", &labels).record(duration);
        counter!("sql_query_total", &labels).increment(1);

        let error_labels = [
            ("operation", self.operation),
            ("table", self.table),
        ];
        counter!("sql_query_errors_total", &error_labels).increment(1);
    }
}

/// Helper macro for timing SQL operations
#[macro_export]
macro_rules! time_sql {
    ($operation:expr, $table:expr, $query:expr) => {{
        let timer = $crate::services::metrics_service::SqlQueryTimer::new($operation, $table);
        match $query {
            Ok(result) => {
                timer.success();
                Ok(result)
            }
            Err(e) => {
                timer.error();
                Err(e)
            }
        }
    }};
}

/// Normalize path by replacing UUIDs and IDs with placeholders
fn normalize_path(path: &str) -> String {
    use regex::Regex;
    use once_cell::sync::Lazy;

    // Regex for UUID format (e.g., 123e4567-e89b-12d3-a456-426614174000)
    static UUID_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
            .unwrap()
    });

    // Replace UUIDs with :id placeholder
    UUID_RE.replace_all(path, ":id").to_string()
}

/// Record an HTTP request metric with dynamic path normalization
pub fn record_http_request(method: &str, path: &str, status: u16, duration_secs: f64) {
    let status_class = match status {
        200..=299 => "2xx",
        300..=399 => "3xx",
        400..=499 => "4xx",
        500..=599 => "5xx",
        _ => "other",
    };

    // Normalize path to avoid high cardinality
    let normalized_path = normalize_path(path);

    histogram!("http_request_duration_seconds",
        "method" => method.to_string(),
        "path" => normalized_path.clone(),
        "status" => status_class.to_string()
    ).record(duration_secs);

    counter!("http_requests_total",
        "method" => method.to_string(),
        "path" => normalized_path,
        "status" => status_class.to_string()
    ).increment(1);
}

/// Record authentication metrics
pub fn record_login_attempt(success: bool) {
    counter!("auth_login_attempts_total").increment(1);

    if success {
        counter!("auth_login_success_total").increment(1);
    } else {
        counter!("auth_login_failure_total").increment(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_query_timer_success() {
        // This test validates the timer structure works
        let timer = SqlQueryTimer::new("SELECT", "users");
        assert_eq!(timer.operation, "SELECT");
        assert_eq!(timer.table, "users");
    }

    #[test]
    fn test_sql_query_timer_error() {
        let timer = SqlQueryTimer::new("INSERT", "clocks");
        assert_eq!(timer.operation, "INSERT");
        assert_eq!(timer.table, "clocks");
    }
}

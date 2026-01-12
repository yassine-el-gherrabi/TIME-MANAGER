//! Integration tests with real database connections.
//!
//! These tests use testcontainers to spin up ephemeral PostgreSQL instances
//! for realistic database interaction testing.
//!
//! Run with: `cargo test --test integration -- --ignored`

pub mod test_fixtures;

// API endpoint integration tests
pub mod auth_tests;
pub mod users_tests;
pub mod teams_tests;
pub mod absences_tests;
pub mod kpis_tests;
pub mod notifications_tests;
pub mod schedules_tests;
pub mod breaks_tests;
pub mod balances_tests;

// Workflow integration tests
pub mod clock_flow_tests;

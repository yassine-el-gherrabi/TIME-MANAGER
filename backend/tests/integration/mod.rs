//! Integration tests with real database connections.
//!
//! These tests use testcontainers to spin up ephemeral PostgreSQL instances
//! for realistic database interaction testing.
//!
//! Run with: `cargo test --test integration -- --ignored`

pub mod test_fixtures;
pub mod clock_flow_tests;

//! Business workflow tests.
//!
//! These tests verify complete business workflows like absence approval,
//! clock override approval, and other multi-step processes.
//!
//! Run with: `cargo test --test workflows -- --ignored`

pub mod absence_approval_workflow;
pub mod clock_approval_workflow;

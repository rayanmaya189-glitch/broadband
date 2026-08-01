//! E2E workflow tests. These are compiled into the `e2e` integration test
//! target via `tests/e2e.rs` and require Docker to run (ignored by default).

pub mod billing_workflow_test;
pub mod customer_lifecycle_test;
pub mod device_workflow_test;
pub mod network_workflow_test;
pub mod payment_gateway_test;
pub mod subscription_workflow_test;
pub mod ticket_workflow_test;

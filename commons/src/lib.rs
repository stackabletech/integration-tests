//!  Common functions used in the integration tests of the Stackable components
//!
//! ## Usage
//!
//! Add the dependency to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! integration-test-commons = { git = "https://github.com/stackabletech/integration-test-commons.git", tag = "0.1.0" }
//! ```
//!
//! Then `use` the prelude module in your test files:
//!
//! ```rust
//! use integration_test_commons::test::prelude::*;
//! ```
//!
//! ## Example
//!
//! The [`test::kube::TestKubeClient`] is used to interact with
//! Kubernetes. [`test::temporary_resource::TemporaryResource`] should
//! be used because it deletes the Kubernetes resource if it goes out of
//! scope which is also the case if a test case panics.
//!
//! ```rust
//! use integration_test_commons::test::prelude::*;
//!
//! #[test]
//! pub fn pod_should_be_started_successfully() {
//!     let client = TestKubeClient::new();
//!
//!     let pod = TemporaryResource::new(
//!         &client,
//!         "
//!             apiVersion: v1
//!             kind: Pod
//!             metadata:
//!               name: test
//!             ...
//!         ",
//!     );
//!
//!     client.verify_pod_condition(&pod, "Ready");
//! }
//! ```

pub mod operator;
pub mod test;

pub use stackable_operator;

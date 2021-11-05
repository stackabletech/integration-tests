//! Prelude for integration tests

pub use super::assertions::*;
pub use super::kube::*;
pub use super::repository::*;
pub use super::temporary_resource::TemporaryResource;

pub use indoc::{formatdoc, indoc};
pub use serde_json::json;
pub use spectral::prelude::*;
pub use stackable_operator::k8s_openapi::api::core::v1::*;

pub mod common;

use crate::common::opa::get_worker_nodes;
use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};
use integration_test_commons::operator::setup::version_label;

#[test]
fn test_create_cluster_0_27_1() -> Result<()> {
    let version = "0.27.1";

    let mut cluster = build_test_cluster();

    let replicas = get_worker_nodes(&cluster);

    let (opa_cr, expected_pod_count) = build_opa_cluster(cluster.name(), version, replicas)?;
    cluster.create_or_update(&opa_cr, &version_label(version), expected_pod_count)?;

    cluster.check_pod_version(version)
}

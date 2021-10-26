pub mod common;

use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};

#[test]
fn test_create_cluster_0_27_1() -> Result<()> {
    let version = "0.27.1";

    let mut cluster = build_test_cluster();

    let (opa_cr, expected_pod_count) = build_opa_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&opa_cr, expected_pod_count)?;

    Ok(())
}

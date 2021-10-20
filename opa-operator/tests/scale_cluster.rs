pub mod common;
use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = "0.27.1";
    let mut cluster = build_test_cluster();

    let (opa_cluster, expected_pod_count) = build_opa_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&opa_cluster, expected_pod_count)?;

    let (opa_cluster, expected_pod_count) = build_opa_cluster(cluster.name(), version, 2)?;
    cluster.create_or_update(&opa_cluster, expected_pod_count)?;

    Ok(())
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = "0.27.1";
    let mut cluster = build_test_cluster();

    let (opa_cluster, expected_pod_count) = build_opa_cluster(cluster.name(), version, 2)?;
    cluster.create_or_update(&opa_cluster, expected_pod_count)?;

    let (opa_cluster, expected_pod_count) = build_opa_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&opa_cluster, expected_pod_count)?;

    Ok(())
}

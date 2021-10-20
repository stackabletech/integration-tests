pub mod common;
use crate::common::nifi::maximize_client_verification_time_out;
use anyhow::Result;
use common::nifi::{build_nifi_cluster, build_test_cluster};

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = "1.13.2";
    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&nifi_cluster, expected_pod_count)?;

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(cluster.name(), version, 2)?;
    cluster.create_or_update(&nifi_cluster, expected_pod_count)?;

    Ok(())
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = "1.13.2";
    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(cluster.name(), version, 2)?;
    cluster.create_or_update(&nifi_cluster, expected_pod_count)?;

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&nifi_cluster, expected_pod_count)?;

    Ok(())
}

pub mod common;
use crate::common::checks::custom_monitoring_checks;
use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};
use integration_test_commons::test::prelude::Pod;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_opa_crd::APP_NAME;
    let version = "0.27.1";

    let mut cluster = build_test_cluster();

    let (opa_cr, expected_pod_count) = build_opa_cluster(cluster.name(), version, 1)?;

    cluster.create_or_update(&opa_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    // container names need to be lowercase; metrics port is ui port.
    let container_ports = vec![("metrics", 8181)];

    custom_monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
    )?;

    Ok(())
}

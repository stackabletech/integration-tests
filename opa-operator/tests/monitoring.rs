pub mod common;
use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};
use integration_test_commons::operator::checks::monitoring_checks;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;

// TODO: ignore once monitoring is using annotations / labels to expose metrics port.
//    Does not work with currently
#[test]
#[ignore]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_opa_crd::APP_NAME;
    let version = "0.27.1";
    let metrics_port: i32 = 8181;

    let mut cluster = build_test_cluster();

    let (opa_cr, expected_pod_count) = build_opa_cluster(cluster.name(), version, 1)?;

    cluster.create_or_update(
        &opa_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    cluster.check_pod_version(&version.to_string())?;
    let created_pods = cluster.list::<Pod>(None);

    // container names need to be lowercase; metrics port is ui port.
    let container_ports = vec![("metrics", metrics_port)];

    let admin_service =
        create_node_port_service(&cluster.client, "opa-metrics", "opa", metrics_port);

    monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
        &admin_service,
    )?;

    Ok(())
}

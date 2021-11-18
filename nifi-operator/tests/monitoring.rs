pub mod common;
use crate::common::checks::custom_monitoring_checks;
use crate::common::nifi::maximize_client_verification_time_out;
use anyhow::Result;
use common::nifi::{build_nifi_cluster_monitoring, build_test_cluster};
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;

#[test]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_nifi_crd::APP_NAME;
    let metrics_port: u16 = 9606;
    let version = "1.13.2";

    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cr, expected_pod_count) =
        build_nifi_cluster_monitoring(cluster.name(), version, 1, metrics_port)?;

    cluster.create_or_update(
        &nifi_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;
    let created_pods = cluster.list::<Pod>(None);

    // container names need to be lowercase
    let container_ports = vec![("metrics", metrics_port)];

    custom_monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
    )
}

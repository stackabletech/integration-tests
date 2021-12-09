pub mod common;

use anyhow::Result;
use common::nifi::maximize_client_verification_time_out;
use common::nifi::{build_nifi_cluster_monitoring, build_test_cluster};
use common::zookeeper::build_zk_test_cluster;
use integration_test_commons::operator::checks::monitoring_checks;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::Pod;
use std::collections::BTreeMap;

#[test]
#[ignore]
fn test_monitoring_and_container_ports() -> Result<()> {
    let container_name = stackable_nifi_crd::APP_NAME;
    let metrics_port: i32 = 9606;
    let version = "1.13.2";

    let zk_client = build_zk_test_cluster("test-kafka-zk")?;

    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cr, expected_pod_count) =
        build_nifi_cluster_monitoring(cluster.name(), version, 1, metrics_port, zk_client.name())?;

    cluster.create_or_update(&nifi_cr, &BTreeMap::new(), expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    // container names need to be lowercase
    let container_ports = vec![("metrics", metrics_port)];

    let http_service =
        create_node_port_service(&cluster.client, "nifi-metrics", "nifi", metrics_port);

    monitoring_checks(
        created_pods.as_slice(),
        container_ports.as_slice(),
        container_name,
        &http_service,
    )
}

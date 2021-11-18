pub mod common;

use anyhow::Result;
use common::nifi::{build_nifi_cluster, build_test_cluster, maximize_client_verification_time_out};
use integration_test_commons::operator::checks::port_check;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::stackable_operator::k8s_openapi::api::core::v1::Pod;

#[test]
fn test_create_1_server_1_13_2() -> Result<()> {
    let version = "1.13.2";
    let http_port: i32 = 28080;
    let protocol_port: i32 = 28010;
    let load_balance_port: i32 = 28020;

    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cr, expected_pod_count) = build_nifi_cluster(
        cluster.name(),
        version,
        1,
        http_port,
        protocol_port,
        load_balance_port,
    )?;
    cluster.create_or_update(
        &nifi_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let http_service = create_node_port_service(&cluster.client, "nifi-http", "nifi", http_port);
    let created_pods = cluster.list::<Pod>(None);
    port_check(&created_pods, &http_service)
}

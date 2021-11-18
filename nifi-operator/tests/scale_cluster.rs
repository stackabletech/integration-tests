pub mod common;

use crate::common::nifi::maximize_client_verification_time_out;
use anyhow::Result;
use common::nifi::{build_nifi_cluster, build_test_cluster};
use integration_test_commons::operator::checks::port_check;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::stackable_operator::k8s_openapi::api::core::v1::Pod;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = "1.13.2";
    let http_port: i32 = 29080;
    let protocol_port: i32 = 29010;
    let load_balance_port: i32 = 29020;

    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    build_nifi_cluster(
        cluster.name(),
        version,
        1,
        http_port,
        protocol_port,
        load_balance_port,
    )?;

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(
        cluster.name(),
        version,
        2,
        http_port,
        protocol_port,
        load_balance_port,
    )?;

    cluster.create_or_update(
        &nifi_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let http_service = create_node_port_service(&cluster.client, "nifi-http", "nifi", http_port);
    let created_pods = cluster.list::<Pod>(None);
    port_check(&created_pods, &http_service)
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = "1.13.2";
    let http_port: i32 = 30080;
    let protocol_port: i32 = 30010;
    let load_balance_port: i32 = 30020;

    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    build_nifi_cluster(
        cluster.name(),
        version,
        2,
        http_port,
        protocol_port,
        load_balance_port,
    )?;

    let (nifi_cluster, expected_pod_count) = build_nifi_cluster(
        cluster.name(),
        version,
        1,
        http_port,
        protocol_port,
        load_balance_port,
    )?;

    cluster.create_or_update(
        &nifi_cluster,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    let http_service = create_node_port_service(&cluster.client, "nifi-http", "nifi", http_port);
    let created_pods = cluster.list::<Pod>(None);
    port_check(&created_pods, &http_service)
}

pub mod common;

use anyhow::Result;
use common::spark::{build_spark_custom_resource, build_test_cluster};
use integration_test_commons::operator::checks::port_check;
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::stackable_operator::k8s_openapi::api::core::v1::Pod;
use stackable_spark_crd::commands::Restart;
use stackable_spark_crd::SparkVersion;

#[test]
fn test_restart_command() -> Result<()> {
    let command_name = "spark-restart-command";
    let command_kind = "Restart";
    let version = SparkVersion::v3_0_1;
    let http_port: i32 = 8080;

    let mut cluster = build_test_cluster();

    let (spark_cr, expected_pod_count) =
        build_spark_custom_resource(cluster.name(), &version, 1, 1, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    cluster.check_pod_version(&version.to_string())?;

    let command: Restart =
        common::spark::build_command(command_name, command_kind, cluster.name())?;
    let restart: Restart = cluster.apply_command(&command)?;

    cluster.wait_ready(&version_label(&version.to_string()), expected_pod_count)?;

    let http_service = create_node_port_service(&cluster.client, "spark-http", "spark", http_port);

    let created_pods = cluster.list::<Pod>(None);
    port_check(&created_pods, &http_service)?;

    cluster.check_pod_creation_timestamp(&restart.metadata.creation_timestamp)?;

    // TODO: Check if label done exists in command
    Ok(())
}

pub mod common;

use anyhow::Result;
use common::spark::{build_spark_custom_resource, build_test_cluster};
use integration_test_commons::operator::setup::version_label;
use stackable_spark_crd::commands::Restart;
use stackable_spark_crd::SparkVersion;

#[test]
fn test_restart_command() -> Result<()> {
    let command_name = "spark-restart-command";
    let command_kind = "Restart";
    let version = SparkVersion::v3_0_1;

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
    cluster.check_pod_creation_timestamp(&restart.metadata.creation_timestamp)?;

    // TODO: Check if label done exists in command
    Ok(())
}

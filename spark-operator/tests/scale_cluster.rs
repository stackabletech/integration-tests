pub mod common;

use anyhow::Result;
use common::spark::{build_spark_custom_resource, build_test_cluster};
use integration_test_commons::operator::setup::version_label;
use stackable_spark_crd::SparkVersion;

#[test]
fn test_scale_cluster_up() -> Result<()> {
    let version = SparkVersion::v3_0_1;
    let mut cluster = build_test_cluster();

    let (spark_cr, expected_pod_count_before) =
        build_spark_custom_resource(cluster.name(), &version, 1, 1, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count_before,
    )?;

    cluster.check_pod_version(&version.to_string())?;

    let (spark_cr, expected_pod_count_after) =
        build_spark_custom_resource(cluster.name(), &version, 2, 2, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count_after,
    )?;

    cluster.check_pod_version(&version.to_string())
}

#[test]
fn test_scale_cluster_down() -> Result<()> {
    let version = SparkVersion::v3_0_1;
    let mut cluster = build_test_cluster();

    let (spark_cr, expected_pod_count_before) =
        build_spark_custom_resource(cluster.name(), &version, 2, 2, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count_before,
    )?;

    cluster.check_pod_version(&version.to_string())?;

    let (spark_cr, expected_pod_count_after) =
        build_spark_custom_resource(cluster.name(), &version, 1, 1, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count_after,
    )?;

    cluster.check_pod_version(&version.to_string())
}

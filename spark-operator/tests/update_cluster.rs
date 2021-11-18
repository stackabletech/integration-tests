pub mod common;

use anyhow::Result;
use common::spark::{build_spark_custom_resource, build_test_cluster};
use integration_test_commons::operator::setup::version_label;
use stackable_spark_crd::SparkVersion;

#[test]
fn test_update_cluster() -> Result<()> {
    let version = SparkVersion::v3_0_1;
    let version_update = SparkVersion::v3_1_1;
    let mut cluster = build_test_cluster();

    let (spark_cr, expected_pod_count) =
        build_spark_custom_resource(cluster.name(), &version, 1, 2, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    cluster.check_pod_version(&version.to_string())?;

    let (spark_cr, expected_pod_count) =
        build_spark_custom_resource(cluster.name(), &version_update, 1, 2, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    cluster.check_pod_version(&version.to_string())
}

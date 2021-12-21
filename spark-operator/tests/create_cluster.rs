pub mod common;

use anyhow::Result;
use common::spark::{build_spark_custom_resource, build_test_cluster};
use integration_test_commons::operator::setup::version_label;

#[test]
fn test_create_cluster() -> Result<()> {
    let version = "3.0.1";
    let mut cluster = build_test_cluster();

    let (spark_cr, expected_pod_count) =
        build_spark_custom_resource(cluster.name(), version, 1, 1, 1)?;

    cluster.create_or_update(
        &spark_cr,
        &version_label(version),
        expected_pod_count,
    )?;

    cluster.check_pod_version(version)
}

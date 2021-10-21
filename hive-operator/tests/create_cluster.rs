pub mod common;

use anyhow::Result;
use common::hive::{build_hive_cluster, build_test_cluster};

#[test]
fn test_create_1_server_2_3_9() -> Result<()> {
    let version = "2.3.9";
    let mut cluster = build_test_cluster();

    let (hive_cr, expected_pod_count) = build_hive_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&hive_cr, expected_pod_count)?;

    // TODO descruct metadata_db in /tmp

    // TODO run the python script to check if the thing is running

    Ok(())
}

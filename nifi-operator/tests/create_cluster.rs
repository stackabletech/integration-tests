pub mod common;

use anyhow::Result;
use common::nifi::{build_nifi_cluster, build_test_cluster, maximize_client_verification_time_out};
use integration_test_commons::operator::setup::version_label;

#[test]
fn test_create_1_server_2_8_0() -> Result<()> {
    let version = "1.13.2";
    let mut cluster = build_test_cluster();
    maximize_client_verification_time_out(&mut cluster.client);

    let (nifi_cr, expected_pod_count) = build_nifi_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(
        &nifi_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )
}

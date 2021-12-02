pub mod common;

use crate::common::zookeeper::build_zk_test_cluster;
use anyhow::Result;
use common::druid::{build_druid_cluster, build_test_cluster, TestService};
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::Pod;
use std::{thread, time};

#[test]
fn test_create_1_cluster_0_22_0() -> Result<()> {
    let version = "0.22.0";

    let zk_client = build_zk_test_cluster("test-druid-zk")?;

    let mut cluster = build_test_cluster();

    let (druid_cr, expected_pod_count) =
        build_druid_cluster(cluster.name(), version, 1, zk_client.name())?;
    cluster.create_or_update(
        &druid_cr,
        &version_label(&version.to_string()),
        expected_pod_count,
    )?;

    // Wait for the cluster to have started fully
    // The pods are shown as running but are not able to respond to the healthcheck immediately
    let delay_time = time::Duration::from_secs(60);
    thread::sleep(delay_time);

    let created_pods = cluster.list::<Pod>(None);
    let actual_pod_count = created_pods.len();

    assert_eq!(
        actual_pod_count, expected_pod_count,
        "Expected different amount of pods"
    );

    // for each process/pod, create a NodePort service and check the health status
    let s1 = TestService::new(&cluster.client, "druid", "coordinator", 8081, 30081);
    let s2 = TestService::new(&cluster.client, "druid", "broker", 8082, 30082);
    let s3 = TestService::new(&cluster.client, "druid", "historical", 8083, 30083);
    let s4 = TestService::new(&cluster.client, "druid", "middleManager", 8091, 30091);
    let s5 = TestService::new(&cluster.client, "druid", "router", 8888, 30888);

    let delay_time = time::Duration::from_secs(3);
    thread::sleep(delay_time);

    s1.conduct_healthcheck(&cluster.client)?;
    s2.conduct_healthcheck(&cluster.client)?;
    s3.conduct_healthcheck(&cluster.client)?;
    s4.conduct_healthcheck(&cluster.client)?;
    s5.conduct_healthcheck(&cluster.client)?;

    Ok(())
}

pub mod common;

use anyhow::Result;
use common::druid::{build_druid_cluster, build_test_cluster};
use common::zookeeper::build_zk_test_cluster;
use integration_test_commons::operator::checks::wait_for_scan_port;
use integration_test_commons::operator::service::create_node_port_service_with_component;
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::test::prelude::{Pod, TestKubeClient};
use stackable_druid_crd::DruidRole;
use std::time::Duration;
use strum::IntoEnumIterator;

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

    // Check pod count
    let created_pods = cluster.list::<Pod>(None);
    let actual_pod_count = created_pods.len();
    assert_eq!(
        actual_pod_count, expected_pod_count,
        "Expected different amount of pods"
    );

    for role in DruidRole::iter() {
        health_check(&cluster.client, &role)?;
    }

    Ok(())
}

/// On every process, call the health check endpoint.
fn health_check(client: &TestKubeClient, role: &DruidRole) -> Result<()> {
    let role_string = role.to_string();
    let service_name = format!("{}-service", role_string);
    let service = create_node_port_service_with_component(
        &client,
        &service_name,
        "druid",
        &role_string,
        role.get_http_port().into(),
    );
    let service_pods = client.list_labeled(&format!(
        "app.kubernetes.io/name=druid,app.kubernetes.io/component={}",
        role_string
    )); // TODO select on instance
    for pod in service_pods {
        let address = &service.address(&pod);
        wait_for_scan_port(address, Duration::from_secs(150))?;
        let url = format!("http://{}/status/health", address);
        println!("Requesting [{}]", url);
        let res = reqwest::blocking::get(&url)?;
        let resp = res.text()?;
        println!("Response: {}", resp);
        assert_eq!(resp, "true", "Response from the healthcheck wasn't 'true'");
    }
    Ok(())
}

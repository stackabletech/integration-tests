pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster};
use anyhow::Result;
use common::service::{ServiceBuilder, ServiceType, TemporaryService};
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_cluster_update() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 9090;
    let client_port: i32 = 2182;
    let version = ZookeeperVersion::v3_5_8;
    let version_update = ZookeeperVersion::v3_7_0;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version,
        replicas,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);
    check_pod_version(
        &version.to_string(),
        created_pods.as_slice(),
        &cluster.labels.version,
    );

    let (zookeeper_cr, expected_pod_count) = build_zk_cluster(
        cluster.name(),
        &version_update,
        replicas,
        Some(admin_port),
        Some(client_port),
    )?;

    cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    let admin_service = TemporaryService::new(
        &cluster.client,
        &ServiceBuilder::new("zookeeper-admin")
            .with_port(admin_port, admin_port)
            .with_selector("app.kubernetes.io/name", "zookeeper")
            .with_type(ServiceType::NodePort)
            .build(),
    );

    custom_checks(
        &cluster.client,
        created_pods.as_slice(),
        &version_update,
        expected_pod_count,
        &admin_service,
    )?;

    check_pod_version(
        &version_update.to_string(),
        created_pods.as_slice(),
        &cluster.labels.version,
    );

    Ok(())
}

fn check_pod_version(version: &str, pods: &[Pod], version_label: &str) {
    for pod in pods {
        let pod_version = pod
            .metadata
            .labels
            .as_ref()
            .and_then(|labels| labels.get(version_label).cloned());
        assert_eq!(Some(version), pod_version.as_deref());
    }
}

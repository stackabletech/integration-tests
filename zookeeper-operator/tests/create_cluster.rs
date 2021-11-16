pub mod common;

use crate::common::checks::custom_checks;

use anyhow::Result;
use common::service::{ServiceBuilder, ServiceType, TemporaryService};
use common::zookeeper::{build_test_cluster, build_zk_cluster};
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

#[test]
fn test_create_cluster_3_5_8() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 8080;
    let version = ZookeeperVersion::v3_5_8;

    let mut cluster = build_test_cluster();

    let (zookeeper_cr, expected_pod_count) =
        build_zk_cluster(cluster.name(), &version, replicas, Some(admin_port), None)?;

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
        &version,
        expected_pod_count,
        &admin_service,
    )?;

    Ok(())
}

#[test]
fn test_create_cluster_3_7_0() -> Result<()> {
    let replicas = 3;
    let admin_port: i32 = 9090;
    let client_port: i32 = 2182;
    let version = ZookeeperVersion::v3_7_0;

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
        &version,
        expected_pod_count,
        &admin_service,
    )?;

    Ok(())
}

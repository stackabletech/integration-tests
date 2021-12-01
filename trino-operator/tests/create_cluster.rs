pub mod common;

use std::collections::BTreeMap;
use std::time::Duration;
use std::{env, thread};

use anyhow::Result;
use common::checks::custom_checks;
use common::hive::{build_hive_cluster, build_hive_test_cluster};
use common::s3::create_s3_test_storage;
use common::trino::{
    build_trino_cluster, build_trino_test_cluster, TRINO_COORDINATOR_HTTPS_PORT, TRINO_PASSWORD,
    TRINO_USERNAME,
};
use integration_test_commons::operator::service::{ServiceBuilder, ServiceType, TemporaryService};
use integration_test_commons::operator::setup::version_label;
use integration_test_commons::stackable_operator::labels::{APP_COMPONENT_LABEL, APP_NAME_LABEL};
use integration_test_commons::test::prelude::Pod;
use stackable_hive_crd::discovery::S3Connection;
use stackable_hive_crd::HiveVersion;
use stackable_trino_crd::{TrinoRole, TrinoVersion};

#[test]
fn test_create_cluster_362() -> Result<()> {
    test_create_cluster(TrinoVersion::v362, HiveVersion::v2_3_9)
}

fn test_create_cluster(trino_version: TrinoVersion, hive_version: HiveVersion) -> Result<()> {
    let trino_coordinator_replicas = 2;
    let trino_worker_replicas = 2;
    let hive_replicas = 2;

    let s3_endpoint = env::var("S3_ENDPOINT")?;
    let s3_access_key = env::var("S3_ACCESS_KEY")?;
    let s3_secret_key = env::var("S3_SECRET_KEY")?;

    let s3_connection = S3Connection {
        end_point: s3_endpoint.to_owned(),
        access_key: s3_access_key.to_owned(),
        secret_key: s3_secret_key.to_owned(),
        ssl_enabled: false,
        path_style_access: true,
    };

    create_s3_test_storage(&s3_endpoint, &s3_access_key, &s3_secret_key)?;

    let mut hive_cluster = build_hive_test_cluster();
    let (hive_cr, expected_hive_pods) = build_hive_cluster(
        hive_cluster.name(),
        hive_version.to_owned(),
        hive_replicas,
        Some(&s3_connection),
    )?;
    println!(
        "Hive cluster\n\
        ============\n\
        {}",
        serde_yaml::to_string(&hive_cr)?
    );

    hive_cluster.create_or_update(
        &hive_cr,
        &version_label(&hive_version.to_string()),
        expected_hive_pods,
    )?;

    let mut trino_cluster = build_trino_test_cluster();
    let nodes = trino_cluster.list_nodes(None);
    let node_addresses = nodes
        .iter()
        .filter_map(|node| node.status.as_ref())
        .filter_map(|node_status| node_status.addresses.as_ref())
        .flatten()
        .collect::<Vec<_>>();
    let (trino_cr, expected_trino_pods, pem_certificate) = build_trino_cluster(
        trino_cluster.name(),
        trino_version.to_owned(),
        trino_coordinator_replicas,
        trino_worker_replicas,
        hive_cluster.name(),
        Some(&s3_connection),
        &node_addresses,
    )?;
    println!(
        "Trino cluster\n\
        =============\n\
        {}",
        serde_yaml::to_string(&trino_cr)?
    );

    trino_cluster.create_or_update(
        &trino_cr,
        &version_label(&trino_version.to_string()),
        expected_trino_pods.into(),
    )?;

    // Wait for Trino authenticators to be loaded.
    thread::sleep(Duration::from_secs(10));

    let trino_service = TemporaryService::new(
        &trino_cluster.client,
        &ServiceBuilder::new("trino")
            .with_port(
                TRINO_COORDINATOR_HTTPS_PORT.into(),
                TRINO_COORDINATOR_HTTPS_PORT.into(),
            )
            .with_selector(APP_NAME_LABEL, stackable_trino_crd::APP_NAME)
            .with_selector(APP_COMPONENT_LABEL, &format!("{}", TrinoRole::Coordinator))
            .with_type(ServiceType::NodePort)
            .build(),
    );

    let mut coordinator_labels = BTreeMap::new();
    coordinator_labels.insert(
        String::from(APP_COMPONENT_LABEL),
        format!("{}", TrinoRole::Coordinator),
    );
    let coordinator_pods = trino_cluster.list::<Pod>(Some(coordinator_labels));

    trino_cluster.check_pod_version(&trino_version.to_string())?;

    custom_checks(
        &trino_service,
        &coordinator_pods,
        TRINO_USERNAME,
        TRINO_PASSWORD,
        &pem_certificate,
    )
}

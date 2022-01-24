pub mod common;

use anyhow::Result;
use common::{
    airflow::{
        build_airflow_cluster, build_airflow_credentials, build_command, build_test_cluster,
    },
    checks::custom_checks,
};
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::{Pod, Secret};
use stackable_airflow_crd::commands::Init;
use std::collections::BTreeMap;

#[test]
fn test_create_cluster_223() -> Result<()> {
    let version = "2.2.3";
    let replicas: usize = 1;
    let mut cluster = build_test_cluster();

    let secret_name = "simple-airflow-credentials";
    let admin_username = "airflow";
    let admin_password = "airflow";

    let airflow_secret = build_airflow_credentials(secret_name, admin_username, admin_password)?;
    cluster
        .client
        .apply::<Secret>(&serde_yaml::to_string(&airflow_secret)?);

    let airflow_cr = build_airflow_cluster(cluster.name(), version, replicas, secret_name)?;
    cluster.create_or_update(&airflow_cr, &BTreeMap::new(), replicas)?;
    let created_pods = cluster.list::<Pod>(None);

    let init: Init = build_command(
        "airflow-cluster-command-init",
        "Init",
        cluster.name(),
        secret_name,
    )?;
    cluster.apply_command(&init)?;

    cluster.client.verify_status(&init, |command| {
        command
            .status
            .as_ref()
            .and_then(|status| status.finished_at.as_ref())
            .is_some()
    });

    let admin_service =
        create_node_port_service(&cluster.client, "airflow-admin", "airflow", 8080);

    custom_checks(
        &created_pods,
        admin_username,
        admin_password,
        &admin_service,
    )
}

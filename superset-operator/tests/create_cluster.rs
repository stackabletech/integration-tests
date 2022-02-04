pub mod common;

use anyhow::Result;
use common::{
    checks::custom_checks,
    superset::{build_superset_cluster, build_superset_credentials, build_test_cluster},
};
use integration_test_commons::operator::service::create_node_port_service;
use integration_test_commons::test::prelude::{Pod, Secret};
use stackable_superset_crd::supersetdb::{SupersetDB, SupersetDBStatusCondition};
use std::collections::BTreeMap;

#[test]
fn test_create_cluster_1_3_2() -> Result<()> {
    let version = "1.3.2";
    let replicas: usize = 1;
    let mut cluster = build_test_cluster();

    let secret_name = "simple-superset-credentials";
    let admin_username = "admin";
    let admin_password = "admin";

    let superset_secret = build_superset_credentials(secret_name, admin_username, admin_password)?;
    cluster
        .client
        .apply::<Secret>(&serde_yaml::to_string(&superset_secret)?);

    let superset_cr = build_superset_cluster(cluster.name(), version, replicas, secret_name)?;
    cluster.create_or_update(&superset_cr, &BTreeMap::new(), replicas)?;
    let created_pods = cluster.list::<Pod>(None);

    let superset_db = cluster
        .client
        .find_namespaced::<SupersetDB>(cluster.name())
        .expect("Resource SupersetDB expected");

    cluster.client.verify_status(&superset_db, |superset_db| {
        superset_db.status.as_ref().map(|status| status.condition)
            == Some(SupersetDBStatusCondition::Ready)
    });

    let admin_service =
        create_node_port_service(&cluster.client, "superset-admin", "superset", 8088);

    custom_checks(
        &created_pods,
        admin_username,
        admin_password,
        &admin_service,
    )
}

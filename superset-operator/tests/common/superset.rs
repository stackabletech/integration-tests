use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::k8s_openapi::serde::de::DeserializeOwned;
use integration_test_commons::stackable_operator::k8s_openapi::serde::Serialize;
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use integration_test_commons::test::prelude::Secret;
use stackable_superset_crd::{SupersetCluster, SupersetVersion, APP_NAME};
use std::fmt::Debug;
use std::time::Duration;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<SupersetCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// This returns the secret with the Superset credentials.
pub fn build_superset_credentials(
    secret_name: &str,
    admin_username: &str,
    admin_password: &str,
) -> Result<Secret> {
    let spec = &formatdoc!(
        "
            apiVersion: v1
            kind: Secret
            metadata:
              name: {secret_name}
            type: superset.stackable.tech/superset-credentials
            stringData:
              adminUser.username: {admin_username}
              adminUser.firstname: Superset
              adminUser.lastname: Admin
              adminUser.email: admin@superset.com
              adminUser.password: {admin_password}
              connections.secretKey: thisISaSECRET_1234
              connections.sqlalchemyDatabaseUri: postgresql://superset:superset@superset-postgresql.default.svc.cluster.local/superset
        ",
        secret_name = secret_name,
        admin_username = admin_username,
        admin_password = admin_password,
    );

    Ok(serde_yaml::from_str(spec)?)
}

/// This returns a Superset custom resource.
pub fn build_superset_cluster(
    name: &str,
    version: &SupersetVersion,
    replicas: usize,
    secret_name: &str,
) -> Result<SupersetCluster> {
    let spec = &formatdoc!(
        "
            apiVersion: superset.stackable.tech/v1alpha1
            kind: SupersetCluster
            metadata:
              name: {name}
            spec:
              version: {version}
              nodes:
                roleGroups:
                  default:
                    config:
                      credentialsSecret: {secret_name}
                    replicas: {replicas}
        ",
        name = name,
        version = version.to_string(),
        secret_name = secret_name,
        replicas = replicas
    );

    Ok(serde_yaml::from_str(spec)?)
}

pub fn build_command<T>(
    name: &str,
    kind: &str,
    cluster_reference: &str,
    secret_name: &str,
) -> Result<T>
where
    T: Clone + Debug + DeserializeOwned + Serialize,
{
    let spec = format!(
        "
            apiVersion: command.superset.stackable.tech/v1alpha1
            kind: {kind}
            metadata:
              name: {name}
            spec:
              name: {cluster_reference}
              credentialsSecret: {secret_name}
              loadExamples: false
        ",
        kind = kind,
        name = name,
        cluster_reference = cluster_reference,
        secret_name = secret_name,
    );

    Ok(serde_yaml::from_str(&spec)?)
}

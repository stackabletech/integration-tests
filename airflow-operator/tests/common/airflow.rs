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
use stackable_airflow_crd::{AirflowCluster, APP_NAME};
use std::fmt::Debug;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<AirflowCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns the secret with the airflow credentials.
pub fn build_airflow_credentials(
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
            type: Opaque
            stringData:
              adminUser.username: {admin_username}
              adminUser.firstname: Airflow
              adminUser.lastname: Admin
              adminUser.email: airflow@airflow.com
              adminUser.password: {admin_password}
              connections.secretKey: thisISaSECRET_1234
              connections.sqlalchemyDatabaseUri: postgresql+psycopg2://airflow:airflow@airflow-postgresql.default.svc.cluster.local/airflow
              connections.celeryResultBackend: db+postgresql://airflow:airflow@airflow-postgresql.default.svc.cluster.local/airflow
              connections.celeryBrokerUrl: redis://:redis@airflow-redis-master:6379/0
        "
    );

    Ok(serde_yaml::from_str(spec)?)
}

/// This returns an airflow custom resource.
pub fn build_airflow_cluster(
    name: &str,
    version: &str,
    secret_name: &str,
) -> Result<AirflowCluster> {
    let spec = &formatdoc!(
        "
            apiVersion: airflow.stackable.tech/v1alpha1
            kind: AirflowCluster
            metadata:
              name: {name}
            spec:
              version: {version}
              executor: CeleryExecutor
              loadExamples: false
              exposeConfig: false
              webservers:
                roleGroups:
                  default:
                    config:
                      credentialsSecret: {secret_name}
              workers:
                roleGroups:
                  default:
                    config:
                      credentialsSecret: {secret_name}
              schedulers:
                roleGroups:
                  default:
                    config:
                      credentialsSecret: {secret_name}
        "
    );

    Ok(serde_yaml::from_str(spec)?)
}

/// This returns a command used to initialize backend components (e.g. database, queue)
/// required by the airflow cluster.
pub fn build_command<T>(
    name: &str,
    kind: &str,
    cluster_reference: &str,
    cluster_uid: &str,
    secret_name: &str,
) -> Result<T>
where
    T: Clone + Debug + DeserializeOwned + Serialize,
{
    let spec = &formatdoc!(
        "
            apiVersion: command.airflow.stackable.tech/v1alpha1
            kind: {kind}
            metadata:
              name: {name}
              ownerReferences:
                - apiVersion: airflow.stackable.tech/v1alpha1
                  kind: AirflowCluster
                  name: {cluster_reference}
                  uid: {cluster_uid}
            spec:
              clusterRef:
                name: {cluster_reference}
              credentialsSecret: {secret_name}
        "
    );

    Ok(serde_yaml::from_str(&spec)?)
}

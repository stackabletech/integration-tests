use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use integration_test_commons::test::prelude::Secret;
use stackable_superset_crd::{SupersetCluster, APP_NAME};

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<SupersetCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns the secret with the Superset credentials.
pub fn build_superset_credentials(
    secret_name: &str,
    admin_username: &str,
    admin_password: &str,
) -> Result<Secret> {
    let spec = formatdoc!(
        "
        apiVersion: v1
        kind: Secret
        metadata:
          name: {secret_name}
        type: Opaque
        stringData:
          adminUser.username: {admin_username}
          adminUser.firstname: Superset
          adminUser.lastname: Admin
          adminUser.email: admin@superset.com
          adminUser.password: {admin_password}
          connections.secretKey: thisISaSECRET_1234
          connections.sqlalchemyDatabaseUri: postgresql://superset:superset@superset-postgresql.default.svc.cluster.local/superset
        "
    );

    Ok(serde_yaml::from_str(&spec)?)
}

/// This returns a Superset custom resource.
pub fn build_superset_cluster(
    name: &str,
    version: &str,
    replicas: usize,
    secret_name: &str,
) -> Result<SupersetCluster> {
    let spec = formatdoc!(
        "
        apiVersion: superset.stackable.tech/v1alpha1
        kind: SupersetCluster
        metadata:
          name: {name}
        spec:
          version: {version}
          credentialsSecret: {secret_name}
          loadExamplesOnInit: false
          nodes:
            roleGroups:
              default:
                config: {{}}
                replicas: {replicas}
        "
    );

    Ok(serde_yaml::from_str(&spec)?)
}

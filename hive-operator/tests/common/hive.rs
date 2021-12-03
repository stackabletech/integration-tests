use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_hive_crd::{HiveCluster, APP_NAME};

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<HiveCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns a Hive custom resource and the expected pod count.
pub fn build_hive_cluster(
    name: &str,
    version: &str,
    replicas: usize,
) -> Result<(HiveCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: hive.stackable.tech/v1alpha1
        kind: HiveCluster
        metadata:
          name: {}
        spec:
          version: {}
          metastore:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  metricsPort: 11111
                  database:
                    connString: jdbc:derby:;databaseName=/stackable/data/metadata_db;create=true
                    user: APP
                    password: mine
                    dbType: derby

        ",
        name, version, replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_druid_crd::{DruidCluster, APP_NAME};

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<DruidCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns a Druid custom resource and the expected pod count.
pub fn build_druid_cluster(
    name: &str,
    version: &str,
    replicas: usize,
    zk_ref_name: &str,
) -> Result<(DruidCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: druid.stackable.tech/v1alpha1
        kind: DruidCluster
        metadata:
          name: {name}
        spec:
          version: {version}
          zookeeperReference:
            namespace: default
            configMapName: {zk_ref_name}
          metadataStorageDatabase:
            dbType: derby
            connString: jdbc:derby://localhost:1527/var/druid/metadata.db;create=true
            host: localhost
            port: 1527
          deepStorage:
            storageType: local
            storageDirectory: /data
          brokers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config: {{}}
                replicas: {replicas}
          coordinators:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config: {{}}
                replicas: {replicas}
          historicals:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config: {{}}
                replicas: {replicas}
          middleManagers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config: {{}}
                replicas: {replicas}
          routers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config: {{}}
                replicas: {replicas}
        ",
        name = name,
        version = version,
        zk_ref_name = zk_ref_name,
        replicas = replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas * 5))
}

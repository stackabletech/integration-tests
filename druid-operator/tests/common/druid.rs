use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use stackable_druid_crd::{DruidCluster, APP_NAME};
use std::time::Duration;

const APP_NAME_LABEL: &str = "app.kubernetes.io/name";
const APP_INSTANCE_LABEL: &str = "app.kubernetes.io/instance";
const APP_VERSION_LABEL: &str = "app.kubernetes.io/version";

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<DruidCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        // TODO: the app, instance and version labels should be recovered from kube-rs / k8s-openapi
        //    independent crate in operator-rs
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(180),
        },
    )
}

/// This returns a Druid custom resource and the expected pod count.
pub fn build_druid_cluster(
    name: &str,
    version: &str,
    replicas: usize,
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
            name: simple
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
                config:
                  plaintextPort: 8082
                replicas: {replicas}
          coordinators:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8081
                replicas: {replicas}
          historicals:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8083
                replicas: {replicas}
          middleManagers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8091
                replicas: {replicas}
          routers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8888
                replicas: {replicas}
        ",
        name=name, version=version, replicas=replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas * 5))
}

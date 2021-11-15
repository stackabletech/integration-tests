use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use stackable_spark_crd::{SparkCluster, SparkVersion};
use std::fmt::Debug;
use std::time::Duration;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<SparkCluster> {
    TestCluster::new(
        &TestClusterOptions::new("spark", "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// This returns a SparkCluster custom resource and the expected pod count.
pub fn build_spark_custom_resource(
    name: &str,
    version: &SparkVersion,
    masters: usize,
    workers: usize,
    history_servers: usize,
) -> Result<(SparkCluster, usize)> {
    let spec = format!(
        "
        apiVersion: spark.stackable.tech/v1alpha1
        kind: SparkCluster
        metadata:
          name: {}
        spec:
          version: {}
          config:
            logDir: file:///tmp
          masters:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {}
          workers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {}
          historyServers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {}
                config:
                  historyWebUiPort: 10000 
    ",
        name,
        version.to_string(),
        masters,
        workers,
        history_servers
    );

    Ok((
        serde_yaml::from_str(&spec)?,
        masters + workers + history_servers,
    ))
}

pub fn build_command<T>(name: &str, kind: &str, cluster_reference: &str) -> Result<T>
where
    T: Clone + Debug + DeserializeOwned + Serialize,
{
    let spec = format!(
        "
        apiVersion: command.spark.stackable.tech/v1alpha1
        kind: {}
        metadata:
          name: {}
        spec:
          name: {}
    ",
        kind, name, cluster_reference
    );

    Ok(serde_yaml::from_str(&spec)?)
}

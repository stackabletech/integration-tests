use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_spark_crd::SparkCluster;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<SparkCluster> {
    TestCluster::new(
        &TestClusterOptions::new("spark", "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns a SparkCluster custom resource and the expected pod count.
pub fn build_spark_custom_resource(
    name: &str,
    version: &str,
    masters: usize,
    workers: usize,
    history_servers: usize,
) -> Result<(SparkCluster, usize)> {
    let spec = format!(
        "
apiVersion: spark.stackable.tech/v1alpha1
kind: SparkCluster
metadata:
  name: {name}
spec:
  version: {version}
  config:
    enableMonitoring: true
  masters:
    roleGroups:
      default:
        selector:
          matchLabels:
            kubernetes.io/os: linux
        replicas: {masters}
  workers:
    roleGroups:
      default:
        selector:
          matchLabels:
            kubernetes.io/os: linux
        replicas: {workers} 
  historyServers:
    roleGroups:
      default:
        selector:
          matchLabels:
            kubernetes.io/os: linux
        replicas: {history_servers}
    "
    );

    Ok((
        serde_yaml::from_str(&spec)?,
        masters + workers + history_servers,
    ))
}

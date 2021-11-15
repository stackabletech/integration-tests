use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_kafka_crd::{KafkaCluster, APP_NAME};
use std::time::Duration;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<KafkaCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// This returns a Kafka custom resource and the expected pod count.
pub fn build_kafka_cluster(
    name: &str,
    version: &str,
    replicas: usize,
) -> Result<(KafkaCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: kafka.stackable.tech/v1alpha1
        kind: KafkaCluster
        metadata:
          name: {}
        spec:
          version:
            kafka_version: {}
          zookeeperReference:
            namespace: default
            name: simple
          brokers:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  logDirs: /tmp/kafka-logs
                  metricsPort: 9606
    ",
        name, version, replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

/// This returns a Kafka custom resource and the expected pod count with monitoring enabled.
pub fn build_kafka_cluster_monitoring(
    name: &str,
    version: &str,
    replicas: usize,
    monitoring_port: u16,
) -> Result<(KafkaCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: kafka.stackable.tech/v1alpha1
        kind: KafkaCluster
        metadata:
          name: {}
        spec:
          version:
            kafka_version: {}
          zookeeperReference:
            namespace: default
            name: simple
          brokers:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  logDirs: /tmp/kafka-logs
                  metricsPort: {}
    ",
        name, version, replicas, monitoring_port
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

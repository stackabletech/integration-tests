use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_kafka_crd::{KafkaCluster, APP_NAME};

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<KafkaCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns a Kafka custom resource and the expected pod count.
pub fn build_kafka_cluster(
    name: &str,
    version: &str,
    replicas: usize,
    zk_ref_name: &str,
) -> Result<(KafkaCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: kafka.stackable.tech/v1alpha1
        kind: KafkaCluster
        metadata:
          name: {}
        spec:
          version: {}
          zookeeperConfigMapName: {}
          brokers:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
    ",
        name, version, zk_ref_name, replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

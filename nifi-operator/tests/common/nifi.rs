use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};

use integration_test_commons::test::kube::TestKubeClient;
use stackable_nifi_crd::{NifiCluster, APP_NAME};
use std::time::Duration;

// Kube defined watcher timeout < 295:
// https://github.com/kube-rs/kube-rs/blob/0.59.0/kube-core/src/params.rs#L69-L73
// https://github.com/kubernetes/kubernetes/issues/6513
const MAX_VERIFY_STATUS_TIMEOUT: u64 = 294;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<NifiCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
            pods_terminated_delay: None,
        },
    )
}

/// This is required to adapt in every NiFi test method. The NiFi package is very big (~1.2GB) and
/// to download and unpack we require a bigger timeout.
pub fn maximize_client_verification_time_out(client: &mut TestKubeClient) {
    client.timeouts().verify_status = Duration::from_secs(MAX_VERIFY_STATUS_TIMEOUT);
}

/// This returns a NiFi custom resource and the expected pod count.
pub fn build_nifi_cluster(
    name: &str,
    version: &str,
    replicas: usize,
    http_port: i32,
    protocol_port: i32,
    load_balance_port: i32,
    zk_ref_name: &str,
) -> Result<(NifiCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: nifi.stackable.tech/v1alpha1
        kind: NifiCluster
        metadata:
          name: {}
        spec:
          version: {}
          zookeeperReference:
            name: {}
            namespace: default
            chroot: /nifi
          nodes:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  httpPort: {}
                  protocolPort: {}
                  loadBalancePort: {}
    ",
        name, version, zk_ref_name, replicas, http_port, protocol_port, load_balance_port
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

/// This returns a NiFi custom resource and the expected pod count with monitoring enabled.
pub fn build_nifi_cluster_monitoring(
    name: &str,
    version: &str,
    replicas: usize,
    monitoring_port: i32,
    zk_ref_name: &str,
) -> Result<(NifiCluster, usize)> {
    let spec = &format!(
        "
        apiVersion: nifi.stackable.tech/v1alpha1
        kind: NifiCluster
        metadata:
          name: {}
        spec:
          version: {}
          metricsPort: {}
          zookeeperReference:
            name: {}
            namespace: default
            chroot: /nifi
          nodes:
            roleGroups:
              default:
                selector:
                  kubernetes.io/os: linux
                replicas: {}
                config:
                  httpPort: 11080
                  protocolPort: 11443
                  loadBalancePort: 11342
    ",
        name, version, monitoring_port, zk_ref_name, replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

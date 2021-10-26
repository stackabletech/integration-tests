use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::test::kube::TestKubeClient;
use stackable_nifi_crd::{NifiCluster, APP_NAME};
use std::time::Duration;

// Kube defined watcher timeout < 295:
// https://github.com/kube-rs/kube-rs/blob/0.59.0/kube-core/src/params.rs#L69-L73
// https://github.com/kubernetes/kubernetes/issues/6513
const MAX_VERIFY_STATUS_TIMEOUT: u64 = 294;

const APP_NAME_LABEL: &str = "app.kubernetes.io/name";
const APP_INSTANCE_LABEL: &str = "app.kubernetes.io/instance";
const APP_VERSION_LABEL: &str = "app.kubernetes.io/version";

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<NifiCluster> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple"),
        // TODO: the app, instance and version labels should be recovered from kube-rs / k8s-openapi
        //    independent crate in operator-rs
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
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
            name: simple
            namespace: default
            chroot: /nifi
          nodes:
            roleGroups:
              default:
                selector:
                  kubernetes.io/arch: stackable-linux
                replicas: {}
                config:
                  httpPort: 10000
                  protocolPort: 10443
                  loadBalancePort: 6342
    ",
        name, version, replicas,
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

/// This returns a NiFi custom resource and the expected pod count with monitoring enabled.
pub fn build_nifi_cluster_monitoring(
    name: &str,
    version: &str,
    replicas: usize,
    monitoring_port: u16,
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
            name: simple
            namespace: default
            chroot: /nifi
          nodes:
            roleGroups:
              default:
                selector:
                  kubernetes.io/arch: stackable-linux
                replicas: {}
                config:
                  httpPort: 10000
                  protocolPort: 10443
                  loadBalancePort: 6342
    ",
        name, version, monitoring_port, replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_monitoring_crd::{MonitoringCluster, APP_NAME};
use std::time::Duration;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<MonitoringCluster> {
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

/// This returns a Monitoring custom resource and the expected pod count.
pub fn build_monitoring_cluster(
    name: &str,
    version: &str,
    available_nodes: usize,
    aggregator_port: u16,
    node_exporter_port: u16,
    federation_port: u16,
) -> Result<(MonitoringCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: monitoring.stackable.tech/v1alpha1
        kind: MonitoringCluster
        metadata:
          name: {}
        spec:
          version: {}
          podAggregator:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  webUiPort: {}
                  scrapeInterval: 15s
                  scrapeTimeout: 5s
                  evaluationInterval: 10s
                  scheme: http
                  cliArgs:
                    - --storage.tsdb.path={{{{configroot}}}}/aggregator_data/
                    - --storage.tsdb.retention.time=7d
          nodeExporter:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  metricsPort: {}
                  cliArgs:
                    - --no-collector.wifi
                    - --no-collector.hwmon
                    - --collector.filesystem.ignored-mount-points=^/(dev|proc|sys|var/lib/docker/.+|var/lib/kubelet/pods/.+)($|/)
                    - --collector.netclass.ignored-devices=^(veth.*|[a-f0-9]{{15}})$
                    - --collector.netdev.device-exclude=^(veth.*|[a-f0-9]{{15}})$
          federation:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: 1
                config:
                  webUiPort: {}
                  scrapeInterval: 15s
                  scrapeTimeout: 5s
                  evaluationInterval: 10s
                  scheme: http
                  cliArgs:
                    - --storage.tsdb.path={{{{configroot}}}}/federation_data/
                    - --storage.tsdb.retention.time=30d
    ",
        name, version, aggregator_port, node_exporter_port, federation_port
    );

    // We have one pod aggregator and one node exporter on each node, and one federation overall.
    let calculated_replicas = 2 * available_nodes + 1;
    Ok((serde_yaml::from_str(spec)?, calculated_replicas))
}

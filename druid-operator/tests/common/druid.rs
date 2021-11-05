use anyhow::Result;
use integration_test_commons::{
    operator::setup::{TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts},
    test::prelude::{Pod, Service, TemporaryResource, TestKubeClient},
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
        name = name,
        version = version,
        replicas = replicas
    );

    Ok((serde_yaml::from_str(spec)?, replicas * 5))
}

/// This is a helper service to expose the ports of the processes in the cluster as node ports,
/// so we can access them from outside of kubernetes to check the /status/health endpoint.
pub struct TestService<'a> {
    service: TemporaryResource<'a, Service>,
    node_port: u16,
}

impl<'a> TestService<'a> {
    pub fn new(
        client: &'a TestKubeClient,
        name: &str,
        component: &str,
        pod_port: u16,
        node_port: u16,
    ) -> Self {
        TestService {
            service: TemporaryResource::new(
                client,
                &format!(
                    "
                    apiVersion: v1
                    kind: Service
                    metadata:
                      name: {svc_name}
                    spec:
                      type: NodePort
                      selector:
                        app.kubernetes.io/name: {name}
                        app.kubernetes.io/component: {component}
                      ports:
                        - port: {pod_port}
                          targetPort: {pod_port}
                          nodePort: {node_port}
                    ",
                    svc_name = format!(
                        "{}-{}",
                        name.to_ascii_lowercase(),
                        component.to_ascii_lowercase()
                    ),
                    name = name,
                    component = component,
                    pod_port = pod_port,
                    node_port = node_port
                ),
            ),
            node_port,
        }
    }

    /// For the defined service, find all applicable pods and check their health.
    pub fn conduct_healthcheck(&self, client: &'a TestKubeClient) -> Result<()> {
        let mut selectors = vec![];
        let selector_map = self
            .service
            .spec
            .as_ref()
            .unwrap()
            .selector
            .as_ref()
            .unwrap();
        for (k, v) in selector_map {
            selectors.push(format!("{}={}", k, v));
        }
        let selector = selectors.join(",");
        let pods = client.list_labeled::<Pod>(&selector);
        for p in pods {
            let host_ip = p.status.unwrap().host_ip.unwrap();
            let url = format!("http://{}:{}/status/health", host_ip, self.node_port);
            println!("Requesting [{}]", url);
            let res = reqwest::blocking::get(&url)?;
            let resp = res.text()?;
            println!("Response: {}", resp);
            assert_eq!(resp, "true", "Response from the healthcheck wasn't 'true'");
        }
        Ok(())
    }
}
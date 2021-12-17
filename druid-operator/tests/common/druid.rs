use anyhow::Result;
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use integration_test_commons::{
    operator::setup::{TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts},
    test::prelude::{Pod, Service, TemporaryResource, TestKubeClient},
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
                config:
                  plaintextPort: 8082
                  metricsPort: 9095
                replicas: {replicas}
          coordinators:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8081
                  metricsPort: 9090
                replicas: {replicas}
          historicals:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8083
                  metricsPort: 9091
                replicas: {replicas}
          middleManagers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8091
                  metricsPort: 9098
                replicas: {replicas}
          routers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  plaintextPort: 8888
                  metricsPort: 9195
                replicas: {replicas}
        ",
        name = name,
        version = version,
        zk_ref_name = zk_ref_name,
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

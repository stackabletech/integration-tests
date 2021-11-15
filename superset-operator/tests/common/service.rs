use indoc::indoc;
use integration_test_commons::test::prelude::{Pod, Service, TemporaryResource, TestKubeClient};
use std::{thread::sleep, time::Duration};

pub struct SupersetService<'a> {
    _service: TemporaryResource<'a, Service>,
    node_port: i32,
}

impl<'a> SupersetService<'a> {
    pub fn new(client: &'a TestKubeClient) -> Self {
        let mut service: TemporaryResource<'a, Service> = TemporaryResource::new(
            client,
            indoc!(
                "
                    apiVersion: v1
                    kind: Service
                    metadata:
                      name: superset
                    spec:
                      type: NodePort
                      selector:
                        app.kubernetes.io/name: superset
                      ports:
                        - port: 8088
                          targetPort: 8088
                "
            ),
        );

        service.update();

        // Wait until the port is opened
        sleep(Duration::from_secs(2));

        let node_port = node_port(&service);

        SupersetService {
            _service: service,
            node_port,
        }
    }

    pub fn address(&self, pod: &Pod) -> String {
        format!("{}:{}", host_ip(pod), self.node_port)
    }
}

fn node_port(service: &Service) -> i32 {
    service
        .spec
        .as_ref()
        .and_then(|spec| spec.ports.as_ref())
        .and_then(|ports| ports.first())
        .and_then(|port| port.node_port.as_ref())
        .unwrap_or_else(|| {
            panic!(
                "nodePort should be set on service [{}].",
                service.metadata.name.as_ref().unwrap()
            )
        })
        .to_owned()
}

fn host_ip(pod: &Pod) -> String {
    pod.status
        .as_ref()
        .and_then(|status| status.host_ip.as_ref())
        .unwrap_or_else(|| {
            panic!(
                "hostIp should be set on pod [{}].",
                pod.metadata.name.as_ref().unwrap()
            )
        })
        .to_owned()
}

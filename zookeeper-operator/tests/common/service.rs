use integration_test_commons::stackable_operator::k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use integration_test_commons::stackable_operator::kube::core::ObjectMeta;
use integration_test_commons::test::prelude::{
    Pod, Service, ServicePort, ServiceSpec, TemporaryResource, TestKubeClient,
};
use std::collections::BTreeMap;
use std::{thread::sleep, time::Duration};
use strum_macros::Display;

#[derive(Clone, Display)]
pub enum ServiceType {
    /// Exposes the Service on a cluster-internal IP. Choosing this value makes the Service only
    /// reachable from within the cluster. This is the default ServiceType.
    ClusterIP,
    /// Exposes the Service on each Node's IP at a static port (the NodePort). A ClusterIP Service,
    /// to which the NodePort Service routes, is automatically created. You'll be able to contact
    /// the NodePort Service, from outside the cluster, by requesting <NodeIP>:<NodePort>.
    NodePort,
    /// Exposes the Service externally using a cloud provider's load balancer. NodePort and ClusterIP
    /// Services, to which the external load balancer routes, are automatically created.
    LoadBalancer,
    /// Maps the Service to the contents of the externalName field (e.g. foo.bar.example.com), by
    /// returning a CNAME record with its value. No proxying of any kind is set up.
    ExternalName,
}

/// A builder to build [`Service`] objects.
#[derive(Clone, Default)]
pub struct ServiceBuilder {
    name: String,
    ports: Vec<ServicePort>,
    selector: BTreeMap<String, String>,
    service_type: Option<ServiceType>,
}

impl ServiceBuilder {
    pub fn new(name: &str) -> ServiceBuilder {
        ServiceBuilder {
            name: name.to_string(),
            ..ServiceBuilder::default()
        }
    }

    pub fn with_port(&mut self, port: i32, target_port: i32) -> &mut Self {
        self.ports.push(ServicePort {
            port,
            target_port: Some(IntOrString::Int(target_port)),
            ..ServicePort::default()
        });
        self
    }

    pub fn with_ports(&mut self, service_ports: Vec<ServicePort>) -> &mut Self {
        self.ports.extend(service_ports);
        self
    }

    pub fn with_selector(&mut self, key: &str, value: &str) -> &mut Self {
        self.selector.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_selectors(&mut self, selectors: BTreeMap<String, String>) -> &mut Self {
        self.selector.extend(selectors);
        self
    }

    pub fn with_type(&mut self, service_type: ServiceType) -> &mut Self {
        self.service_type = Some(service_type);
        self
    }

    pub fn build(&mut self) -> Service {
        Service {
            metadata: ObjectMeta {
                name: Some(self.name.clone()),
                ..ObjectMeta::default()
            },
            spec: Some(ServiceSpec {
                ports: Some(self.ports.clone()),
                selector: Some(self.selector.clone()),
                type_: self.service_type.as_ref().map(|t| t.to_string()),
                ..ServiceSpec::default()
            }),
            ..Service::default()
        }
    }
}

pub struct TemporaryService<'a> {
    _service: TemporaryResource<'a, Service>,
    node_port: i32,
}

impl<'a> TemporaryService<'a> {
    pub fn new(client: &'a TestKubeClient, service: &Service) -> Self {
        let mut service: TemporaryResource<'a, Service> =
            TemporaryResource::new(client, &serde_yaml::to_string(service).unwrap());

        service.update();

        // Wait until the port is opened
        sleep(Duration::from_secs(2));

        let node_port = node_port(&service);

        TemporaryService {
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

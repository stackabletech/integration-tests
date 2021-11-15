use anyhow::{anyhow, Result};
use integration_test_commons::{operator::checks, test::prelude::Pod};

/// Collect and gather all checks with regard to metrics and container ports.
pub fn custom_monitoring_checks(
    pods: &[Pod],
    container_ports: &[(&str, u16)],
    container_name: &str,
) -> Result<()> {
    for pod in pods {
        // only check if container ports are specified
        if container_ports.is_empty() {
            check_container_ports(pod, container_ports, container_name)?;
            check_metrics_port_open(pod, container_name)?;
        }
    }
    Ok(())
}

/// Check if container ports with given name and port number are set in the pod.
pub fn check_container_ports(
    pod: &Pod,
    container_ports: &[(&str, u16)],
    container_name: &str,
) -> Result<()> {
    let port_count = pod
        .spec
        .as_ref()
        .and_then(|pod| {
            pod.containers
                .iter()
                .find(|container| container.name == container_name)
                .cloned()
        })
        .and_then(|container| container.ports)
        .map_or(0usize, |ports| {
            let mut found: usize = 0;
            for port in &ports {
                for (name, number) in container_ports {
                    if port.name == Some(name.to_string())
                        && port.container_port == i32::from(*number)
                    {
                        found += 1;
                    }
                }
            }
            found
        });

    return if port_count == container_ports.len() {
        Ok(())
    } else {
        Err(anyhow!("Required container_ports in container [{}] do not match the specified pod container ports. Required [{}] vs provided [{}]",
        container_name, container_ports.len(), port_count))
    };
}

pub fn check_metrics_port_open(pod: &Pod, container_name: &str) -> Result<()> {
    let container_port_name = "metrics";
    // extract hostname from port
    let node_name = match &pod.spec.as_ref().unwrap().node_name {
        None => {
            return Err(anyhow!(
                "Missing node_name in pod [{}]. Cannot create host address for metrics port check!",
                pod.metadata.name.as_ref().unwrap(),
            ))
        }
        Some(name) => name,
    };

    // extract metrics port from container_port
    let port = match pod.spec.as_ref().and_then(|pod| {
        pod.containers
            .iter()
            .find(|container| container.name == container_name)
            .cloned()
    }) {
        None => {
            return Err(anyhow!(
            "Missing container [{}] in pod [{}]. Cannot extract host port for metrics port check!",
            container_name,
            pod.metadata.name.as_ref().unwrap(),
        ))
        }
        Some(container) => {
            match container.ports.and_then(|ports| ports.iter().find(|port| port.name == Some(container_port_name.to_string())).cloned()) {
                None => { return Err(anyhow!(
                "Missing container_port [{}] in pod [{}]. Cannot extract host port for metrics port check!",
                container_port_name, pod.metadata.name.as_ref().unwrap(),
            ))}
                Some(container_port) => {
                    container_port.container_port
                }
            }
        }
    };

    checks::scan_port(&format!("{}:{}", node_name, port))
}

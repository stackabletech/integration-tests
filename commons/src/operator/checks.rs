use crate::operator::service::TemporaryService;
use crate::stackable_operator::k8s_openapi::api::core::v1::Pod;
use anyhow::{anyhow, Result};
use std::net::TcpStream;

/// Scan port of an address.
pub fn scan_port(address: &str) -> Result<()> {
    match TcpStream::connect(address) {
        Ok(_) => {
            println!("PortCheck for [{}] successful", address);
            Ok(())
        }
        Err(err) => Err(anyhow!(
            "TCP error occurred when connecting to [{}]: {}",
            address,
            err.to_string()
        )),
    }
}

/// Collect and gather all checks with regard to metrics and container ports.
pub fn monitoring_checks(
    pods: &[Pod],
    container_ports: &[(&str, i32)],
    container_name: &str,
    service: &TemporaryService,
) -> Result<()> {
    for pod in pods {
        let address = &service.address(pod);
        scan_port(address)?;
        check_container_ports(pod, container_ports, container_name)?;
    }
    Ok(())
}

/// Check if container ports with given name and port number are set in the pod.
pub fn check_container_ports(
    pod: &Pod,
    container_ports: &[(&str, i32)],
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
                    if port.name == Some(name.to_string()) && port.container_port == *number {
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

use crate::common::four_letter_commands::send_4lw_i_am_ok;
use anyhow::{anyhow, Result};
use integration_test_commons::operator::checks;
use integration_test_commons::stackable_operator::configmap::CONFIGMAP_TYPE_LABEL;
use integration_test_commons::test::kube::TestKubeClient;
use integration_test_commons::test::prelude::{ConfigMap, ConfigMapVolumeSource, Pod};
use stackable_zookeeper_crd::ZookeeperVersion;

/// Collect and gather all checks that may be performed on ZooKeeper server pods.
pub fn custom_checks(
    client: &TestKubeClient,
    pods: &[Pod],
    version: &ZookeeperVersion,
    client_port: u16,
    expected_pod_count: usize,
) -> Result<()> {
    for pod in pods {
        send_4lw_i_am_ok(pod, version, client_port)?;
        check_config_map(client, pod, expected_pod_count)?;
    }
    Ok(())
}

/// Collect and gather all checks with regard to metrics and container ports.
pub fn custom_monitoring_checks(
    pods: &[Pod],
    container_ports: &[(&str, u16)],
    container_name: &str,
) -> Result<()> {
    for pod in pods {
        check_container_ports(pod, container_ports, container_name)?;
        check_metrics_port_open(pod, container_name)?;
    }
    Ok(())
}

/// Perform checks on configmaps for:
/// - server.<id> property set correctly (especially with scale up / down)
pub fn check_config_map(
    client: &TestKubeClient,
    pod: &Pod,
    expected_server_count: usize,
) -> Result<()> {
    let config_cm_name = get_config_cm(client, pod, CONFIGMAP_TYPE_LABEL)?;
    let config_map: Option<ConfigMap> = client.find_namespaced(&config_cm_name);

    check_for_server_id_property_count(config_map, expected_server_count)
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
            .find(|container| container.name == container_name).cloned()
    }) {
        None => {
            return Err(anyhow!(
                "Missing container [{}] in pod [{}]. Cannot create extract host port for metrics port check!",
                container_name, pod.metadata.name.as_ref().unwrap(),
            ))
        }
        Some(container) => {
            match container.ports.and_then(|ports| ports.iter().find(|port| port.name == Some(container_port_name.to_string())).cloned()) {
                None => { return Err(anyhow!(
                "Missing container_port [{}] in pod [{}]. Cannot create extract host port for metrics port check!",
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

/// This is a simple check for the correctness of the server property in config maps.
/// Every known server will be registered like:
/// server.1 = some_url
/// server.2 = another_url
/// If pods crash or scaling appears we have to make sure that the config maps
/// and pods are updated / restarted in order to contain the correct state of the cluster.
fn check_for_server_id_property_count(
    cm: Option<ConfigMap>,
    expected_server_count: usize,
) -> Result<()> {
    let mut server_count: usize = 0;
    if let Some(config_map) = cm {
        let data = config_map.data;

        // TODO: This might interfere with other properties having the "server."
        //    string contained. Needs more stable solution.
        if let Some(value) = data.and_then(|data| data.get("zoo.cfg").cloned()) {
            server_count = value.matches("server.").count();
        }

        if server_count == expected_server_count {
            return Ok(());
        }
    }

    Err(anyhow!(
        "ConfigMap server.<id> properties [{}] do not match the expected number of server.<id> properties [{}]",
        server_count, expected_server_count
    ))
}

/// Extracts the name of the `config_type_label` configmap of a pod.
fn get_config_cm(client: &TestKubeClient, pod: &Pod, config_type_label: &str) -> Result<String> {
    let pod_name = pod.metadata.name.as_ref().unwrap();

    if let Some(volumes) = &pod.spec.as_ref().unwrap().volumes {
        for volume in volumes {
            if let Some(ConfigMapVolumeSource {
                name: Some(cm_name),
                ..
            }) = &volume.config_map
            {
                // get config map and check labels for `config_type_label` which indicates the type
                // of the config map we are looking for.
                if let Some(config_map) = client.find_namespaced::<ConfigMap>(cm_name) {
                    if config_map
                        .metadata
                        .labels
                        .and_then(|labels| labels.get(config_type_label).cloned())
                        == Some(stackable_zookeeper_crd::CONFIG_MAP_TYPE_DATA.to_string())
                    {
                        return Ok(cm_name.clone());
                    }
                }
            }
        }
    }

    Err(anyhow!(
        "Could not find config map of type [{}] for pod [{}]",
        config_type_label,
        pod_name
    ))
}

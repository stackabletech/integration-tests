use std::net::TcpStream;

use crate::common::service::SupersetService;
use anyhow::{anyhow, Context, Result};
use integration_test_commons::test::kube::TestKubeClient;
use integration_test_commons::test::prelude::{json, Pod};
use reqwest::blocking::Client;
use stackable_operator::kube::ResourceExt;

/// Collect and gather all checks that may be performed on Superset node pods.
pub fn custom_checks(
    client: &TestKubeClient,
    pods: &[Pod],
    admin_username: &str,
    admin_password: &str,
) -> Result<()> {
    let service = SupersetService::new(client);

    for pod in pods {
        scan_port(&service, pod)?;
        login(&service, pod, admin_username, admin_password)?;
    }

    Ok(())
}

/// Scan HTTP port.
pub fn scan_port(service: &SupersetService, pod: &Pod) -> Result<()> {
    let address = service.address(pod);

    TcpStream::connect(&address)
        .with_context(|| format!("TCP error occurred when connecting to [{}]", address))?;

    Ok(())
}

/// Login to Superset as admin
pub fn login(
    service: &SupersetService,
    pod: &Pod,
    admin_username: &str,
    admin_password: &str,
) -> Result<()> {
    let client = Client::new();

    let address = service.address(pod);

    let response = client
        .post(format!("http://{}/api/v1/security/login", address))
        .json(&json!({
            "password": admin_password,
            "provider": "db",
            "refresh": true,
            "username": admin_username,
        }))
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Login on pod [{}] failed. {:?}",
            pod.name(),
            response
        ))
    }
}

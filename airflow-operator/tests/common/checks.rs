use crate::Service;
use anyhow::{anyhow, Result};
use integration_test_commons::operator::checks;
use integration_test_commons::operator::service::TemporaryService;
use integration_test_commons::stackable_operator::kube::ResourceExt;
use integration_test_commons::test::prelude::{json, Pod};
use reqwest::blocking::Client;

/// Collect and gather all checks that may be performed on airflow node services.
pub fn custom_checks(
    service_pods: &[Pod],
    admin_username: &str,
    admin_password: &str,
    service: &TemporaryService,
) -> Result<()> {
    for service_pod in service_pods {
        let named_pod = service_pod
            .metadata
            .generate_name
            .as_ref()
            .unwrap()
            .as_str();
        if named_pod.ends_with("webserver-default-") {
            println!("{:?}/{}", named_pod, &service.address(service_pod));
            checks::scan_port(&service.address(service_pod))?;
        } else {
            println!("{}", named_pod);
        }
        //login(service, pod, admin_username, admin_password)?;
    }

    Ok(())
}

/// Login to Superset as admin
pub fn login(
    service: &TemporaryService,
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

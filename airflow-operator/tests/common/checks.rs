use anyhow::{anyhow, Result};
use integration_test_commons::operator::checks;
use integration_test_commons::operator::service::TemporaryService;
use integration_test_commons::stackable_operator::kube::ResourceExt;
use integration_test_commons::test::prelude::{json, Pod};
use reqwest::blocking::Client;

/// Collect and gather all checks that may be performed on airflow node pods.
pub fn custom_checks(
    pods: &[Pod],
    admin_username: &str,
    admin_password: &str,
    service: &TemporaryService,
) -> Result<()> {
    for pod in pods {
        println!("{}", &service.address(pod));
        checks::scan_port(&service.address(pod))?;
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

use anyhow::{anyhow, Result};
use integration_test_commons::operator::checks;
use integration_test_commons::operator::service::TemporaryService;
use integration_test_commons::stackable_operator::kube::ResourceExt;
use integration_test_commons::test::prelude::Pod;
use reqwest::blocking::Client;

/// Collect and gather all checks that may be performed on airflow node services.
pub fn custom_checks(service_pods: &[Pod], service: &TemporaryService) -> Result<()> {
    for service_pod in service_pods {
        let named_pod = service_pod
            .metadata
            .generate_name
            .as_ref()
            .unwrap()
            .as_str();
        println!("{:?}/{}", named_pod, &service.address(service_pod));
        checks::scan_port(&service.address(service_pod))?;
        health_check(service, service_pod)?;
    }
    Ok(())
}

pub fn health_check(service: &TemporaryService, pod: &Pod) -> Result<()> {
    let client = Client::new();

    let address = service.address(pod);

    let response = client
        .get(format!("http://{}/api/v1/health", address))
        .send()?;
    println!("Returned {}, {:?}", &response.status(), &response);

    if response.status().is_success() {
        Ok(())
    } else {
        Err(anyhow!(
            "Health-check on pod [{}] failed. {:?}",
            pod.name(),
            response
        ))
    }
}

use crate::common::service::SupersetService;
use anyhow::Result;
use integration_test_commons::test::kube::TestKubeClient;
use integration_test_commons::test::prelude::Pod;

/// Collect and gather all checks that may be performed on Superset node pods.
pub fn custom_checks(client: &TestKubeClient, pods: &[Pod]) -> Result<()> {
    let service = SupersetService::new(client);

    for pod in pods {
        scan_port(&service, pod)?;
    }

    Ok(())
}

/// Scan HTTP port.
pub fn scan_port(service: &SupersetService, pod: &Pod) -> Result<()> {
    service.scan_port(pod)
}

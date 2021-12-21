use crate::common::four_letter_commands::send_4lw_i_am_ok;
use anyhow::Result;
use integration_test_commons::operator::service::TemporaryService;
use integration_test_commons::test::prelude::Pod;

/// Collect and gather all checks that may be performed on ZooKeeper server pods.
pub fn custom_checks(pods: &[Pod], version: &str, service: &TemporaryService) -> Result<()> {
    for pod in pods {
        let address = &service.address(pod);
        send_4lw_i_am_ok(version, address)?;
    }
    Ok(())
}

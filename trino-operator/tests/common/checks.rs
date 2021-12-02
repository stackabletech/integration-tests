use anyhow::Result;
use indoc::indoc;
use integration_test_commons::operator::service::TemporaryService;
use integration_test_commons::stackable_operator::k8s_openapi::serde::Deserialize;
use integration_test_commons::test::prelude::Pod;

use super::trino_client::TrinoClient;

/// Company table schema
#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Company {
    name: String,
}

/// Collect and gather all checks that may be performed on Trino node pods.
pub fn custom_checks(
    trino_service: &TemporaryService,
    pods: &[Pod],
    trino_username: &str,
    trino_password: &str,
    pem_certificate: &[u8],
) -> Result<()> {
    for pod in pods {
        let trino_client = TrinoClient::new(
            &trino_service.address(pod),
            trino_username,
            trino_password,
            pem_certificate,
        );

        let result = trino_client.execute::<Vec<bool>>(indoc!(
            "CREATE SCHEMA IF NOT EXISTS hive.test
            WITH (location = 's3a://test/')"
        ))?;
        assert_eq!(vec![vec![true]], result);

        let result = trino_client.execute::<Vec<bool>>(indoc!(
            "CREATE TABLE IF NOT EXISTS hive.test.companies (
              name VARCHAR
            )
            WITH (
              external_location = 's3a://test/csv',
              format = 'CSV'
            )",
        ))?;
        assert_eq!(vec![vec![true]], result);

        let data_set = trino_client.execute("SELECT * FROM hive.test.companies")?;
        assert_eq!(
            vec![Company {
                name: String::from("Stackable")
            }],
            data_set
        );
    }

    Ok(())
}

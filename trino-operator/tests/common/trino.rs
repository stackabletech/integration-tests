use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::k8s_openapi::serde::{
    de::DeserializeOwned, Serialize,
};
use integration_test_commons::stackable_operator::kube::api::ObjectMeta;
use integration_test_commons::stackable_operator::kube::Resource;
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use integration_test_commons::stackable_operator::role_utils::{
    CommonConfiguration, Role, RoleGroup,
};
use integration_test_commons::test::prelude::NodeAddress;
use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MsbOption};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::x509::extension::{
    BasicConstraints, ExtendedKeyUsage, KeyUsage, SubjectAlternativeName, SubjectKeyIdentifier,
};
use openssl::x509::{X509Name, X509};
use stackable_hive_crd::discovery::{HiveReference, S3Connection};
use stackable_trino_crd::{TrinoCluster, TrinoClusterSpec, TrinoConfig, TrinoVersion, APP_NAME};

use std::collections::HashMap;
use std::fmt::Debug;
use std::str;
use std::time::Duration;

pub const TRINO_USERNAME: &str = "admin";
pub const TRINO_PASSWORD: &str = "hunter2";
pub const TRINO_COORDINATOR_HTTPS_PORT: u16 = 8443;

pub fn build_trino_test_cluster<T>() -> TestCluster<T>
where
    T: Clone + Debug + DeserializeOwned + Resource<DynamicType = ()> + Serialize,
{
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "test"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
            pods_terminated_delay: Duration::ZERO,
        },
    )
}

/// Creates a Trino cluster custom resource
pub fn build_trino_cluster(
    name: &str,
    version: TrinoVersion,
    coordinator_replicas: u16,
    worker_replicas: u16,
    hive_name: &str,
    s3_connection: Option<&S3Connection>,
    node_addresses: &[&NodeAddress],
) -> Result<(TrinoCluster, u16, Vec<u8>)> {
    let key_pair = Rsa::generate(2048).and_then(PKey::from_rsa)?;
    let cert = generate_tls_certificate(&key_pair, node_addresses)?;

    let mut coordinator_role_groups = HashMap::new();
    coordinator_role_groups.insert(
        String::from("default"),
        RoleGroup {
            config: Some(CommonConfiguration {
                config: Some(TrinoConfig {
                    coordinator: Some(true),
                    http_server_http_port: Some(8080),
                    http_server_https_port: Some(TRINO_COORDINATOR_HTTPS_PORT),
                    query_max_memory: None,
                    query_max_memory_per_node: None,
                    query_max_total_memory_per_node: None,
                    node_data_dir: Some(String::from("/stackable/trino/data")),
                    io_trino: None,
                    metrics_port: None,
                    server_certificate: Some(format!(
                        "{}{}",
                        str::from_utf8(&key_pair.private_key_to_pem_pkcs8()?)?,
                        str::from_utf8(&cert.to_pem()?)?
                    )),
                    password_file_content: Some(format!(
                        "{}:{}",
                        TRINO_USERNAME,
                        bcrypt::hash_with_result(TRINO_PASSWORD, 10)?
                            .format_for_version(bcrypt::Version::TwoY),
                    )),
                }),
                config_overrides: None,
                env_overrides: None,
                cli_overrides: None,
            }),
            replicas: Some(coordinator_replicas),
            selector: None,
        },
    );

    let mut worker_role_groups = HashMap::new();
    worker_role_groups.insert(
        String::from("default"),
        RoleGroup {
            config: Some(CommonConfiguration {
                config: Some(TrinoConfig {
                    coordinator: None,
                    http_server_http_port: Some(8081),
                    http_server_https_port: None,
                    query_max_memory: None,
                    query_max_memory_per_node: None,
                    query_max_total_memory_per_node: None,
                    node_data_dir: Some(String::from("/stackable/trino/data")),
                    io_trino: None,
                    metrics_port: None,
                    server_certificate: None,
                    password_file_content: None,
                }),
                config_overrides: None,
                env_overrides: None,
                cli_overrides: None,
            }),
            replicas: Some(worker_replicas),
            selector: None,
        },
    );

    let spec = TrinoCluster {
        api_version: String::from("trino.stackable.tech/v1alpha1"),
        kind: String::from("TrinoCluster"),
        metadata: ObjectMeta {
            name: Some(String::from(name)),
            ..Default::default()
        },
        spec: TrinoClusterSpec {
            version,
            node_environment: String::from("production"),
            hive_reference: HiveReference {
                namespace: String::from("default"),
                name: String::from(hive_name),
                ..Default::default()
            },
            opa: None,
            authorization: None,
            s3_connection: s3_connection.cloned(),
            coordinators: Role {
                config: None,
                role_groups: coordinator_role_groups,
            },
            workers: Role {
                config: None,
                role_groups: worker_role_groups,
            },
        },
        status: None,
    };

    Ok((spec, coordinator_replicas + worker_replicas, cert.to_pem()?))
}

/// Generates a TLS certificate
fn generate_tls_certificate(
    key_pair: &PKey<Private>,
    node_addresses: &[&NodeAddress],
) -> Result<X509> {
    let mut cert_builder = X509::builder()?;

    let subject_name = {
        let mut builder = X509Name::builder()?;
        builder.append_entry_by_text("CN", "trino-coordinator")?;
        builder.build()
    };

    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(128, MsbOption::MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };

    let subject_alternative_name = {
        let mut builder = SubjectAlternativeName::new();
        for node_address in node_addresses {
            if node_address.type_ == "Hostname" {
                builder.dns(&node_address.address);
            } else {
                builder.ip(&node_address.address);
            }
        }
        builder.build(&cert_builder.x509v3_context(None, None))?
    };

    cert_builder.set_version(2)?;
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&subject_name)?;
    cert_builder.set_issuer_name(&subject_name)?;
    cert_builder.set_pubkey(key_pair)?;
    cert_builder.set_not_before(&Asn1Time::from_str("19700101000000Z").unwrap())?;
    cert_builder.set_not_after(&Asn1Time::from_str("99991231235959Z").unwrap())?;
    cert_builder.append_extension(BasicConstraints::new().build()?)?;
    cert_builder.append_extension(
        SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?,
    )?;
    cert_builder.append_extension(
        KeyUsage::new()
            .digital_signature()
            .key_encipherment()
            .build()?,
    )?;
    cert_builder.append_extension(ExtendedKeyUsage::new().server_auth().build()?)?;
    cert_builder.append_extension(subject_alternative_name)?;
    cert_builder.sign(key_pair, MessageDigest::sha256())?;

    Ok(cert_builder.build())
}

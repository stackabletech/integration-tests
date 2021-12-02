use anyhow::Result;
use indoc::formatdoc;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::k8s_openapi::serde::{
    de::DeserializeOwned, Serialize,
};
use integration_test_commons::stackable_operator::kube::Resource;
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use stackable_hive_crd::discovery::S3Connection;
use stackable_hive_crd::{HiveCluster, HiveVersion, APP_NAME};
use std::fmt::Debug;
use std::time::Duration;

pub fn build_hive_test_cluster<T>() -> TestCluster<T>
where
    T: Clone + Debug + DeserializeOwned + Resource<DynamicType = ()> + Serialize,
{
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "test"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts {
            cluster_ready: Duration::from_secs(300),
            pods_terminated: Duration::from_secs(30),
        },
    )
}

/// Creates a Hive cluster custom resource
pub fn build_hive_cluster(
    name: &str,
    version: HiveVersion,
    replicas: usize,
    s3_connection: Option<&S3Connection>,
) -> Result<(HiveCluster, usize)> {
    let spec = &formatdoc!(
        "
        apiVersion: hive.stackable.tech/v1alpha1
        kind: HiveCluster
        metadata:
          name: {name}
        spec:
          version: {version}
          metastore:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                replicas: {replicas}
                config:
                  database:
                    connString: jdbc:derby:;databaseName=/stackable/data/metadata_db;create=true
                    user: APP
                    password: mine
                    dbType: derby
                  s3Connection:
                    endPoint: {s3_endpoint}
                    accessKey: {s3_access_key}
                    secretKey: {s3_secret_key}
                    sslEnabled: {s3_ssl_enabled}
                    pathStyleAccess: {s3_path_style_access}
        ",
        name = name,
        version = version.to_string(),
        replicas = replicas,
        s3_endpoint = s3_connection
            .map(|c| c.end_point.as_str())
            .unwrap_or_default(),
        s3_access_key = s3_connection
            .map(|c| c.access_key.as_str())
            .unwrap_or_default(),
        s3_secret_key = s3_connection
            .map(|c| c.secret_key.as_str())
            .unwrap_or_default(),
        s3_ssl_enabled = s3_connection.map(|c| c.ssl_enabled).unwrap_or_default(),
        s3_path_style_access = s3_connection
            .map(|c| c.path_style_access)
            .unwrap_or_default(),
    );

    Ok((serde_yaml::from_str(spec)?, replicas))
}

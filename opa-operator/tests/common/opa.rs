use anyhow::Result;
use integration_test_commons::operator::setup::{
    TestCluster, TestClusterLabels, TestClusterOptions, TestClusterTimeouts,
};
use integration_test_commons::stackable_operator::k8s_openapi::api::apps::v1::DaemonSet;
use integration_test_commons::stackable_operator::kube::Resource;
use integration_test_commons::stackable_operator::labels::{
    APP_INSTANCE_LABEL, APP_NAME_LABEL, APP_VERSION_LABEL,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use stackable_opa_crd::{OpenPolicyAgent, APP_NAME};
use std::fmt::Debug;

/// Predefined options and timeouts for the TestCluster.
pub fn build_test_cluster() -> TestCluster<OpenPolicyAgent> {
    TestCluster::new(
        &TestClusterOptions::new(APP_NAME, "simple-opa"),
        &TestClusterLabels::new(APP_NAME_LABEL, APP_INSTANCE_LABEL, APP_VERSION_LABEL),
        &TestClusterTimeouts::default(),
    )
}

/// This returns an OPA custom resource.
pub fn build_opa_cluster(name: &str, version: &str) -> Result<OpenPolicyAgent> {
    let spec = &format!(
        "
        apiVersion: opa.stackable.tech/v1alpha1
        kind: OpenPolicyAgent
        metadata:
          name: {}
        spec:
          version: {}
          servers:
            roleGroups:
              default:
                selector:
                  matchLabels:
                    kubernetes.io/os: linux
                config:
                  regoRuleReference: regorule-operator:3030/opa/v1
    ",
        name, version,
    );

    Ok(serde_yaml::from_str(spec)?)
}

/// The Opa operator runs based on a [`DaemonSet`]. We use the `desired_number_scheduled` status
/// field to retrieve the expected amount of created pods / replicas.
pub fn get_worker_nodes<T>(cluster: &TestCluster<T>) -> i32
where
    T: Clone + Debug + DeserializeOwned + Resource<DynamicType = ()> + Serialize,
{
    let daemon_sets = cluster.list::<DaemonSet>(None);
    let ds = daemon_sets.get(0).expect("No opa daemon set found!");
    let ds_status = ds.status.as_ref().expect("Opa daemon set has no status!");

    println!(
        "DaemonSet - desired_number_scheduled: {}",
        ds_status.desired_number_scheduled
    );

    ds_status.desired_number_scheduled
}

//! Clients for interacting with the Kubernetes API
//!
//! These clients simplify testing.

use anyhow::{anyhow, Result};
use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::core::v1::{Node, NodeCondition, Pod, PodCondition, Taint};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition, CustomResourceDefinitionCondition,
};
use kube::api::{
    Api, DeleteParams, ListParams, ObjectList, Patch, PatchParams, PostParams, WatchEvent,
};
use kube::{Client, Resource, ResourceExt};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::{fmt::Debug, time::Duration};
use tokio::runtime::Runtime;
use uuid::Uuid;

pub use kube::api::LogParams;

/// A client for interacting with the Kubernetes API
///
/// [`TestKubeClient`] is a synchronous version of [`KubeClient`] which
/// additionally panics on erroneous results. It reduces the verbosity of
/// test cases.
pub struct TestKubeClient {
    runtime: Runtime,
    kube_client: KubeClient,
}

impl TestKubeClient {
    /// Creates a [`TestKubeClient`].
    pub fn new() -> TestKubeClient {
        let runtime = Runtime::new().expect("Tokio runtime could not be created");
        let kube_client = runtime.block_on(async {
            KubeClient::new()
                .await
                .expect("Kubernetes client could not be created")
        });
        TestKubeClient {
            runtime,
            kube_client,
        }
    }

    pub fn timeouts(&mut self) -> &mut Timeouts {
        &mut self.kube_client.timeouts
    }

    /// Gets a list of resources restricted by the label selector.
    ///
    /// The label selector supports `=`, `==`, `!=`, and can be comma
    /// separated: `key1=value1,key2=value2`.
    pub fn list_labeled<K>(&self, label_selector: &str) -> ObjectList<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .list_labeled(label_selector)
                .await
                .expect("List of Stackable nodes could not be retrieved")
        })
    }

    /// Applies the given custom resource definition and blocks until it is accepted.
    pub fn apply_crd(&self, crd: &CustomResourceDefinition) {
        self.runtime.block_on(async {
            self.kube_client
                .apply_crd(crd)
                .await
                .expect("Custom resource definition could not be applied")
        })
    }

    /// Searches for a resource.
    pub fn find<K>(&self, name: &str) -> Option<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime
            .block_on(async { self.kube_client.find::<K>(name).await })
    }

    /// Searches for a namespaced resource.
    pub fn find_namespaced<K>(&self, name: &str) -> Option<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime
            .block_on(async { self.kube_client.find_namespaced::<K>(name).await })
    }

    /// Applies a resource with the given YAML specification.
    pub fn apply<K>(&self, spec: &str) -> K
    where
        K: Clone + Debug + DeserializeOwned + Resource + Serialize,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .apply::<K>(spec)
                .await
                .expect("Resource could not be applied")
        })
    }

    /// Creates a resource with the given YAML specification.
    pub fn create<K>(&self, spec: &str) -> K
    where
        K: Clone + Debug + DeserializeOwned + Resource + Serialize,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .create(spec)
                .await
                .expect("Resource could not be created")
        })
    }

    /// Deletes the given resource.
    pub fn delete<K>(&self, resource: K)
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .delete(resource)
                .await
                .expect("Resource could not be deleted")
        })
    }

    /// Returns the value of an annotation for the given resource.
    pub fn get_annotation<K>(&self, resource: &K, key: &str) -> String
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .get_annotation(resource, key)
                .await
                .expect("Annotation could not be retrieved")
        })
    }

    /// Verifies that the given pod condition becomes true within the
    /// specified timeout.
    pub fn verify_pod_condition(&self, pod: &Pod, condition_type: &str) -> Pod {
        self.runtime.block_on(async {
            self.kube_client
                .verify_pod_condition(pod, condition_type)
                .await
                .expect("Pod condition could not be verified")
        })
    }

    /// Verifies that the status of a resource fulfills the given
    /// predicate within the specified timeout.
    pub fn verify_status<K, P>(&self, resource: &K, predicate: P) -> K
    where
        P: Fn(&K) -> bool,
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .verify_status(resource, predicate)
                .await
                .expect("Resource did not reach the expected status")
        })
    }

    /// Returns the given resource with an updated status.
    pub fn get_status<K>(&self, resource: &K) -> K
    where
        K: DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        self.runtime.block_on(async {
            self.kube_client
                .get_status(resource)
                .await
                .expect("Status could not be retrieved")
        })
    }

    /// Returns the logs for the given pod.
    pub fn get_logs(&self, pod: &Pod, params: &LogParams) -> Vec<String> {
        self.runtime.block_on(async {
            self.kube_client
                .get_logs(pod, params)
                .await
                .expect("Logs could not be retrieved")
        })
    }
}

impl Default for TestKubeClient {
    fn default() -> Self {
        Self::new()
    }
}

/// A client for interacting with the Kubernetes API
///
/// [`KubeClient`] wraps a [`Client`][kube::Client]. It provides methods
/// which are less verbose and await the according status change within
/// defined timeouts.
pub struct KubeClient {
    client: Client,
    namespace: String,
    pub timeouts: Timeouts,
}

/// Timeouts for operations
pub struct Timeouts {
    pub apply_crd: Duration,
    pub create: Duration,
    pub delete: Duration,
    pub get_annotation: Duration,
    pub verify_status: Duration,
}

impl Default for Timeouts {
    fn default() -> Self {
        Timeouts {
            apply_crd: Duration::from_secs(30),
            create: Duration::from_secs(10),
            delete: Duration::from_secs(10),
            get_annotation: Duration::from_secs(10),
            verify_status: Duration::from_secs(294),
        }
    }
}

impl KubeClient {
    /// Creates a [`KubeClient`].
    pub async fn new() -> Result<KubeClient> {
        let client = Client::try_default().await?;
        Ok(KubeClient {
            client,
            namespace: String::from("default"),
            timeouts: Default::default(),
        })
    }

    /// Gets a list of resources restricted by the label selector.
    ///
    /// The label selector supports `=`, `==`, `!=`, and can be comma separated:
    /// `key1=value1,key2=value2`.
    pub async fn list_labeled<K>(&self, label_selector: &str) -> Result<ObjectList<K>>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let api: Api<K> = Api::all(self.client.clone());
        let lp = ListParams::default().labels(label_selector);
        Ok(api.list(&lp).await?)
    }

    /// Applies the given custom resource definition and awaits the accepted status.
    pub async fn apply_crd(&self, crd: &CustomResourceDefinition) -> Result<()> {
        let is_ready = |crd: &CustomResourceDefinition| {
            get_crd_conditions(crd)
                .iter()
                .any(|condition| condition.type_ == "NamesAccepted" && condition.status == "True")
        };

        let timeout_secs = self.timeouts.apply_crd.as_secs() as u32;
        let crds: Api<CustomResourceDefinition> = Api::all(self.client.clone());

        let lp = ListParams::default()
            .fields(&format!("metadata.name={}", crd.name()))
            .timeout(timeout_secs);
        let mut stream = crds.watch(&lp, "0").await?.boxed();

        let apply_params = PatchParams::apply("agent_integration_test").force();
        crds.patch(&crd.name(), &apply_params, &Patch::Apply(crd))
            .await?;

        if crds.get(&crd.name()).await.is_ok() {
            return Ok(());
        }

        while let Some(status) = stream.try_next().await? {
            if let WatchEvent::Modified(crd) = status {
                if is_ready(&crd) {
                    return Ok(());
                }
            }
        }

        Err(anyhow!(
            "Custom resource definition [{}] could not be applied within {} seconds.",
            crd.name(),
            timeout_secs
        ))
    }

    /// Searches for a resource.
    pub async fn find<K>(&self, name: &str) -> Option<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let api: Api<K> = Api::all(self.client.clone());
        api.get(name).await.ok()
    }

    /// Searches for a namespaced resource.
    pub async fn find_namespaced<K>(&self, name: &str) -> Option<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);
        api.get(name).await.ok()
    }

    /// Applies a resource with the given YAML specification.
    pub async fn apply<K>(&self, spec: &str) -> Result<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource + Serialize,
        <K as Resource>::DynamicType: Default,
    {
        let resource: K = from_yaml(spec);
        let apply_params = PatchParams::apply("agent_integration_test").force();
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);
        Ok(api
            .patch(&resource.name(), &apply_params, &Patch::Apply(&resource))
            .await?)
    }

    /// Creates a resource with the given YAML specification and awaits the
    /// confirmation of the creation.
    pub async fn create<K>(&self, spec: &str) -> Result<K>
    where
        K: Clone + Debug + DeserializeOwned + Resource + Serialize,
        <K as Resource>::DynamicType: Default,
    {
        let timeout_secs = self.timeouts.create.as_secs() as u32;
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);

        let resource: K = from_yaml(spec);

        let list_params = ListParams::default()
            .fields(&format!("metadata.name={}", resource.name()))
            .timeout(timeout_secs);
        let mut stream = api.watch(&list_params, "0").await?.boxed();

        api.create(&PostParams::default(), &resource).await?;

        while let Some(status) = stream.try_next().await? {
            if let WatchEvent::Added(resource) = status {
                return Ok(resource);
            }
        }

        Err(anyhow!(
            "Resource [{}] could not be created within {} seconds.",
            resource.name(),
            timeout_secs
        ))
    }

    /// Deletes the given resource and awaits the confirmation of the deletion.
    pub async fn delete<K>(&self, resource: K) -> Result<()>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let timeout_secs = self.timeouts.delete.as_secs() as u32;
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);

        let list_params = ListParams::default()
            .fields(&format!("metadata.name={}", resource.name()))
            .timeout(timeout_secs);
        let mut stream = api.watch(&list_params, "0").await?.boxed();

        let result = api
            .delete(&resource.name(), &DeleteParams::default())
            .await?;

        if result.is_right() {
            return Ok(());
        }

        while let Some(status) = stream.try_next().await? {
            if let WatchEvent::Deleted(_) = status {
                return Ok(());
            }
        }

        Err(anyhow!(
            "Resource [{}] could not be deleted within {} seconds.",
            resource.name(),
            timeout_secs
        ))
    }

    /// Returns the value of an annotation for the given resource.
    pub async fn get_annotation<K>(&self, resource: &K, key: &str) -> Result<String>
    where
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let get_value = |resource: &K| {
            resource
                .meta()
                .annotations
                .as_ref()
                .and_then(|annotations| annotations.get(key).cloned())
        };

        let timeout_secs = self.timeouts.get_annotation.as_secs() as u32;
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);

        let lp = ListParams::default()
            .fields(&format!("metadata.name={}", resource.name()))
            .timeout(timeout_secs);
        let mut stream = api.watch(&lp, "0").await?.boxed();

        if let Some(value) = get_value(resource) {
            return Ok(value);
        }

        while let Some(event) = stream.try_next().await? {
            if let WatchEvent::Added(resource) | WatchEvent::Modified(resource) = event {
                if let Some(value) = get_value(&resource) {
                    return Ok(value);
                }
            }
        }

        Err(anyhow!(
            "Annotation [{}] could not be retrieved from [{}] within {} seconds",
            key,
            resource.name(),
            timeout_secs
        ))
    }

    /// Verifies that the given pod condition becomes true within the specified timeout.
    pub async fn verify_pod_condition(&self, pod: &Pod, condition_type: &str) -> Result<Pod> {
        let is_condition_true = |pod: &Pod| {
            get_pod_conditions(pod)
                .iter()
                .any(|condition| condition.type_ == condition_type && condition.status == "True")
        };
        self.verify_status(pod, is_condition_true).await
    }

    /// Verifies that the status of a resource fulfills the given
    /// predicate within the specified timeout.
    pub async fn verify_status<K, P>(&self, resource: &K, predicate: P) -> Result<K>
    where
        P: Fn(&K) -> bool,
        K: Clone + Debug + DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let timeout_secs = self.timeouts.verify_status.as_secs() as u32;
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);

        let lp = ListParams::default()
            .fields(&format!("metadata.name={}", resource.name()))
            .timeout(timeout_secs);
        let mut stream = api.watch(&lp, "0").await?.boxed();

        let resource = api.get_status(&resource.name()).await?;

        if predicate(&resource) {
            return Ok(resource);
        }

        while let Some(status) = stream.try_next().await? {
            if let WatchEvent::Modified(resource) = status {
                if predicate(&resource) {
                    return Ok(resource);
                }
            }
        }

        Err(anyhow!(
            "Resource [{}] did not reach the expected status within {} seconds.",
            resource.name(),
            timeout_secs
        ))
    }

    /// Returns the given resource with an updated status.
    pub async fn get_status<K>(&self, resource: &K) -> Result<K>
    where
        K: DeserializeOwned + Resource,
        <K as Resource>::DynamicType: Default,
    {
        let api: Api<K> = Api::namespaced(self.client.clone(), &self.namespace);
        Ok(api.get_status(&resource.name()).await?)
    }

    /// Returns the logs for the given pod.
    pub async fn get_logs(&self, pod: &Pod, params: &LogParams) -> Result<Vec<String>> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), &self.namespace);

        let bytes = pods
            .log_stream(&pod.name(), params)
            .await?
            .try_collect::<Vec<_>>()
            .await?
            .concat();

        let lines = String::from_utf8_lossy(&bytes)
            .lines()
            .map(|line| line.to_owned())
            .collect();

        Ok(lines)
    }
}

/// Deserializes the given JSON value into the desired type.
pub fn from_value<T>(value: Value) -> T
where
    T: DeserializeOwned,
{
    T::deserialize(value).expect("Deserialization failed")
}

/// Deserializes the given YAML text into the desired type.
pub fn from_yaml<T>(yaml: &str) -> T
where
    T: DeserializeOwned,
{
    serde_yaml::from_str(yaml).expect("String is not a well-formed YAML")
}

/// Appends a UUID to `metadata/name`.
pub fn with_unique_name(yaml: &str) -> String {
    let mut spec: serde_yaml::Value = from_yaml(yaml);
    let name = &mut spec["metadata"]["name"];
    *name = format!(
        "{}-{}",
        name.as_str().expect("metadata/name is invalid"),
        Uuid::new_v4()
    )
    .into();
    serde_yaml::to_string(&spec).unwrap()
}

/// Returns the conditions of the given node.
pub fn get_node_conditions(node: &Node) -> Vec<NodeCondition> {
    if let Some(status) = &node.status {
        status.conditions.clone().unwrap_or_default()
    } else {
        vec![]
    }
}

/// Returns the conditions of the given pod.
pub fn get_pod_conditions(pod: &Pod) -> Vec<PodCondition> {
    if let Some(status) = &pod.status {
        status.conditions.clone().unwrap_or_default()
    } else {
        vec![]
    }
}

/// Returns the conditions of the given custom resource definition.
pub fn get_crd_conditions(
    crd: &CustomResourceDefinition,
) -> Vec<CustomResourceDefinitionCondition> {
    if let Some(status) = &crd.status {
        status.conditions.clone().unwrap_or_default()
    } else {
        vec![]
    }
}

/// Returns the taints of the given node.
pub fn get_node_taints(node: &Node) -> Vec<Taint> {
    if let Some(spec) = &node.spec {
        spec.taints.clone().unwrap_or_default()
    } else {
        vec![]
    }
}

/// Returns the number of allocatable pods of the given node.
pub fn get_allocatable_pods(node: &Node) -> u32 {
    node.status
        .as_ref()
        .and_then(|status| status.allocatable.as_ref())
        .and_then(|allocatable| allocatable.get("pods"))
        .and_then(|quantity| quantity.0.parse().ok())
        .unwrap_or_default()
}

pub mod common;

use crate::common::opa::get_worker_nodes;
use anyhow::Result;
use common::opa::{build_opa_cluster, build_test_cluster};
use std::collections::BTreeMap;
use std::convert::TryInto;

#[test]
fn test_create_cluster_0_27_1() -> Result<()> {
    let version = "0.27.1";

    let mut cluster = build_test_cluster();

    let opa_cr = build_opa_cluster(cluster.name(), version)?;

    cluster.apply(&opa_cr)?;

    let replicas = get_worker_nodes(&cluster);
    cluster.wait_ready(&BTreeMap::new(), replicas.try_into().unwrap())?;
    cluster.check_pod_version(version)
}

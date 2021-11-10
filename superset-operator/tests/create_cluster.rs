pub mod common;

use anyhow::Result;
use common::{
    checks::custom_checks,
    superset::{build_command, build_superset_cluster, build_test_cluster},
};
use integration_test_commons::test::prelude::Pod;
use stackable_superset_crd::{commands::Init, SupersetVersion};

#[test]
fn test_create_cluster_1_3_2() -> Result<()> {
    let version = SupersetVersion::v1_3_2;
    let mut cluster = build_test_cluster();

    let superset_cr = build_superset_cluster(cluster.name(), &version)?;
    let pod_count = 1;
    cluster.create_or_update(&superset_cr, pod_count)?;
    let created_pods = cluster.list::<Pod>(None);

    let init: Init = build_command("superset-cluster-command-init", "Init", cluster.name())?;
    cluster.apply_command(&init)?;

    custom_checks(&cluster.client, &created_pods)
}

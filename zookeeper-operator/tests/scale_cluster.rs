pub mod common;

use crate::common::checks::custom_checks;
use crate::common::zookeeper::{build_test_cluster, build_zk_cluster};

use anyhow::Result;
use integration_test_commons::test::prelude::Pod;
use stackable_zookeeper_crd::ZookeeperVersion;

// This will cause the integration tests to fail because config maps are not updated correctly. This
// can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
// #[test]
// #[ignore]
// fn test_scale_cluster_up() -> Result<()> {
//     let version = ZookeeperVersion::v3_5_8;
//     let mut cluster = build_test_cluster();
//
//     let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;
//     cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
//     let created_pods = cluster.list::<Pod>(None);
//
//     custom_checks(
//         &cluster.client,
//         created_pods.as_slice(),
//         &version,
//         expected_pod_count,
//     )?;
//
//     let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 3)?;
//     cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
//     let created_pods = cluster.list::<Pod>(None);
//
//     custom_checks(
//         &cluster.client,
//         created_pods.as_slice(),
//         &version,
//         expected_pod_count,
//     )?;
//
//     Ok(())
// }
//
// // This will cause the integration tests to fail because config maps are not updated correctly. This
// // can be activated once https://github.com/stackabletech/zookeeper-operator/issues/128 is fixed.
// #[test]
// #[ignore]
// fn test_scale_cluster_down() -> Result<()> {
//     let version = ZookeeperVersion::v3_5_8;
//     let mut cluster = build_test_cluster();
//
//     let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 3)?;
//     cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
//     let created_pods = cluster.list::<Pod>(None);
//
//     custom_checks(
//         &cluster.client,
//         created_pods.as_slice(),
//         &version,
//         expected_pod_count,
//     )?;
//
//     let (zookeeper_cr, expected_pod_count) = build_zk_cluster(cluster.name(), &version, 1)?;
//     cluster.create_or_update(&zookeeper_cr, expected_pod_count)?;
//     let created_pods = cluster.list::<Pod>(None);
//
//     custom_checks(
//         &cluster.client,
//         created_pods.as_slice(),
//         &version,
//         expected_pod_count,
//     )?;
//
//     Ok(())
// }

pub mod common;

use anyhow::{anyhow, Result};
use common::druid::{build_druid_cluster, build_test_cluster, TestService};
use integration_test_commons::test::prelude::{Pod};
use std::{thread, time};

#[test]
fn test_create_1_cluster_0_22_0() -> Result<()> {
    let version = "0.22.0";
    let mut cluster = build_test_cluster();

    let (druid_cr, expected_pod_count) = build_druid_cluster(cluster.name(), version, 1)?;
    cluster.create_or_update(&druid_cr, expected_pod_count)?;

    // Wait for the metastore to have started fully
    let delay_time = time::Duration::from_secs(3);
    thread::sleep(delay_time);

    let created_pods = cluster.list::<Pod>(None);
    let actual_pod_count = created_pods.len();

    if actual_pod_count != expected_pod_count {
        return Err(anyhow!(
            "Expected {} pods but got {}!",
            expected_pod_count,
            actual_pod_count
        ));
    }

    // for each process/pod, create a NodePort service and check the health status
    let s = TestService::new(&cluster.client, "druid", "coordinator", 8081, 30081);
    s.conduct_healthcheck(&cluster.client);
    let s = TestService::new(&cluster.client, "druid", "broker", 8082, 30082);
    s.conduct_healthcheck(&cluster.client);
    let s = TestService::new(&cluster.client, "druid", "historical", 8083, 30083);
    s.conduct_healthcheck(&cluster.client);
    let s = TestService::new(&cluster.client, "druid", "middleManager", 8091, 30091);
    s.conduct_healthcheck(&cluster.client);
    let s = TestService::new(&cluster.client, "druid", "router", 8888, 30888);
    s.conduct_healthcheck(&cluster.client);

    let delay_time = time::Duration::from_secs(30);
    thread::sleep(delay_time);


    /*
    // Check if the metastore is running on the pod
    for pod in created_pods {
        // extract hostname from port
        let node_name = match &pod.spec.as_ref().unwrap().node_name {
            None => {
                return Err(anyhow!(
                "Missing node_name in pod [{}]. Cannot create host address for metrics port check!",
                pod.metadata.name.as_ref().unwrap(),
            ))
            }
            Some(name) => name,
        };

        println!("Running python healthcheck script ...");
        let status = Command::new("/integration-tests/hive-operator/python/test_metastore.py")
            .args(["-a", node_name])
            .status()
            .expect("Failed to execute healthcheck script.");

        assert!(status.success());
    }
     */

    Ok(())
}

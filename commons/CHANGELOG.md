# Changelog

## [Unreleased]

## [0.6.0] - 2021-10-19

### Changed
- The CDN jsdelivr.net is used instead of githubusercontent.com for the Stackable repository because
  it serves the content with the content type "application/gzip" which is expected by the Stackable
  Agent ([#32]).
- Dependency to `operator-rs` reestablished and reexports for `kube`, `k8s-openapi`, and `schemars`
  used ([#12]).

[#12]: https://github.com/stackabletech/integration-tests/pull/12
[#32]: https://github.com/stackabletech/integration-test-commons/pull/32

## [0.5.0] - 2021-09-22

### Added
- Generic method `list` for resources like pods or configmaps ([#27]).
- `TestClusterLabels` struct for app, instance and version labels ([#27]).
- `instance_name` to `TestClusterOptions` and implemented `new()` where a UUID is automatically appended to the `instance_name` ([#27]).

### Changed
- `kube-rs`: `0.58` → `0.60` ({25}).
- `k8s-openapi`: `version: 0.12, feature: v1_21` → `version: 0.13, feature: v1_22` ([#25]).
- `operator-rs` labels are replaced through the `TestClusterLabels` ([#27]).

### Removed
- `operator-rs` dependency ([#27]).
- `list_pods` and `list_config_maps` methods in favor for the generic `list` ([#27]).

[#25]: https://github.com/stackabletech/integration-test-commons/pull/25
[#27]: https://github.com/stackabletech/integration-test-commons/pull/27

## [0.4.0] - 2021-09-01

### Added
* list_config_maps via label selector ([#21]).
* list_pods `LabelSelector` now has `stackable_operator::labels::APP_INSTANCE_LABEL` for cluster name ([#21]).

### Changed
* `BREAKING [Operator]`: All `TestCluster` "get_" methods renamed to "list_" ([#21]).
* `BREAKING [Operator]`: Renamed `pod_name_label` to `cluster_type` ([#21]).
* `kube-rs`: `0.57` → `0.58` ({15}).
* `tokio`: `1.8` → `1.10` ([#21]).
* Added operator checks for pod version (label) and pod creation timestamps ([#19]).
* Improved operator logging output (messages are prefixed with [<Kind>/<Name>]) ([#19]).
* Sorted `TestCluster` (except `TestCluster::new()`) methods alphabetically ([#21]).

### Fixed
* `test::kube::KubeClient::get_annotation` listens not only on `Added`
  events but also on `Modified` events ([#22]).

[#15]: https://github.com/stackabletech/integration-test-commons/pull/15
[#19]: https://github.com/stackabletech/integration-test-commons/pull/19
[#21]: https://github.com/stackabletech/integration-test-commons/pull/21
[#22]: https://github.com/stackabletech/integration-test-commons/pull/22

## [0.3.0] - 2021-07-13

### Removed
* Kubernetes version feature removed from the k8s-openapi dependency. It
  must be set by the binary crate which uses this library ([#10]).

### Fixed
* Add synchronization to create the integration test repository only once per test file ([#8], [#11]).

### Changed
* `k8s-openapi`: `0.11` → `0.12` ([#9]).
* `kube-rs`: `0.56` → `0.57` ([#9]).
* `tokio`: `1.6` → `1.8` ([#9]).

[#8]: https://github.com/stackabletech/integration-test-commons/pull/8
[#9]: https://github.com/stackabletech/integration-test-commons/pull/9
[#10]: https://github.com/stackabletech/integration-test-commons/pull/10
[#11]: https://github.com/stackabletech/integration-test-commons/pull/11

## [0.2.0] - 2021-06-22

### Added
* TestCluster merged from spark-operator-integration-tests and zookeeper-operator-integration-tests ([#4]).
* `test::kube::KubeClient::verify_status` and `test::kube::KubeClient::get_status` added ([#5]).
* All modules in `k8s_openapi::api::core::v1` re-exported in `test::prelude` ([#5]).

### Fixed
* Race conditions in `test::kube::KubeClient` fixed ([#5]).

### Changed
* `kube-rs`: `0.52` → `0.56` ([#4]).
* `test::kube::Timeouts::verify_pod_condition` renamed to `verify_status` ([#5]).

[#4]: https://github.com/stackabletech/integration-test-commons/pull/4
[#5]: https://github.com/stackabletech/integration-test-commons/pull/5


## [0.1.0] - 2021-06-10

### Added
* Module `test` copied and merged from agent-integration-tests, spark-operator-integration-tests, and zookeeper-operator-integration-tests ([#2]).
* Crate documented ([#2]).
* Unit tests created ([#2]).

### Fixed
* Doctests fixed ([#2]).

### Changed
* `kube-rs`: `0.0 → `0.52` ([#2]).
* `KubeClient::find` renamed to `KubeClient::find_namespaced` ([#2]).
* `KubeClient::find_crd` generalized and renamed to `KubeClient::find` ([#2]).

[#2]: https://github.com/stackabletech/integration-test-commons/pull/2

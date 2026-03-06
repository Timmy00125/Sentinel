# Sentinel Architecture (Rust Workspace)

This document maps the PRD to concrete Rust crates, modules, and trait contracts.

## 1. Workspace Layout

```
sentinel/
  Cargo.toml
  ARCHITECTURE.md
  ROADMAP.md
  PRD.md
  crates/
    sentinel-cli/
    sentinel-core/
    sentinel-config/
    sentinel-policy/
    sentinel-collector/
    sentinel-detection/
    sentinel-decision/
    sentinel-executor/
    sentinel-audit/
    sentinel-telemetry/
    sentinel-integrations-host/
    sentinel-integrations-container/
    sentinel-integrations-k8s/
    sentinel-integrations-lb/
```

## 2. PRD Capability to Crate Mapping

| PRD Area                                                                                             | Rust Crate(s)                     | Notes                                                                                |
| ---------------------------------------------------------------------------------------------------- | --------------------------------- | ------------------------------------------------------------------------------------ |
| CLI command surface (`run`, `doctor`, `integrations`, `policy`, `incidents`, `remediation`, `audit`) | `sentinel-cli`                    | Clap-based command front-end and orchestration wiring.                               |
| Common domain model and contracts                                                                    | `sentinel-core`                   | Shared structs/enums/traits for signals, incidents, actions, policies, integrations. |
| Declarative config loading (`sentinel.yaml`, `policy.yaml`)                                          | `sentinel-config`                 | Serde-backed typed config with mode/policy/integration settings.                     |
| Policy guardrails and deny-by-default model                                                          | `sentinel-policy`                 | `PolicyEngine` implementation and policy checks.                                     |
| Signal collection fan-in                                                                             | `sentinel-collector`              | Multiplexes collectors from enabled integrations.                                    |
| Detection rules and windows                                                                          | `sentinel-detection`              | Executes `DetectionRule` implementations.                                            |
| Incident to remediation mapping and scoring                                                          | `sentinel-decision`               | `DecisionStrategy` implementation and action ranking.                                |
| Action execution orchestration and mode-aware behavior                                               | `sentinel-executor`               | Applies policy check + mode behavior + executor dispatch.                            |
| Auditability and immutable event stream adapters                                                     | `sentinel-audit`                  | `AuditSink` implementations (stdout first, append-only later).                       |
| Metrics and health surfaces                                                                          | `sentinel-telemetry`              | `MetricsSink` implementations and metric naming bridge.                              |
| Host integration (system/service health)                                                             | `sentinel-integrations-host`      | Host read/write adapters and safe restart stubs.                                     |
| Container integration (Docker/containerd abstraction)                                                | `sentinel-integrations-container` | Runtime health and container restart primitives.                                     |
| Kubernetes integration (`k8s-openapi`)                                                               | `sentinel-integrations-k8s`       | Typed API adapter for read/write capabilities.                                       |
| LB/traffic integration                                                                               | `sentinel-integrations-lb`        | Weight-adjust and route control adapter surface.                                     |

## 3. Core Trait Contracts

The following traits are the seam lines for all plugins and runtime behavior.

### 3.1 Data Plane Traits

- `Collector`
  - Role: Pulls normalized `SignalEvent` values from one integration.
  - Key methods: `id`, `integration_type`, `collect`.

- `DetectionRule`
  - Role: Evaluates windows/thresholds and emits incidents.
  - Key methods: `id`, `evaluate`.

- `DecisionStrategy`
  - Role: Produces candidate remediations with confidence/risk.
  - Key methods: `id`, `rank_actions`.

- `ActionExecutor`
  - Role: Executes one action type against one capability target.
  - Key methods: `action_type`, `execute`.

### 3.2 Control Plane Traits

- `PolicyEngine`
  - Role: Final allow/deny gate for suggested/applied actions.
  - Key methods: `evaluate`.

- `IntegrationAdapter`
  - Role: Capability discovery and health reporting for one integration.
  - Key methods: `id`, `integration_type`, `capabilities`, `health_check`.

- `AuditSink`
  - Role: Emits incident/action lifecycle records.
  - Key methods: `write_incident`, `write_action`.

- `MetricsSink`
  - Role: Exposes counters/gauges for detections/actions/integration health.
  - Key methods: `incr_counter`, `set_gauge`.

## 4. Runtime Composition

`sentinel-cli` orchestrates a default pipeline:

1. Load `SentinelConfig` + `PolicyConfig`.
2. Build integration adapters and collector set.
3. Collect signal batch.
4. Run detection rules -> incidents.
5. Run decision strategy -> candidate actions.
6. Run policy checks (`deny-by-default`, thresholds, cooldowns, allowlist/denylist).
7. Depending on mode:
   - `observe`: no execution.
   - `suggest`: emit explain/approval data only.
   - `apply`: execute allowlisted actions.
8. Emit audit and telemetry records for all lifecycle events.

## 5. Module Boundaries and Dependencies

- Dependency direction is inward toward `sentinel-core`:
  - All crates may depend on `sentinel-core`.
  - `sentinel-cli` depends on orchestrating crates and integration crates.
  - Integration crates do not depend on each other.
  - `sentinel-policy` depends on `sentinel-config` + `sentinel-core` only.
- This keeps policy, detection, and execution testable and swappable.

## 6. Kubernetes Integration Plan (`k8s-openapi`)

- Keep K8s specifics isolated in `sentinel-integrations-k8s`.
- Use typed API structures from `k8s-openapi` for read/write operations.
- Expose only normalized `SignalEvent`, `IntegrationStatus`, and action handlers to core.
- Add a `doctor` compatibility check that validates:
  - cluster API access,
  - RBAC permissions for read/write operations,
  - supported version range (N-2).

## 7. Safety and Governance by Design

- Deny-by-default policy logic in `sentinel-policy`.
- Mode-aware execution in `sentinel-executor`.
- Auditable records in `sentinel-audit` before and after each action.
- Circuit breaker and anti-thrashing controls represented in `PolicyConfig` and enforced by `PolicyEngine`.

## 8. Immediate Next Implementation Steps

1. Implement config schema and file validation in `sentinel-config`.
2. Add first detection rule set for crash-loop, memory pressure, and disk pressure.
3. Wire `sentinel run` end-to-end in `suggest` mode with stub integrations.
4. Add policy tests for confidence thresholds, cooldown, and denied targets.
5. Replace Kubernetes placeholders with real `k8s-openapi` client logic.

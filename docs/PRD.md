# PRD: Autonomous DevOps "Sentinel"

## 1. Document Control

- Product Name: Sentinel
- Version: v0.2 (Platform-Agnostic Rewrite)
- Date: 2026-03-06
- Status: Draft
- Owner: Platform Engineering
- Audience: Engineering, SRE/DevOps, Security, Product, IT Operations

## 2. Executive Summary

Sentinel is a Rust-based autonomous DevOps CLI agent that monitors system health, logs, and resource usage across multiple environments, then suggests or safely applies remediation actions.

Sentinel is not Kubernetes-only. It supports host-level operations, container runtimes, load balancers, cloud infrastructure APIs, and Kubernetes clusters through a modular integration model. Kubernetes remains a key feature, with direct typed API access through `k8s-openapi`.

Sentinel is built for small teams managing complex infrastructure who need a low-footprint, always-on operational assistant that reduces repetitive incident work.

## 3. Problem Statement

Small teams increasingly run mixed infrastructure: VMs, containers, Kubernetes, cloud services, and network edges. Incident handling is fragmented across dashboards and manual runbooks, leading to:

- High cognitive load and alert fatigue.
- Slow triage and slow remediation.
- Inconsistent incident response quality.
- Increased risk from manual fixes under pressure.

## 4. Product Vision

Build a portable autonomous operations agent that continuously detects known failure patterns across environments and executes policy-governed remediations with auditability and human control.

## 5. Goals and Non-Goals

### 5.1 Goals (MVP)

- Reduce MTTD and MTTR for common infra and runtime incidents.
- Provide confidence-scored remediation suggestions with explainable evidence.
- Support optional auto-apply mode under strict safety constraints.
- Run with low memory and CPU overhead using Rust.
- Deliver a pluggable integration model for host, container, load balancer, cloud, and Kubernetes targets.
- Support first-class Kubernetes actions via `k8s-openapi`.

### 5.2 Non-Goals (MVP)

- General-purpose AI troubleshooting for unknown incident categories.
- Automatic application code changes.
- Full infrastructure provisioning/orchestration replacement (Terraform, Ansible, GitOps tools).
- Replacing observability products (Prometheus, Grafana, Loki, Datadog, etc.).

## 6. Target Users

- Primary: Small DevOps/platform teams (1-8 engineers) handling mixed infrastructure.
- Secondary: Startup engineering teams sharing on-call duties.
- Tertiary: SRE teams wanting lightweight autonomous runbook execution at the edge.

## 7. User Stories

- As an on-call engineer, I want Sentinel to detect memory leak symptoms and recommend the safest fix quickly.
- As an ops lead, I want policy-scoped auto-remediation so repetitive incidents are handled without waking humans.
- As a security reviewer, I want full action audit logs and deny-by-default permissions.
- As a platform engineer, I want one tool that can act on Linux services, containers, cloud resources, and Kubernetes.

## 8. Key Value Proposition

- Reduces operational overhead by automating repetitive runbook steps.
- Improves remediation consistency with policy-driven actions.
- Portable and low footprint due to Rust.
- Works across environments, not just Kubernetes.
- Includes deep Kubernetes support through `k8s-openapi`.

## 9. Product Scope

### 9.1 In Scope (MVP)

- Signal collection:
  - Host metrics (CPU, memory, disk, load average, process pressure).
  - Host/service health (systemd service state, restart loops).
  - Container runtime health (Docker/containerd where available).
  - Kubernetes workload and node health.
  - Log signature detection from configured sources.
  - Optional metrics backends (Prometheus-compatible pull/query).
- Remediation modes:
  - `observe`: detect and report only.
  - `suggest`: recommend actions with evidence and confidence.
  - `apply`: execute policy-authorized actions.
- Remediation primitives:
  - Restart unhealthy service/container/workload.
  - Apply bounded scaling changes where supported.
  - Adjust load balancer target weights where supported.
  - Quarantine noisy workload/instance via labels, drains, or route removal (integration-dependent).
- Governance and visibility:
  - Structured event logs.
  - Action approvals (manual/automated policy gate).
  - Decision traceability.

### 9.2 Out of Scope (MVP)

- Autonomous schema/database migrations.
- Cost optimization and rightsizing recommendations across all cloud services.
- Fully autonomous multi-region failover orchestration.

## 10. Platform and Integration Model

Sentinel is built around adapters and capabilities:

- Core engine: detection, decisioning, safety policy, execution orchestration.
- Integrations: each target environment exposes read and write capabilities.
- Capability-based actions: an action executes only if integration and policy both allow it.

### 10.1 MVP Integrations

- Linux host integration (process/service checks and safe restart hooks).
- Container runtime integration (Docker or containerd).
- Kubernetes integration using `k8s-openapi`.
- Load balancer/traffic integration via configurable adapters (for example NGINX/Traefik/API-driven LB).
- Notification integrations (Slack/Webhook) for suggestions and applied actions.

### 10.2 Post-MVP Integrations

- Cloud-native integrations (AWS/GCP/Azure service APIs).
- Incident management integrations (PagerDuty/Opsgenie/Jira).
- Service mesh and advanced traffic management adapters.

## 11. Product Requirements

### 11.1 Functional Requirements

1. Sentinel must ingest health, resource, and log signals from enabled integrations.
2. Sentinel must evaluate rule-based detections on configurable intervals/windows.
3. Sentinel must generate incident objects with:
   - Trigger evidence.
   - Impact estimate.
   - Suggested remediation options.
   - Risk level and confidence score.
4. Sentinel must enforce policy checks before any action suggestion or execution.
5. Sentinel must support manual approval workflows in `suggest` mode.
6. Sentinel must execute only allowlisted actions in `apply` mode.
7. Sentinel must expose integration health and capability status.
8. Sentinel must support direct Kubernetes interactions through `k8s-openapi`.
9. Sentinel must emit structured logs and metrics for all detections and actions.
10. Sentinel must provide CLI commands for run, validate, explain, approve, and audit workflows.

### 11.2 Non-Functional Requirements

- Performance:
  - Memory target: <= 150 MB steady state for typical mixed-node deployment.
  - CPU target: <= 300m average during normal operation.
- Reliability:
  - Sentinel failure must not cause workload outage.
  - Actions should be idempotent when possible.
- Security:
  - Least privilege for each integration.
  - Signed binaries/images and integrity checks.
  - Secret material must not be stored in plaintext.
- Compliance:
  - Exportable immutable audit trail.
  - Action-level actor/mode/policy context retained.

## 12. CLI Requirements

### 12.1 Command Surface (MVP)

- `sentinel run --mode observe|suggest|apply --config ./sentinel.yaml`
- `sentinel doctor --config ./sentinel.yaml`
- `sentinel integrations list --config ./sentinel.yaml`
- `sentinel policy validate --policy ./policy.yaml`
- `sentinel incidents list --since 24h`
- `sentinel remediation explain --incident <id>`
- `sentinel remediation approve --incident <id> --action <action_id>`
- `sentinel audit tail --since 1h`

### 12.2 CLI Behavior

- Human-readable output by default; JSON output for automation.
- Stable exit codes for CI and script integration.
- Support for in-cluster, host-local, and remote API credentials.

## 13. Architecture

### 13.1 High-Level Components

- Collector Layer:
  - Pulls host/container/Kubernetes/log/metrics signals.
- Detection Engine:
  - Evaluates rule sets and temporal thresholds.
- Decision Engine:
  - Maps incidents to candidate actions.
  - Scores confidence and risk.
- Policy Engine:
  - Applies allow/deny, scope, cooldown, and blast-radius controls.
- Executor:
  - Invokes integration-specific action handlers.
- Audit and Telemetry:
  - Emits event logs and operational metrics.

### 13.2 Data Flow

1. Collect signals from enabled integrations.
2. Normalize into common event model.
3. Detect incidents using rules/windows.
4. Generate candidate remediations.
5. Enforce policy and safety checks.
6. Suggest or apply action based on mode.
7. Record full lifecycle in audit stream.

## 14. Kubernetes Integration (`k8s-openapi`)

### 14.1 Requirement

Sentinel must use `k8s-openapi` for typed Kubernetes API access, avoiding shell command wrappers where possible.

### 14.2 MVP Kubernetes Capabilities

- Read:
  - Pods, Deployments, ReplicaSets, Nodes, Events.
- Write:
  - Rolling restart via patching workload metadata.
  - Bounded scaling within policy limits.
  - Traffic-weight related resource patching when mapped by integration adapter.

### 14.3 Compatibility

- Target Kubernetes support: N-2 minor versions.
- `sentinel doctor` must validate API compatibility and permissions at startup.

## 15. Incident and Remediation Catalog (MVP)

1. Host memory pressure and process leak:
   - Signal: sustained memory growth plus process/log indicators.
   - Action: controlled service restart and optional temporary traffic reduction.
2. Container restart storms:
   - Signal: repeated container exits with known transient signatures.
   - Action: bounded restart strategy with cooldown.
3. Kubernetes crash loops:
   - Signal: CrashLoopBackOff plus matching error pattern.
   - Action: rollout restart or bounded scale change.
4. Traffic imbalance:
   - Signal: backend error/latency skew.
   - Action: adjust load balancer weights within configured limits.
5. Disk pressure:
   - Signal: low disk watermark plus log growth anomalies.
   - Action: rotate cleanup workflow (policy-gated) and notify operator.

## 16. Autonomy and Safety Model

### 16.1 Modes

- `observe`: no remediations.
- `suggest`: proposals only; optional approval gates.
- `apply`: autonomous execution for allowlisted action classes.

### 16.2 Guardrails

- Deny-by-default action policy.
- Per-action confidence thresholds.
- Max actions per target per time window.
- Global cooldowns and anti-thrashing controls.
- Circuit breaker that auto-downgrades from `apply` to `suggest` after repeated failures.

### 16.3 Human Control

- Pause/resume execution commands.
- Emergency kill switch.
- Mandatory explainability output for each suggested/applied action.

## 17. Configuration Requirements

- Declarative config (`sentinel.yaml`) for:
  - Integrations and credentials references.
  - Detection rules and thresholds.
  - Remediation policies and limits.
  - Notification and output sinks.
  - Data retention settings.
- Policy file (`policy.yaml`) for environment-specific safety boundaries.

## 18. Security and Governance

- Least-privilege model per integration.
- Separate policy profiles by environment (dev/staging/prod).
- Signed artifacts and provenance metadata.
- Immutable audit sink option for compliance-sensitive environments.
- Secrets via env vars, external secret stores, or orchestrator-native secret systems.

## 19. Observability Requirements

- Metrics:
  - `sentinel_incidents_detected_total`
  - `sentinel_suggestions_generated_total`
  - `sentinel_actions_applied_total`
  - `sentinel_actions_blocked_total`
  - `sentinel_action_failures_total`
  - `sentinel_integrations_unhealthy_total`
- Logs:
  - Correlate with `incident_id`, `action_id`, `target_id`, `integration_id`.
- Health surfaces:
  - CLI `doctor` command.
  - Optional `/healthz` and `/readyz` for daemonized deployments.

## 20. Success Metrics

### 20.1 Product Metrics

- At least 40% reduction in manual remediation steps for covered incidents.
- At least 30% MTTR improvement for covered incidents.
- At least 75% operator acceptance of suggestions after tuning period.

### 20.2 Technical Metrics

- False-positive recommendation rate below 12% after baseline tuning.
- Action success rate at least 95% for low-risk allowlisted actions.
- Crash-free runtime at least 99.9% in pilot.

## 21. Rollout Plan

### Phase 0: Foundation

- Implement core engine, policy model, and audit framework.
- Enable host and read-only integration checks.

### Phase 1: Suggest Mode Pilot

- Enable host, container, Kubernetes, and LB read paths.
- Validate recommendation quality with no autonomous writes.

### Phase 2: Controlled Apply

- Enable low-risk auto-actions for selected integrations.
- Enforce strict blast-radius and circuit-breaker policies.

### Phase 3: Expansion

- Add cloud API integrations and richer remediation catalog.
- Tune confidence models and incident routing.

## 22. Testing and Validation

- Unit tests:
  - Rule evaluation, policy enforcement, confidence calculations.
- Integration tests:
  - Host/container/Kubernetes/LB adapters with mocked and real test targets.
- Chaos tests:
  - Leak, crash-loop, latency skew, and disk pressure scenarios.
- Safety tests:
  - Verify blocked actions remain blocked in all modes.

## 23. Risks and Mitigations

- Risk: Wrong autonomous action degrades service.
  - Mitigation: deny-by-default policy, confidence gates, cooldowns, and circuit breaker.
- Risk: Integration drift across platforms.
  - Mitigation: capability discovery, adapter versioning, and `doctor` validation.
- Risk: Noisy telemetry increases false positives.
  - Mitigation: multi-signal correlation and debounce windows.
- Risk: Excessive permissions.
  - Mitigation: scoped credentials and environment-specific policy packs.

## 24. Dependencies

- Rust toolchain and async runtime (`tokio`).
- Integration SDKs/clients for enabled targets.
- `k8s-openapi` crate for Kubernetes support.
- Optional observability and notification backends.

## 25. Open Questions

1. Which non-Kubernetes integrations are mandatory for MVP in your environment (systemd, Docker, cloud LB API, etc.)?
2. Which load balancer implementations must be supported first?
3. Should medium-risk actions require two-step approvals in production?
4. What audit retention period is required by policy/compliance?
5. Is Sentinel deployed primarily as host daemon, sidecar, or both?

## 26. Acceptance Criteria (MVP Exit)

1. Sentinel runs with documented resource envelope and does not impact application availability.
2. Sentinel detects and explains covered incidents across at least three integration domains (for example host, container, and Kubernetes).
3. Sentinel provides policy-compliant suggestions with confidence and evidence.
4. Sentinel autonomously applies at least two low-risk remediations in controlled environments.
5. All actions are fully auditable with end-to-end correlation.
6. Kubernetes interactions are implemented directly via `k8s-openapi`.

## 27. Appendix: Example Policy Constraints

- `max_actions_per_hour: 4`
- `max_actions_per_target_per_hour: 2`
- `allowed_actions: [service_restart, container_restart, rolling_restart, bounded_scale_up, lb_weight_adjust]`
- `denied_targets: [prod-db-primary, kube-system/*]`
- `confidence_thresholds:`
  - `service_restart: 0.70`
  - `rolling_restart: 0.75`
  - `lb_weight_adjust: 0.85`
- `global_cooldown_seconds: 600`
- `auto_downgrade_after_failures: 3`

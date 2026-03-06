# Sentinel Build Tasks

This checklist translates the PRD and roadmap into executable tasks.
Mark each item complete as work is finished.

## How to Use

- Task ID format: `S{Sprint}-{Number}` for sprint tasks, `X-{Number}` for cross-sprint tracks, `MVP-{Number}` for exit checks.
- Keep task descriptions objective-focused: build, verify, and document outcomes.
- If a task changes scope, update this file with the new acceptance objective before implementing.

## Sprint 1 - Foundation A

- [ ] `S1-01` Workspace Baseline: confirm Rust workspace boundaries and crate dependency policy are final and documented in `ARCHITECTURE.md`.
- [ ] `S1-02` Core Domain Models: implement and validate `SignalEvent`, `Incident`, `CandidateAction`, and `ActionResult` in `sentinel-core`.
- [ ] `S1-03` Core Trait Contracts: define and compile-check traits for collector, detection, decision, policy, executor, audit, and telemetry seams.
- [ ] `S1-04` Config Schema: implement typed `sentinel.yaml` and `policy.yaml` schema loading and validation in `sentinel-config`.
- [ ] `S1-05` Example Configs: add sample config files that load successfully and cover required MVP fields.
- [ ] `S1-06` Logging/Tracing Baseline: establish structured logging conventions with correlation fields (`incident_id`, `action_id`, `target_id`, `integration_id`).
- [ ] `S1-07` Foundation Validation: run `cargo fmt`, `cargo clippy`, and `cargo test`; verify workspace builds cleanly.
- [ ] `S1-08` Contract Review Package: document core contracts and config expectations for platform + SRE review.

## Sprint 2 - Foundation B

- [ ] `S2-01` Policy Engine Core: implement deny-by-default policy evaluation flow in `sentinel-policy`.
- [ ] `S2-02` Policy Rules: add allowlist/denylist target checks, confidence thresholds, mode gates, and cooldown logic.
- [ ] `S2-03` Action Rate Limits: implement global and per-target action throttling constraints.
- [ ] `S2-04` Audit Interfaces: define audit sink interfaces and event payload contracts for incident/action lifecycle records.
- [ ] `S2-05` Local Append-Only Audit Sink: implement first durable sink adapter for append-only local audit logs.
- [ ] `S2-06` Doctor Command Skeleton: scaffold `sentinel doctor` CLI flow and integration capability status reporting.
- [ ] `S2-07` CI Pipeline: add CI jobs for formatting, linting, tests, and clippy enforcement.
- [ ] `S2-08` Policy Test Matrix: add positive/negative tests covering blocked and allowed action scenarios.
- [ ] `S2-09` Doctor Output UX: ensure doctor output is actionable and maps failures to suggested next steps.

## Sprint 3 - Suggest Pilot A

- [ ] `S3-01` Host Read Adapter: implement host signal collection (CPU, memory, disk, service status) in `sentinel-integrations-host`.
- [ ] `S3-02` Container Read Adapter: implement container runtime health/restart read path in `sentinel-integrations-container`.
- [ ] `S3-03` Kubernetes Read Adapter: implement pod/deployment/node/event read path using `k8s-openapi` in `sentinel-integrations-k8s`.
- [ ] `S3-04` Signal Normalization: map integration outputs into unified `SignalEvent` model.
- [ ] `S3-05` Collector Fan-In: build collector scheduler loop and fan-in pipeline in `sentinel-collector`.
- [ ] `S3-06` Integration Health Surfaces: expose integration capability + health via CLI and internal status model.
- [ ] `S3-07` Read-Path Integration Tests: validate normalized stream correctness across at least three domains.

## Sprint 4 - Suggest Pilot B

- [ ] `S4-01` Incident Rule - Memory Pressure: implement sustained memory + leak-indicator detection rule.
- [ ] `S4-02` Incident Rule - Restart Storm: implement container restart storm detection rule.
- [ ] `S4-03` Incident Rule - K8s Crash Loop: implement CrashLoopBackOff correlation rule.
- [ ] `S4-04` Incident Rule - Disk Pressure: implement disk watermark + growth anomaly rule.
- [ ] `S4-05` Decision Engine: map incidents to candidate remediations with confidence and risk scoring.
- [ ] `S4-06` Incidents List CLI: implement `sentinel incidents list --since <window>` command.
- [ ] `S4-07` Remediation Explain CLI: implement `sentinel remediation explain --incident <id>` with evidence output.
- [ ] `S4-08` Notification Integrations: add Slack/Webhook suggestion notifications.
- [ ] `S4-09` Suggest-Mode E2E: verify full suggest-mode flow with no write actions executed.

## Sprint 5 - Controlled Apply A

- [ ] `S5-01` Host Service Restart Executor: implement low-risk service restart action.
- [ ] `S5-02` Container Restart Executor: implement low-risk container restart action.
- [ ] `S5-03` K8s Rolling Restart Executor: implement rollout restart action through typed Kubernetes patch flow.
- [ ] `S5-04` Approval Workflow CLI: implement `sentinel remediation approve --incident <id> --action <action_id>`.
- [ ] `S5-05` Pre-Action Audit: emit mandatory audit record before action execution.
- [ ] `S5-06` Post-Action Audit: emit mandatory audit record for success/failure result.
- [ ] `S5-07` Mode Enforcement: verify apply-mode only executes policy-authorized actions.
- [ ] `S5-08` Block Reason Explainability: ensure blocked actions return clear policy rationale in CLI and audit.

## Sprint 6 - Controlled Apply B

- [ ] `S6-01` Bounded Scaling Executor: implement bounded scale up/down action with policy limits.
- [ ] `S6-02` LB Weight Adapter: implement first load balancer weight adjustment executor.
- [ ] `S6-03` Circuit Breaker: implement automatic downgrade from `apply` to `suggest` after repeated failures.
- [ ] `S6-04` Audit Tail CLI: implement `sentinel audit tail --since <window>` with human and JSON output.
- [ ] `S6-05` JSON Output Parity: ensure automation-safe JSON output exists for core CLI workflows.
- [ ] `S6-06` Action Safety Tests: expand integration/safety tests for all enabled action types.
- [ ] `S6-07` Autonomous Low-Risk Validation: demonstrate at least two low-risk remediations in controlled environment.

## Sprint 7 - Expansion A

- [ ] `S7-01` First Cloud Adapter: implement first cloud API integration selected by team priority.
- [ ] `S7-02` Remediation Catalog Expansion: add higher-coverage incident-to-action mappings.
- [ ] `S7-03` Confidence Tuning Hooks: add configurable score tuning controls.
- [ ] `S7-04` Multi-Signal Correlation: improve debounce/correlation windows to reduce false positives.
- [ ] `S7-05` Daemon Health Endpoints: implement `/healthz` and `/readyz` for daemon mode.
- [ ] `S7-06` Pilot Readiness Verification: validate new integration for production pilot readiness.

## Sprint 8 - Expansion B / MVP Exit

- [ ] `S8-01` Reliability Hardening: address crash recovery, retry behavior, and graceful degradation.
- [ ] `S8-02` Performance Envelope Validation: measure memory and CPU against PRD targets.
- [ ] `S8-03` Chaos/Safety Suite: run leak, crash-loop, latency skew, and disk pressure scenarios.
- [ ] `S8-04` Action Success and Failure Metrics: validate telemetry coverage and reliability KPIs.
- [ ] `S8-05` Operator Runbook: publish operational runbook for deployment and on-call usage.
- [ ] `S8-06` Policy Profile Templates: provide baseline dev/staging/prod policy templates.
- [ ] `S8-07` Deployment Docs: finalize setup and integration docs for supported environments.
- [ ] `S8-08` MVP Exit Review Package: prepare evidence bundle for leadership sign-off.

## Cross-Sprint Security/Compliance Track

- [ ] `X-01` Least-Privilege Reviews: validate minimal permissions for each integration.
- [ ] `X-02` Secret Handling: enforce non-plaintext secrets strategy and validate references.
- [ ] `X-03` Artifact Integrity: define and implement binary/image signing and provenance flow.
- [ ] `X-04` Immutable Audit Option: design and verify immutable audit sink path for compliance.

## Cross-Sprint Observability/Quality Track

- [ ] `X-05` Required Metrics: instrument all PRD-required counters/gauges and verify naming.
- [ ] `X-06` Correlation IDs: enforce lifecycle correlation in logs and audit records.
- [ ] `X-07` Test Pyramid Growth: maintain unit, integration, chaos, and safety coverage targets.
- [ ] `X-08` False-Positive Monitoring: track recommendation quality and tune rules over time.
- [ ] `X-09` Regression Gates: prevent merges on failing lint/test/critical integration checks.

## MVP Exit Checklist Tasks

- [ ] `MVP-01` Resource Envelope Pass: prove <=150 MB memory and <=300m average CPU in representative workload.
- [ ] `MVP-02` Multi-Domain Detection Pass: detect covered incidents across at least three integration domains.
- [ ] `MVP-03` Explainable Suggestions Pass: provide evidence + confidence + policy context for suggestions.
- [ ] `MVP-04` Autonomous Action Pass: verify two or more low-risk actions in controlled apply mode.
- [ ] `MVP-05` Full Auditability Pass: confirm end-to-end auditable action lifecycle records.
- [ ] `MVP-06` Kubernetes Typed API Pass: verify Kubernetes interactions use `k8s-openapi` directly.

## Backlog / Open Questions to Resolve

- [ ] `B-01` MVP Mandatory Integrations: decide exact non-Kubernetes integration set for MVP.
- [ ] `B-02` LB Priority: select first load balancer implementation target.
- [ ] `B-03` Approval Policy: decide if medium-risk production actions require two-step approvals.
- [ ] `B-04` Audit Retention: define required retention and archival policy.
- [ ] `B-05` Deployment Topology: decide host daemon, sidecar, or mixed deployment standard.

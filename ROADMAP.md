# Sentinel Delivery Roadmap

This roadmap derives directly from PRD phases (0-3) and breaks them into executable sprint slices.

Assumption: 2-week sprints, single cross-functional platform squad.

## Sprint 1 (Phase 0: Foundation A)

- Initialize Rust workspace, crate boundaries, and dependency policy.
- Define core domain model (`SignalEvent`, `Incident`, `CandidateAction`, `ActionResult`).
- Define core traits for collector/detection/decision/policy/executor/audit.
- Implement base config schema (`sentinel.yaml`, `policy.yaml`) and parser.
- Establish structured logging baseline and tracing conventions.

Definition of done:

- Workspace builds cleanly.
- Config loads and validates with example files.
- Core contracts reviewed and accepted by platform and SRE.

## Sprint 2 (Phase 0: Foundation B)

- Implement policy engine with deny-by-default behavior.
- Add policy checks for allowlist/denylist, confidence threshold, and mode gates.
- Create audit sink interfaces and local append-only sink implementation.
- Build `sentinel doctor` command skeleton with integration capability reporting.
- Add CI (format, lint, unit tests, clippy).

Definition of done:

- Policy tests cover positive/negative paths.
- `sentinel doctor` returns actionable status output.

## Sprint 3 (Phase 1: Suggest Pilot A)

- Implement host integration read-path (CPU/memory/disk/service status).
- Implement container integration read-path (restart counts/state/events).
- Implement Kubernetes read-path (pods/deployments/nodes/events) via `k8s-openapi`.
- Normalize signals into common event model.
- Implement collector fan-in and scheduling loop.

Definition of done:

- Signals from at least 3 domains available in unified stream.
- Integration health visible through CLI.

## Sprint 4 (Phase 1: Suggest Pilot B)

- Implement first incident catalog rules:
  - memory pressure + leak indicators,
  - container restart storm,
  - Kubernetes crash loops,
  - disk pressure.
- Build decision engine for candidate remediations with confidence and risk.
- Implement `sentinel incidents list` and `sentinel remediation explain`.
- Add Slack/Webhook notifications for suggestions.

Definition of done:

- End-to-end `suggest` mode works without write actions.
- Evidence and confidence are shown for each recommendation.

## Sprint 5 (Phase 2: Controlled Apply A)

- Implement low-risk action executors:
  - service restart (host),
  - container restart,
  - rolling restart (Kubernetes).
- Add cooldown, max-action-rate, and per-target throttle constraints.
- Implement approval workflow command (`remediation approve`).
- Add mandatory pre-action and post-action audit records.

Definition of done:

- Approved actions execute only when policy allows.
- Blocked actions are clearly explained and audited.

## Sprint 6 (Phase 2: Controlled Apply B)

- Implement bounded scaling action with policy guardrails.
- Implement LB weight adjustment adapter (first target implementation).
- Add circuit-breaker logic (auto-downgrade `apply` -> `suggest` after repeated failures).
- Add `sentinel audit tail` and JSON output support for automation.
- Expand integration and safety tests for all enabled action types.

Definition of done:

- At least two low-risk remediations can run autonomously in controlled env.
- Circuit breaker verified in failure simulation.

## Sprint 7 (Phase 3: Expansion A)

- Add first cloud API adapter (team-priority target).
- Add richer remediation catalog entries and confidence tuning hooks.
- Improve incident correlation with multi-signal debounce windows.
- Add `/healthz` and `/readyz` for daemon mode.

Definition of done:

- One new integration beyond initial domains is production-pilot ready.
- False-positive rate trending toward PRD target.

## Sprint 8 (Phase 3: Expansion B / MVP Exit)

- Harden reliability, performance, and crash recovery.
- Run chaos and safety validation suite (leak, crash-loop, latency skew, disk pressure).
- Validate resource envelope and action success metrics.
- Prepare operator runbook, policy profile templates, and deployment docs.
- Formal MVP exit assessment against acceptance criteria.

Definition of done:

- MVP exit criteria pass with measurable evidence.
- Rollout decision package ready for leadership sign-off.

## Cross-Sprint Tracks

- Security and compliance:
  - least-privilege reviews per integration,
  - secret handling checks,
  - artifact signing/provenance setup.
- Observability:
  - maintain required metrics and correlation IDs,
  - track false positives and action outcomes.
- Quality:
  - maintain unit/integration/chaos/safety test pyramid.

## MVP Exit Checklist (From PRD)

- Runs within memory/CPU envelope.
- Detects covered incidents across at least three integration domains.
- Suggests policy-compliant remediations with evidence/confidence.
- Autonomously applies at least two low-risk remediations in controlled environments.
- Produces complete audit trail for each action lifecycle.
- Uses direct Kubernetes interaction through `k8s-openapi`.

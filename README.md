# Sentinel

Sentinel is a lightweight, Rust‑based autonomous DevOps agent and CLI designed to monitor heterogeneous infrastructure, detect common operational issues, and either suggest or safely apply remediation actions. It is built for small teams managing mixed environments (hosts, containers, Kubernetes, load balancers, cloud resources) who want a low‑footprint, policy‑governed assistant to reduce manual toil and improve incident response consistency.

---

## 🚀 Key Features

- **Multi‑environment** support: Linux hosts, container runtimes (Docker/containerd), Kubernetes clusters, and pluggable load‑balancer APIs.
- **Mode of operation**:
  - `observe` – collect signals and report only.
  - `suggest` – recommend remediation options with evidence and confidence scores.
  - `apply` – execute policy‑authorized actions automatically.
- **Policy‑driven governance**: deny‑by‑default, scoped allow/deny lists, cooldowns, blast‑radius limits.
- **Auditability**: immutable event stream logging, action approvals, and decision traceability.
- **Extensible architecture**: plugin‑friendly crates for collectors, detectors, decision strategies, and executors.
- **Deep Kubernetes integration** via `k8s-openapi` with typed API access.
- **CLI commands** for diagnostics (`doctor`), policy validation, incident browsing, and manual approvals.

---

## 📦 Workspace Layout

```
crates/
  sentinel-cli/          # Clap-based command line front end
  sentinel-core/         # Shared domain models & traits
  sentinel-config/       # Config loading and validation
  sentinel-policy/       # Policy engine and guards
  sentinel-collector/    # Signal fan‑in
  sentinel-detection/    # Detection rules
  sentinel-decision/     # Incident→action mapping
  sentinel-executor/     # Mode‑aware execution orchestration
  sentinel-audit/        # Audit sink implementations
  sentinel-telemetry/    # Metrics and health
  sentinel-integrations-*# Host, container, k8s, LB adapters
```

Refer to `docs/ARCHITECTURE.md` for a detailed mapping of PRD requirements to crates.

---

## 🛠 Installation & Build

Sentinel is developed as a Cargo workspace. To build the entire project:

```bash
cd /path/to/Sentinel
cargo check          # verify dependencies per crate
cargo build --release # produce optimized binaries
```

Each crate can be built/checked individually, which is useful during development.

> **Tip:** After adding a dependency to one of the workspace crates, run `cargo check` from the root; workspace dependency version is centralized but each crate must opt‑in.

---

## ⚙️ Configuration

Configuration is provided via a YAML file (default `sentinel.yaml`) and policy files
(e.g. `policy.yaml`). Example fields include enabled integrations, collection intervals,
mode (`observe|suggest|apply`), and credentials for external APIs.

The CLI exposes a `policy validate` command to check policy syntax and semantics.

---

## 📋 CLI Commands

```text
sentinel run --mode observe|suggest|apply --config ./sentinel.yaml
sentinel doctor --config ./sentinel.yaml
sentinel integrations list --config ./sentinel.yaml
sentinel policy validate --policy ./policy.yaml
sentinel incidents list --since 24h
sentinel remediation explain --incident <id>
sentinel remediation approve --incident <id> --action <action_id>
sentinel audit tail --since 1h
```

All commands default to human‑readable output; add `--output json` for automation. Stable
exit codes facilitate integration with CI pipelines.

---

## 📐 Architecture Overview

At runtime `sentinel-cli` orchestrates the following pipeline:

1. Load configuration and policy.
2. Assemble integration adapters and collectors.
3. Gather signal events from hosts, containers, Kubernetes, etc.
4. Run detection rules to emit incidents.
5. Map incidents to candidate remediation actions and score them.
6. Run the policy engine (allow/deny, cooldowns, thresholds).
7. Depending on mode, either stop, suggest, or execute allow‑listed actions.
8. Emit audit events and telemetry metrics for every lifecycle event.

For more detail see `docs/ARCHITECTURE.md`.

---

## 📚 Documentation & Roadmap

- **PRD** – `PRD.md` describes the product requirements and goals.
- **Architecture** – `docs/ARCHITECTURE.md` maps those requirements to the Rust workspace.
- **Roadmap** – `docs/ROADMAP.md` lists planned features and post‑MVP integrations.
- **Tasks** – `docs/tasks.md` contains development TODOs and prioritization.

---

## 🤝 Contributing

Contributions are welcome! Please follow standard Rust workspace practices:

1. Fork the repository and create a feature branch.
2. `cargo check` regularly to catch missing per‑crate dependencies.
3. Add tests for new rules, policies, or integration behavior.
4. Run `cargo fmt` and `cargo clippy` before submitting a PR.
5. Update documentation (PRD, architecture, README) as needed.

Refer to the `docs/` folder for ongoing design notes.

---

## 📝 License

[Specify your project's license here – e.g., MIT/Apache‑2.0 dual license]

---

## 🙋 Who Should Use Sentinel?

Sentinel is ideal for small DevOps or platform engineering teams that need an always‑on
assistant to detect and remediate routine infrastructure problems across mixed
environments without adding significant resource overhead.

Start with `sentinel run --mode suggest --config ./sentinel.yaml` in a staging
farm and iterate on detection rules and policies.

---

_For further details, consult the documentation in `docs/` or reach out to the project
maintainers._

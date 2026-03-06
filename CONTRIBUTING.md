# Contributing to Sentinel

Thank you for contributing to Sentinel.

This document explains how to propose changes, how we review them, and which branching strategy we use.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Development Setup](#development-setup)
- [Branching Strategy](#branching-strategy)
- [Commit Guidelines](#commit-guidelines)
- [Pull Request Process](#pull-request-process)
- [Quality Gates](#quality-gates)
- [Review and Merge Policy](#review-and-merge-policy)
- [Security Reporting](#security-reporting)
- [License](#license)

## Code of Conduct

Be respectful, constructive, and professional in all interactions.

## Ways to Contribute

You can help by:

- Reporting bugs
- Proposing features
- Improving detection rules or remediation logic
- Adding tests and benchmarks
- Improving documentation (`README.md`, `PRD.md`, `docs/*`)

Before starting large work, open an issue to discuss scope and design.

## Development Setup

1. Install stable Rust (`rustup`) and ensure toolchain is up to date.
2. Clone the repository and enter the workspace root.
3. Run:

```bash
cargo check --workspace
cargo test --workspace
```

Recommended tools:

- `rustfmt` for formatting
- `clippy` for linting

## Branching Strategy

Sentinel uses a lightweight trunk-based strategy with `main` as the protected branch.

- `main` must always be releasable.
- Never commit directly to `main`.
- Use short-lived branches from `main`.
- Rebase or merge `main` frequently to keep branches current.

Branch naming:

- `feat/<short-description>` for new features
- `fix/<short-description>` for bug fixes
- `docs/<short-description>` for documentation only changes
- `refactor/<short-description>` for behavior-preserving refactors
- `test/<short-description>` for tests and test tooling
- `chore/<short-description>` for maintenance

Examples:

- `feat/k8s-crashloop-rule`
- `fix/policy-cooldown-window`
- `docs/contributing-guide`

## Commit Guidelines

Keep commits focused and atomic.

Use clear, imperative commit messages. Conventional Commits are recommended:

- `feat: add disk pressure detection rule`
- `fix: prevent duplicate action scheduling`
- `docs: update architecture flow`

Good commit hygiene:

- One logical change per commit
- Include tests with behavior changes
- Avoid mixing refactors with feature logic in one commit

## Pull Request Process

1. Create or reference an issue.
2. Create a branch from `main`.
3. Implement the smallest complete change.
4. Add or update tests.
5. Update docs when behavior, config, or CLI surface changes.
6. Run all quality gates locally.
7. Open a PR with a clear description.

PR description should include:

- Problem statement
- Approach and tradeoffs
- Evidence (logs, test output, screenshots for CLI output if relevant)
- Risk and rollback notes

## Quality Gates

Before opening a PR, run:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo check --workspace
```

Workspace dependency note:

- Even when versions are centralized in `[workspace.dependencies]`, each crate must still opt in explicitly in its own `Cargo.toml`.
- After dependency changes, run `cargo check` from the workspace root.

## Review and Merge Policy

- At least one maintainer approval is required.
- PRs must pass CI checks before merge.
- Keep PRs small and reviewable.
- Prefer squash merge to keep history clean unless maintainers request otherwise.

Do not force-push after approvals unless necessary. If you must, explain what changed.

## Security Reporting

Do not open public issues for vulnerabilities.

Report potential security issues privately to maintainers with:

- Impact summary
- Reproduction steps
- Affected components
- Suggested mitigation (if known)

## License

By contributing, you agree that your contributions are licensed under this repository's license.

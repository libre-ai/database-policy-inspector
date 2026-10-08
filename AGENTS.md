# database-policy-inspector Canonical Agent Rules

## Authority

Fail-closed PostgreSQL schema inspection gate, a transverse standalone tool of
the constellation (Rust crate `db-inspect`).
Doctrine lives upstream: https://raw.githubusercontent.com/libre-ai/project-governance/HEAD/AGENTS.md
Local decisions live in `docs/adr/`.

## Boundaries

- Static inspection of supplied SQL files only: no connection to, and no
  mutation of, a live database during inspection.
- Not an ORM nor a migration framework; product schemas live in their own
  repositories.
- Upstream code is tracked explicitly; local patches stay small and temporary.

## Quality gates

Run `cargo fmt --all --check`, `cargo clippy --locked --all-targets --all-features -- -D warnings`
and `cargo test --locked --all-features` before pushing; never hide a red test.

## Agents

- Security > quality > performance > completeness, in that order on conflict.
- Stage files before running tree-walking gates.
- An unparsed statement is an `inspection_integrity` finding (ADR-0002),
  never a silent pass; raw SQL never appears in findings.
- Never commit a machine-local absolute filesystem path.

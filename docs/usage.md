<!-- SPDX-FileCopyrightText: 2026 Libre AI contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Running the inspector

`cargo install --path . --locked` installs the `db-inspect` command using the repository's pinned Rust toolchain.

Run `db-inspect --help` for the full command. Supply a security manifest describing the expected tables and access rules, and a SQL schema file. The examples in `tests/fixtures/pass` and `tests/fixtures/fail` cover accepted and rejected configurations.

Use `--report-json report.json` or `--report-md report.md` to save a report. Use `--inspection-at 2026-09-16T00:00:00Z` for a reproducible fixture run; omit it to evaluate expiring waivers against the current UTC time. Unsupported SQL is reported and may block the selected profile.

Static findings do not replace tests against the database roles and policies used by an application.

## Verify a change

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --locked --all-features
cargo deny check bans licenses sources
cargo llvm-cov --locked --all-features --fail-under-lines 79 --fail-under-functions 77 --json --output-path coverage.json
```

The recovery baseline measured 79.45% line coverage and 77.64% function coverage on macOS with Rust 1.97.0 and cargo-llvm-cov 0.8.7. CI enforces the integer floors above; Linux CI still needs to run on the published code. Branch coverage was not measured. The CLI integration tests exercise success, refusal and invalid input through the compiled executable and verify that inputs remain unchanged.

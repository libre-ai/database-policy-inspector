<!-- SPDX-FileCopyrightText: 2026 Libre AI contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Database Policy Inspector

Inspect PostgreSQL SQL files before applying a migration: find missing access policies, excessive permissions and dangerous operations.

The tool produces a JSON report and can block a CI check. It analyses the supplied files; it does not connect to your database.

## Try it

With Rust installed, from this repository:

```sh
cargo run --locked -- run \
  --manifest tests/fixtures/pass/rls_tenant_policy_ok/manifest.json \
  --schema-dump tests/fixtures/pass/rls_tenant_policy_ok/schema.sql
```

Then adapt the manifest and SQL to your application. Exit codes: `0` accepted, `1` blocking finding, `2` input error.

[Guide](docs/usage.md) · [Français](README.fr.md)

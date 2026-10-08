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

## Project status

<!-- libre-ai:project-status:begin -->
<!-- Section générée depuis project.v1.yaml — ne pas éditer à la main. -->

- Situation actuelle : Recovered source snapshot d27dd3007dceb684585b94702de9f11fa5e8af72 is present. Product tests and CI were not rerun for this documentary integration; historical evidence is not qualification of this tree. No product admission or authority transfer is established. Historical responsibilities are not transferred here; the short-name repository that held them has been retired.
- Maturité : idea
- Exposition : idea
- Confiance : medium
- Preuves vérifiées le : 2026-10-07
- Avancement : Avancement non calculable — périmètre à clarifier

<!-- libre-ai:project-status:end -->

[Guide](docs/usage.md) · [Français](README.fr.md)

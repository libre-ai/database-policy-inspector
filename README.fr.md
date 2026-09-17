<!-- SPDX-FileCopyrightText: 2026 Libre AI contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->

# Database Policy Inspector

Examiner des fichiers SQL PostgreSQL avant d’appliquer une migration : repérer des règles d’accès manquantes, des permissions trop larges ou des opérations dangereuses.

L’outil produit un rapport JSON et peut bloquer une vérification CI. Il analyse les fichiers fournis ; il ne se connecte pas à votre base.

## Essayer

Avec Rust installé, depuis ce dépôt :

```sh
cargo run --locked -- run \
  --manifest tests/fixtures/pass/rls_tenant_policy_ok/manifest.json \
  --schema-dump tests/fixtures/pass/rls_tenant_policy_ok/schema.sql
```

Adaptez ensuite le manifeste et le SQL à votre application. Code de sortie : `0` accepté, `1` contrôle bloquant, `2` erreur d’entrée.

[Guide](docs/usage.md) · [English](README.md)

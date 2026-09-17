// SPDX-FileCopyrightText: 2026 Libre AI contributors
// SPDX-License-Identifier: Apache-2.0

use std::{error::Error, fs, path::Path, process::Command};

fn inspect_fixture(fixture: &str, blocked: bool) -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = format!("tests/fixtures/{fixture}/manifest.json");
    let schema = format!("tests/fixtures/{fixture}/schema.sql");
    let manifest_before = fs::read(root.join(&manifest))?;
    let schema_before = fs::read(root.join(&schema))?;
    let output = Command::new(env!("CARGO_BIN_EXE_db-inspect"))
        .current_dir(root)
        .args([
            "run",
            "--manifest",
            &manifest,
            "--schema-dump",
            &schema,
            "--profile",
            "protected_branch",
            "--inspection-at",
            "2026-09-16T00:00:00Z",
        ])
        .output()?;
    assert_eq!(output.status.code(), Some(i32::from(blocked)));
    assert!(output.stderr.is_empty());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["data"]["summary"]["gate_blocked"], blocked);
    assert_eq!(
        report["data"]["status"],
        if blocked { "failed" } else { "passed" }
    );
    assert_eq!(report["data"]["metrics"]["parser_error_count"], 0);
    assert_eq!(
        report["data"]["scope"]["inspected_at"],
        "2026-09-16T00:00:00Z"
    );
    assert_eq!(report["meta"]["tool"], "db-inspect");
    assert_eq!(fs::read(root.join(manifest))?, manifest_before);
    assert_eq!(fs::read(root.join(schema))?, schema_before);
    Ok(())
}

#[test]
fn cli_accepts_a_valid_policy_without_changing_inputs() -> Result<(), Box<dyn Error>> {
    inspect_fixture("pass/rls_tenant_policy_ok", false)
}

#[test]
fn cli_rejects_excessive_grants_without_changing_inputs() -> Result<(), Box<dyn Error>> {
    inspect_fixture("fail/grant_all_to_app_role", true)
}

#[test]
fn cli_rejects_missing_input() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_db-inspect"))
        .arg("run")
        .output()?;
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8(output.stderr)?.contains("--manifest is required"));
    Ok(())
}

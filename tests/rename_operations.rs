// SPDX-FileCopyrightText: 2026 Libre AI contributors
// SPDX-License-Identifier: Apache-2.0

use std::{collections::BTreeSet, error::Error, fs, path::Path, process::Command};

use serde_json::Value;

const FIXTURE_FILES: [&str; 3] = ["expected-report.json", "manifest.json", "schema.sql"];
const NEW_FIXTURES: [&str; 3] = [
    "fail/rename_column_dangerous",
    "fail/rename_table_dangerous",
    "pass/rename_in_comment_ok",
];

struct Inspection {
    exit_code: Option<i32>,
    report: Value,
    expected: Value,
}

fn read_fixture(dir: &Path) -> Result<Vec<Vec<u8>>, Box<dyn Error>> {
    FIXTURE_FILES
        .iter()
        .map(|name| fs::read(dir.join(name)).map_err(Into::into))
        .collect()
}

/// Runs the compiled binary on a fixture and proves the fixture is left byte-identical.
fn inspect(fixture: &str, profile: &str) -> Result<Inspection, Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let relative = format!("tests/fixtures/{fixture}");
    let dir = root.join(&relative);
    let before = read_fixture(&dir)?;
    let output = Command::new(env!("CARGO_BIN_EXE_db-inspect"))
        .current_dir(root)
        .args([
            "run",
            "--manifest",
            &format!("{relative}/manifest.json"),
            "--schema-dump",
            &format!("{relative}/schema.sql"),
            "--profile",
            profile,
            "--inspection-at",
            "2026-09-16T00:00:00Z",
        ])
        .output()?;
    assert!(output.stderr.is_empty(), "stderr must stay empty");
    assert_eq!(read_fixture(&dir)?, before, "fixture bytes changed");
    Ok(Inspection {
        exit_code: output.status.code(),
        report: serde_json::from_slice(&output.stdout)?,
        expected: serde_json::from_slice(&before[0])?,
    })
}

/// Asserts that every field declared by `expected` holds the same value in `actual`.
/// Objects may carry extra fields in `actual`; arrays must have the same length.
fn assert_declared_fields(expected: &Value, actual: &Value, path: &str) {
    match expected {
        Value::Object(fields) => {
            for (key, value) in fields {
                assert_declared_fields(value, &actual[key], &format!("{path}.{key}"));
            }
        }
        Value::Array(items) => {
            let actual_items = actual.as_array().map(Vec::as_slice).unwrap_or_default();
            assert_eq!(items.len(), actual_items.len(), "length of {path}");
            for (index, (item, actual_item)) in items.iter().zip(actual_items).enumerate() {
                assert_declared_fields(item, actual_item, &format!("{path}[{index}]"));
            }
        }
        scalar => assert_eq!(scalar, actual, "value of {path}"),
    }
}

fn assert_single_blocking_rename(
    fixture: &str,
    rule_id: &str,
    subject_type: &str,
    subject_name: &str,
) -> Result<(), Box<dyn Error>> {
    let run = inspect(fixture, "protected_branch")?;
    assert_eq!(run.exit_code, Some(1));
    let data = &run.report["data"];
    assert_eq!(data["status"], "failed");
    assert_eq!(data["summary"]["gate_blocked"], true);
    assert_eq!(data["summary"]["high"], 1);
    let findings = data["findings"].as_array().ok_or("findings is an array")?;
    assert_eq!(findings.len(), 1);
    let finding = &findings[0];
    assert_eq!(finding["rule_id"], rule_id);
    assert_eq!(finding["category"], "migration_safety");
    assert_eq!(finding["severity"], "high");
    assert_eq!(finding["subject"]["type"], subject_type);
    assert_eq!(finding["subject"]["name"], subject_name);
    assert_eq!(finding["gate"]["blocks"], true);
    assert_declared_fields(&run.expected, &run.report, "report");
    Ok(())
}

#[test]
fn column_rename_blocks_on_protected_branch() -> Result<(), Box<dyn Error>> {
    assert_single_blocking_rename(
        "fail/rename_column_dangerous",
        "RENAME_COLUMN_DANGEROUS",
        "column",
        "public.tasks.title",
    )
}

#[test]
fn table_rename_blocks_on_protected_branch() -> Result<(), Box<dyn Error>> {
    assert_single_blocking_rename(
        "fail/rename_table_dangerous",
        "RENAME_TABLE_DANGEROUS",
        "table",
        "public.tasks",
    )
}

#[test]
fn rename_and_drop_inside_comments_raise_nothing() -> Result<(), Box<dyn Error>> {
    let run = inspect("pass/rename_in_comment_ok", "protected_branch")?;
    assert_eq!(run.exit_code, Some(0));
    let data = &run.report["data"];
    assert_eq!(data["status"], "passed");
    assert_eq!(data["findings"], Value::Array(Vec::new()));
    assert_eq!(data["metrics"]["parser_error_count"], 0);
    assert_declared_fields(&run.expected, &run.report, "report");
    Ok(())
}

#[test]
fn column_rename_only_warns_under_local_profile() -> Result<(), Box<dyn Error>> {
    let run = inspect("fail/rename_column_dangerous", "local")?;
    assert_eq!(run.exit_code, Some(0));
    let data = &run.report["data"];
    assert_eq!(data["summary"]["gate_blocked"], false);
    let findings = data["findings"].as_array().ok_or("findings is an array")?;
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0]["rule_id"], "RENAME_COLUMN_DANGEROUS");
    assert_eq!(findings[0]["gate"]["blocks"], false);
    Ok(())
}

#[test]
fn new_fixtures_are_complete_triplets_listed_in_the_readme() -> Result<(), Box<dyn Error>> {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    let readme = fs::read_to_string(fixtures.join("README.md"))?;
    let expected_files: BTreeSet<String> = FIXTURE_FILES.iter().map(ToString::to_string).collect();
    for fixture in NEW_FIXTURES {
        let row = format!("| `{fixture}` |");
        assert_eq!(
            readme.lines().filter(|line| line.starts_with(&row)).count(),
            1,
            "README row for {fixture}"
        );
        let mut found = BTreeSet::new();
        for entry in fs::read_dir(fixtures.join(fixture))? {
            found.insert(entry?.file_name().to_string_lossy().into_owned());
        }
        assert_eq!(found, expected_files, "files of {fixture}");
    }
    Ok(())
}

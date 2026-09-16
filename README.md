<!-- SPDX-FileCopyrightText: 2026 Libre AI contributors -->
<!-- SPDX-License-Identifier: CC-BY-4.0 -->
<!-- Written for the retained Libre AI portfolio on 2026-09-14; earlier source documents and revisions retain their original licensing. -->

# Libre AI Database Policy Inspector

[Français](README.fr.md)

Database changes can alter who can access data in ways that are difficult to spot during review. This project explores static inspection of PostgreSQL-related files for developers reviewing SQL access policies and migrations. Findings would connect a source location to an explicit rule, helping reviewers focus their investigation.

## Intended uses

- Inspect an SQL access policy before applying a database change.
- Review migration files for patterns covered by a defined rule set.
- Identify unsupported syntax and cases that need further investigation or runtime tests.

## Availability

This repository currently contains documentation only; no installable tool is available.

Explore the [Libre AI project catalogue](https://github.com/libre-ai/.github/blob/main/profile/README.md).

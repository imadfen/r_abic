# ADR-0001 — Record architecture decisions

* **Status:** accepted (2026-10-31)
* **Deciders:** maintainer
* **Supersedes:** —

## Context

`r_abic` is a long-lived library whose behaviour must stay predictable for users coming from PyArabic
while evolving into idiomatic Rust. Decisions that shape the public API, the dependency set, the
licensing story or the testing policy are expensive to reverse and easy to forget.

`AGENTS.md` holds the day-to-day rules; this directory holds the *why* behind decisions that override
or extend those rules (`AGENTS.md` §0.1, precedence item 3).

## Decision

We keep Architecture Decision Records in `docs/adr/`, one file per decision, numbered sequentially:

```text
docs/adr/0001-record-architecture-decisions.md
docs/adr/0002-licensing-and-provenance.md
```

Each ADR uses this template:

```markdown
# ADR-000N — <short title>

* **Status:** proposed | accepted | superseded by ADR-000M | rejected
* **Deciders:** …
* **Supersedes:** …

## Context
What forces are at play; link to issues, benchmarks, upstream sources.

## Decision
What we will do, stated so that a reviewer can check compliance.

## Consequences
What becomes easier, what becomes harder, what is now forbidden.

## Alternatives considered
What we rejected and why.
```

Rules: ADRs are append-only; a decision changes by adding a new ADR that supersedes the old one and by
editing only the old one's `Status` line. Any ADR that relaxes a `P*` principle in `AGENTS.md` must be
cross-referenced from `AGENTS.md` and from the PR description.

## Consequences

* Design intent survives maintainer changes and review churn.
* Every `#[allow]`-style or dependency-level exception has a traceable justification.
* Slight overhead: a new ADR is required for API-breaking, dependency, licensing and scope decisions.

## Alternatives considered

* **Comments in code only** — invisible at review time and lost when files move.
* **A wiki** — drifts away from the code and is not part of code review.
* **A single `DESIGN.md`** — becomes a stale monolith; ADRs stay scoped and immutable.
# ADR-0002 — Licensing and provenance

* **Status:** accepted (2026-10-31)
* **Deciders:** maintainer
* **Supersedes:** —

## Context

`r_abic` is published under `MIT OR Apache-2.0`. Its behavioural reference,
[PyArabic](https://github.com/linuxscout/pyarabic), is licensed under the **GNU GPL**. Copying PyArabic
code, docstrings, tables of literals or test bodies into this repository would make `r_abic` a
derivative work and force a GPL-compatible licence, which contradicts the crate's stated licence and
its goal of being embeddable anywhere (including in closed-source applications).

At the same time, users expect API and behaviour parity, which requires knowing precisely what
upstream does.

## Decision

1. **No verbatim copying.** PyArabic source, its docstrings, its data tables and its test bodies are
   never copied into this repository — not into `src/`, not into `tests/`, not into `docs/`.
2. **Parity is derived from documented behaviour**, from the published PyArabic documentation
   (<https://pyarabic.readthedocs.io/>) and from the Unicode Standard. Algorithms are re-derived and
   re-expressed in idiomatic Rust with our own names, comments, error types and examples.
3. **API names are tracked, not copied.** `docs/PARITY.md` records upstream symbol names so parity
   stays greppable; names and call signatures are facts about an interface, not expressive code.
4. **Dev-only investigation harness.** If a behaviour must be observed directly, an optional harness
   may live in `tools/oracle/` (git-ignored, never referenced from `src/`, never published, never
   vendored). Any test vector that is committed must be independently justified from the Unicode
   Standard or from documented behaviour, and reviewed for provenance.
5. **Attribution.** `README.md` credits PyArabic as the inspiration and names the parity target;
   `docs/DIVERGENCES.md` records intentional differences.
6. Every source file carries `// SPDX-License-Identifier: MIT OR Apache-2.0`.

## Consequences

* We cannot copy a table of literals verbatim; number word lists must be re-derived from Arabic
  grammar (Phase 3) and reviewed.
* Parity work is slower than a mechanical port and needs more tests — this is the intended trade-off.
* Contributors must not paste upstream snippets into issues, PRs or commits; reviewers check this.
* If a task genuinely requires copying upstream code, the agent **stops and asks the maintainer**
  (`CLAUDE.md` §2.1) instead of improvising a licence decision.

## Alternatives considered

* **Relicense `r_abic` as GPL-3.0** — maximal freedom to port code, but blocks the common
  MIT/Apache use cases and defeats the purpose of the crate.
* **A separate `pyarabic-sys`-style binding crate (GPL)** — needs Python or a translated blob, adds a
  dependency, and cannot satisfy `P1`.
* **Clean-room port with a second, isolated team** — disproportionate for a single-maintainer project;
  the documented-behaviour approach already gives an independent derivation.
# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Releases are automated: [`release-plz`](https://release-plz.dev) derives every version, git tag, GitHub
Release and changelog entry from [Conventional Commit](https://www.conventionalcommits.org/) messages
(policy: `CLAUDE.md` §8). **Do not edit released sections by hand** — only `## [Unreleased]` is
hand-editable, and even that is normally unnecessary because the commit messages carry the information.

## [Unreleased]

### Added

- **Automated semantic versioning and releases** (`CLAUDE.md` §8).
  - `.github/workflows/release.yml`: release-plz runs `release` (git tag + GitHub Release once the
    release PR is merged) and `release-pr` (bump `Cargo.toml`/`Cargo.lock`, regenerate this file) on
    every push to `main`.
  - `.github/workflows/conventional-commits.yml`: rejects non-conventional PR titles and commit
    subjects on pull requests and on `main`.
  - `release-plz.toml`: changelog grouping (Keep a Changelog style headings), `release_always = false`,
    `publish = false` (crates.io publishing stays disabled until an ADR enables it), `semver_check =
    false` until the crate is published.
- **Phase 0 foundation.**
  - `Error` / `Result`: dependency-free error enum (`EmptyInput`, `InvalidTashkeelEncoding`,
    `TashkeelLengthMismatch`, `NumberOutOfRange`) with `Display` + `std::error::Error`; the enum is
    `#[non_exhaustive]` so later phases can add variants without a breaking change.
  - `unicode::constants`: every Arabic code point the crate handles, under its PyArabic name
    (letters, ḥarakāt, the three digit sets, typography, lam-alef ligatures, `SIMPLE_LAM_ALEF*`
    sequences, plus `r_abic`-specific formatting marks: ZWJ, ZWNJ, LRM, RLM, ALM).
  - `unicode::tables`: `HARAKAT`, `TASHKEEL`, `SHORTHARAKAT`, `TANWIN`, `NOT_DEF_HARAKA`,
    `LIGUATURES`, `HAMZAT`, `ALEFAT`, `LETTERS`, `WEAK`, `YEHLIKE`, `WAWLIKE`, `TEHLIKE`, `SMALL`,
    `MOON`, `SUN`, `NUMBERS_EAST/WEST/PERS`, and a sorted `ALPHABETIC_ORDER` table with the
    `alphabetic_order()` binary-search lookup.
  - `unicode::classify`: `const fn` predicates (`is_haraka`, `is_tanwin`, `is_short_haraka`,
    `is_tashkeel`, `is_shadda`, `is_sukun`, `is_tatweel`, `is_ligature`, `is_hamza`,
    `is_hamza_above`, `is_hamza_below`, `is_vowel`, `is_arabic_letter`, `is_arabic_block_char`,
    `is_arabic_indic_digit`, `is_extended_arabic_indic_digit`, `is_arabic_digit`, `is_formatting`)
    and whole-`&str` helpers (`has_shadda`, `has_harakat`, `has_tashkeel`, `is_vocalized`,
    `is_arabicrange`, `is_arabicword`).
  - Crate-level lint set: `forbid(unsafe_code)`, `warn(clippy::pedantic)`, deny on
    `unwrap`/`expect`/`panic`/`todo`/`unimplemented`/`dbg_macro`/`print_stdout` (tests exempted).
- Documentation and process scaffolding: `CLAUDE.md` (canonical rules, v1.0.1), `.cursorrules`,
  `.github/copilot-instructions.md`, `docs/PARITY.md`, `docs/DIVERGENCES.md`, `docs/adr/0001`,
  `docs/adr/0002`, CI workflow.

### Changed

- `Cargo.toml`: added `rust-version = "1.85"`, `repository`, `documentation`.

### Notes

- Zero runtime dependencies (`cargo tree -e normal --depth 1` lists only `r_abic`).
- `CLAUDE.md` v1.1.0: added §8 (automated versioning & releases), the enforced commit grammar (§5.2),
  gate G9 (§4.8), red lines 16–19 (§5.4) and §7.1/§7.2 for the new workflows.

<!-- Links -->
[Unreleased]: https://github.com/imadfen/r_abic/commits/main
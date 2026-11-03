# AGENTS.md — Agent & Developer Instructions for `r_abic`

> **Canonical ruleset** (`AGENTS.md`) for how this repository is designed, written, tested and shipped.
> Read this file top-to-bottom before touching code, and re-read the sections relevant to your task
> before declaring the task finished. It is deliberately **agent-neutral**: every agent *and* every human
> reads *this* file (§0.5).
>
> Vendor bindings — condensed copies of the non-negotiables — are `.cursorrules` (Cursor) and
> `.github/copilot-instructions.md` (Copilot). If a binding disagrees with **this** file, this file wins
> and the binding must be fixed in the same change.
>
> Last reviewed: **2026-11-03** · Ruleset version **v1.4.1** · Toolchain in use: `rustc`/`cargo`
> 1.98.0, `clippy` 0.1.98, `rustfmt` 1.9.0-stable · Edition **2024** · Status: **Phase 1 in progress**
> (`error`, `unicode` and `normalize::strip_tashkeel` landed) · **Releases are automated** from Conventional Commits via release-plz
> (§8) — versions, tags and `CHANGELOG.md` are never written by hand.

---

## Table of contents

0. [How to use this document](#0-how-to-use-this-document)
1. [Project identity & goal](#1-project-identity--goal)
2. [Licensing, provenance & dependencies](#2-licensing-provenance--dependencies)
3. [Architecture & code conventions](#3-architecture--code-conventions)
4. [Testing & automation standards (TDD)](#4-testing--automation-standards-tdd)
5. [Workflow & agent execution rules](#5-workflow--agent-execution-rules)
6. [Roadmap & parity tracker](#6-roadmap--parity-tracker)
7. [CI specification](#7-ci-specification)
8. [Versioning & releases (automated)](#8-versioning--releases-automated)
- [Appendix A — Arabic / Unicode quick reference](#appendix-a--arabic--unicode-quick-reference)
- [Appendix B — PyArabic → `r_abic` parity checklist](#appendix-b--pyarabic--r_abic-parity-checklist)
- [Appendix C — Glossary](#appendix-c--glossary)
- [Appendix D — Command cheat sheet](#appendix-d--command-cheat-sheet)
- [Appendix E — Changelog of this document](#appendix-e--changelog-of-this-document)

---

## 0. How to use this document

### 0.1 Precedence order

When rules conflict, resolve in this order (highest first):

1. **The user's explicit instruction in the current message.**
2. **This file (`AGENTS.md`)** — including its principles (`P1`…`P12`), cited by ID in reviews.
3. **Accepted ADRs** in `docs/adr/` (a later ADR supersedes an earlier one; ADRs may override this
   file's *defaults*, never its *principles* — principles change only by editing this file).
4. **Documented upstream behaviour** of PyArabic (<https://github.com/linuxscout/pyarabic>) for
   *public API semantics*.
5. **The Unicode Standard** (UCD, UAX #15, UAX #29) where upstream leaves behaviour undefined.

If you cannot satisfy 1–3 without violating 4 or 5, stop and raise it (§5.5). Never guess silently.

### 0.2 The agent contract (read → plan → test → implement → verify → report)

For **every** task, in this order:

1. **Read**: this file; the modules you will touch; `docs/PARITY.md`; the relevant ADR; existing tests.
2. **Plan**: publish the plan *before* writing implementation code (§5.1). No plan, no production code.
3. **Test first**: write the failing tests and show `cargo test` failing for the *intended* reason.
4. **Implement**: minimal code that makes the tests pass; refactor only while green.
5. **Verify**: run the **full** quality gate (§4.8) — never a subset.
6. **Report**: use the completion-report format (§5.6), including raw gate output and divergences.

### 0.3 Plan mode vs act mode

* In **plan mode** you may read, search and run non-mutating commands; never edit files, never run
  state-changing commands (no `cargo add`, no `git commit`, no file writes).
* In **act mode** you may change files, but `P8` (tests first) and §4.8 (gate) still apply, always.
* A mode switch applies from the newest message onward; permissions never carry over silently.

### 0.4 Cite rules by ID

Reports and reviews cite rule IDs — e.g. `violates P3`, `per §4.3 edge case E7`,
`per Appendix B item normalize::strip_tatweel`. Citations make reviews mechanical instead of vibes.

### 0.5 The ruleset file and its bindings

* The canonical ruleset is **`AGENTS.md`** — an **agent-neutral** name, because every agent *and* every
  human must read the same file and vendor names age badly. Never rename it after a tool, and never fork it
  per audience: one canonical document with stable §-numbers is what makes rule citations (`§4.3`, `P5`)
  work at all.
* **Vendor bindings** are separate and thin: `.cursorrules` (Cursor) and
  `.github/copilot-instructions.md` (Copilot) carry a condensed copy of the non-negotiables. They must be
  updated in the same change as this file (§8.3) and they defer to this file on any conflict.
* `CONTRIBUTING.md` is the human on-ramp (setup → branch → commit → PR → release); it must stay consistent
  with §5.2, §7 and §8.
* A vendor file may be **added** as a pointer to this file (one line: "the rules live in `AGENTS.md`") when
  somebody actually needs it; a second *copy* of the rules must never be created. The repository currently
  ships no `CLAUDE.md` for exactly that reason — it would be an unused duplicate whose only effect is drift.

## 1. Project identity & goal

### 1.1 Mission statement

`r_abic` is a **high-performance, memory-efficient, idiomatic Rust library for Arabic text
processing**: character classification, text normalization, diacritic (tashkeel/ḥarakāt) handling,
tokenization, presentation-form unshaping, transliteration and Arabic number-to-words.

It is an **independent, from-scratch Rust implementation** whose *documented public behaviour* aims
to be a well-specified superset of [PyArabic](https://github.com/linuxscout/pyarabic)'s API. It is
**not** a line-by-line transliteration of Python code (see §2.1).

### 1.2 Design principles — non-negotiable

Cited by ID; violating one requires an ADR (and, for `P1`/`P2`/`P5`, an explicit ruling from the user).

| ID | Principle | What it concretely means |
|----|-----------|--------------------------|
| **P1** | **Zero unnecessary external dependencies** | The published crate has **no runtime dependencies** (`[dependencies]` stays empty). No `regex`, `unicode-normalization`, `once_cell`, `smallvec`, `hashbrown`, `thiserror`. Dev-dependencies are minimal and justified (§2.2). |
| **P2** | **Idiomatic, safe Rust — stricter than the Python equivalent** | `#![forbid(unsafe_code)]`; no `unwrap()`/`expect()`/panicking indexing in library code; `Result` for fallible public APIs; no `todo!`/`unimplemented!` reaching users; no global mutable state. |
| **P3** | **Borrow first, allocate last** | Public APIs take `&str` and return `Cow<'_, str>` where "input unchanged" is the common case; iterator-based (allocation-free) scanning APIs; `*_into(&mut String)` variants for hot loops. Allocate only when the output cannot alias the input. |
| **P4** | **Explicit error handling** | One crate-level `Error` enum (`src/error.rs`) + `pub type Result<T>`; no silent data loss; underspecified input is either handled by an explicitly documented policy (e.g. "unknown characters pass through unchanged") or rejected with `Err`. |
| **P5** | **Unicode correctness by construction** | Iterate with `char`/`char_indices()`; **never** slice at a raw byte index; every emitted range must satisfy `str::is_char_boundary`; never assume "one Arabic letter = 1 byte/1 `char`" (a letter + shadda + ḥaraka is 3 `char`s / 6 bytes). |
| **P6** | **Deterministic, stateless, thread-safe** | Pure functions; no `static mut`; no locale/time/env dependence; public types are `Send + Sync`; same input ⇒ same output on every platform. |
| **P7** | **Parity where documented, standards where ambiguous** | Match PyArabic's *documented* semantics for the same-named function; where upstream is undefined or inconsistent, follow the Unicode Standard and record the delta in `docs/DIVERGENCES.md`. |
| **P8** | **Test-driven, always** | Tests are written before implementation, committed with it, cover the §4.3 edge-case matrix and re-express every Rustdoc example. |
| **P9** | **Documentation is part of the API** | Every public item has Rustdoc with a `# Examples` doctest; every module has a `//!` module doc with a minimal example. |
| **P10** | **Measured performance only** | Hot paths have a Criterion benchmark; performance claims come from `cargo bench` output, never intuition; a >10% regression needs justification in the commit message. |
| **P11** | **Small, reviewable, single-purpose changes** | One logical feature per commit (§5.2); never mix reformatting, renames and behaviour changes. |
| **P12** | **Explicit scope** | Data-heavy upstream modules (`propernouns.py` alone is ~630 KB of name lists) are **out of scope** unless an ADR says otherwise. We ship algorithms, not word lists. |

### 1.3 Non-goals (do not build these)

* Morphological analysis, POS tagging or grammar engines (Qalsadi/Buckwalter territory).
* Full Unicode normalization (NFC/NFD/NFKC/NFKD): we do **not** ship a normalizer. Document loudly
  that input is assumed to be in whatever normalization form the caller supplied, and expose only
  Arabic-specific canonicalizations (`normalize_hamza`, `normalize_ligature`, `strip_tashkeel`, …).
* Spell checking, dialect handling, translation, TTS, stemming beyond documented helpers.
* Any I/O: no file loading, no network, no async runtime, no progress reporting.
* Panicking `Deref`/`Index` tricks that hide errors, and "clever" `unsafe` SIMD. If SIMD is ever
  wanted it arrives as a separate ADR-backed, `unsafe`-contained module behind a feature flag.

### 1.4 Quality & performance budgets (acceptance thresholds)

| Dimension | Budget | Verified by |
|---|---|---|
| Runtime dependencies | **0** | `cargo tree -e normal --depth 1` (must show only `r_abic`) |
| `unsafe` blocks | **0** | `#![forbid(unsafe_code)]` in `src/lib.rs` |
| Clippy | 0 warnings, `-D warnings`, all targets/features | Gate G2 (§4.8) |
| Formatting | `cargo fmt --check` clean | Gate G1 |
| Tests | 100% pass incl. doctests | Gate G3 |
| Docs | `cargo doc` clean with `-D warnings`, no broken intra-doc links | Gate G5 |
| `unwrap`/`expect`/`panic!` in `src/` | 0 | `clippy::unwrap_used`, `expect_used`, `panic` denied (§3.8) |
| Core transforms | single pass, O(n) time, O(1) extra space when nothing changes | `# Performance` Rustdoc section + benchmark |
| Allocation policy | `Cow::Borrowed` for untouched input | test asserting `matches!(out, Cow::Borrowed(_))` |
| Byte-boundary safety | every public byte range is a `char` boundary | property test (§4.5) |
| Coverage (advisory) | ≥ 90% lines on `unicode`, `normalize`, `diacritics`, `tokenize` | `cargo llvm-cov` (local, optional) |

## 2. Licensing, provenance & dependencies

### 2.1 License & provenance — read before copying *anything*

* Crate license: **`MIT OR Apache-2.0`** (as declared in `Cargo.toml`). Every file must stay
  compatible with both; never paste a snippet whose license you have not verified.
* PyArabic is **GPL**-licensed. Therefore, legally and by policy:
  * **Never** copy PyArabic source code, docstrings, tables of literals or test bodies verbatim.
  * API *names* and *documented semantics* are what we target — algorithms are re-derived and
    re-expressed in idiomatic Rust, with our own comments, tests, examples and error types.
  * Any local "oracle" investigation (running upstream Python to observe behaviour) is **dev-only**:
    it lives outside the crate (e.g. a git-ignored `tools/oracle/`), is never referenced from `src/`,
    and any test vector you commit must be independently justifiable from the Unicode Standard or from
    documented behaviour. Record the decision in `docs/adr/0002-licensing-and-provenance.md` first.
  * Never vendor GPL sources into the repo (`vendor/`, `tests/fixtures/`, etc.).
* Attribution is required: `README.md` credits PyArabic as the inspiration and states the parity
  target; `docs/DIVERGENCES.md` lists intentional differences. If a task seems to require copying
  upstream code, **stop and ask the user** — do not improvise a license decision.
* Every new source file starts with `// SPDX-License-Identifier: MIT OR Apache-2.0` (after any
  `//!` doc block, before code).

### 2.2 Dependency policy

* `[dependencies]` stays **empty** (`P1`). To add one you must: (1) justify it in the plan, (2) name
  the rejected hand-rolled alternative, (3) get explicit user approval, (4) add
  `docs/adr/000N-dependency-<crate>.md`. The default answer is "hand-roll it".
* Allowed `[dev-dependencies]` — dev-only, never shipped with the library:

  | Crate | Purpose | Notes |
  |---|---|---|
  | `criterion` | benchmark harness in `benches/` (HTML reports, statistical comparison) | add via `cargo add --dev criterion`; pin the resolved version; must compile on the MSRV, otherwise fall back to the `Instant`-based harness (§4.7) and record why |
  | `proptest` *(optional)* | property tests | only when hand-rolled deterministic property tests are insufficient; keep generators seeded/reproducible |

* Do **not** add `thiserror`: the error surface is intentionally tiny (§3.4). Reconsider only via ADR
  once the enum exceeds ~6 variants with many `From` conversions.
* Banned without an ADR: `regex`, `unicode-normalization`, `once_cell`/`lazy_static`, `anyhow`,
  `serde`, `rayon`, `chrono`, `itertools`.
* Commit `Cargo.lock` (reproducible dev tooling and CI).

### 2.3 Toolchain & MSRV

* Edition **2024**.
* `rust-version` must be declared in `Cargo.toml`; recommended value **`1.85`** (the minimum edition-2024
  toolchain). Raise it only deliberately, through a commit whose body states the MSRV policy change (§8.2).
* Local toolchain: `rustc`/`cargo` 1.98.0 stable. CI additionally runs `stable`, `beta` and the
  declared MSRV (§7).
* Required components: `rustfmt`, `clippy`. Optional local tools: `cargo-llvm-cov`, `cargo-deny`.
* Never commit `target/`, editor state or machine-specific paths.

## 3. Architecture & code conventions

### 3.1 Target repository layout

```text
r_abic/
├── AGENTS.md                      # canonical, agent-neutral ruleset (§0.5)
├── .cursorrules                   # short mirror of the non-negotiables
├── .github/
│   ├── copilot-instructions.md    # short mirror of the non-negotiables
│   └── workflows/
│       ├── ci.yml                 # gate G1–G7 on every PR (§7)
│       ├── conventional-commits.yml # commit + PR-title enforcement (§7.1)
│       └── release.yml            # release-plz: tag + GitHub Release (§7.2, §8)
├── release-plz.toml               # versioning, tag and changelog automation (§8.3)
├── Cargo.toml / Cargo.lock
├── README.md                      # user-facing intro, quick start, parity statement
├── CONTRIBUTING.md                # human on-ramp: setup, branches, commits, PRs (§5.2)
├── CHANGELOG.md                   # Keep a Changelog; sections generated by release-plz (§8)
├── LICENSE-MIT / LICENSE-APACHE
├── docs/
│   ├── PARITY.md                  # living upstream ↔ r_abic symbol tracker
│   ├── DIVERGENCES.md             # documented intentional differences
│   └── adr/0001-record-architecture-decisions.md, 0002-…
├── src/
│   ├── lib.rs                     # crate docs, lints, `mod`s, re-exports only (no logic)
│   ├── error.rs                   # Error, Result, policy docs
│   ├── unicode/
│   │   ├── mod.rs                 # module docs + submodule wiring (no logic)
│   │   ├── constants.rs           # individual code points as `pub const char`
│   │   ├── tables.rs              # class groups, sorted const slices, unshape map
│   │   ├── names.rs               # letter-name/glyph lookup tables (phase 3)
│   │   └── classify.rs            # public `const fn` predicates + `&str` helpers
│   ├── normalize.rs               # strip_*, reduce_*, normalize_*, normalize_searchtext
│   ├── diacritics.rs              # separate/joint, tashkeel codec, similarity
│   ├── tokenize.rs                # tokenize, sentence_tokenize, Token/Tokenizer, fix_spaces
│   ├── unshape.rs                 # presentation forms → base letters; reversal helpers
│   ├── numbers.rs                 # number→words, ordinal, digit conversion, phrase detection
│   ├── trans.rs                   # Buckwalter/TIM/SAMPA transliteration, language segmentation
│   └── kalima.rs                  # Word: (letters, marks) pair
├── tests/                         # integration tests (public API only)
├── benches/                       # criterion benches (or Instant fallback)
└── examples/                      # small runnable end-to-end examples
```

Layout rules: one concern per module, no logic in `lib.rs`, private helpers stay in their consumer
module (never a `pub(crate)` dumping ground), and no `mod.rs` sprawl except the `unicode/` folder.

### 3.2 Module map: PyArabic → `r_abic`

| PyArabic source | Reference symbols (verified against upstream `master`, 2026-10-31) | `r_abic` module | Notes |
|---|---|---|---|
| `araby.py`, `araby_const.py` (constants) | `COMMA`…`BYTE_ORDER_MARK`, `HARAKAT`, `TASHKEEL`, `SHORTHARAKAT`, `TANWIN`, `NOT_DEF_HARAKA`, `LIGUATURES`, `HAMZAT`, `ALEFAT`, `LETTERS`, `NUMBERS_EAST/WEST/PERS`, `MOON`, `SUN`, `WEAK`, `YEHLIKE`, `WAWLIKE`, `TEHLIKE`, `SMALL`, `ALPHABETIC_ORDER`, `NAMES` | `unicode::constants`, `unicode::tables` | `pub const char` per code point + `pub const &[char]` groups; matching uses sorted slices/binary search, never per-call allocation |
| `araby.py` (letter predicates) | `is_hamza`, `is_hamza_above`, `is_hamza_below`, `is_vowel`, `is_shadda`, `is_sukun`, `is_tatweel`, `is_ligature`, `is_letter`, `is_arabicrange`, `is_arabicword`, `has_shadda`, `is_vocalized` | `unicode::classify` | keep the `is_*`/`has_*` snake_case names; the legacy camelCase aliases (`hasShadda`, `isVocalized`, `isArabicword`) are **not** ported → `DIVERGENCES.md` |
| `araby.py` (normalization) | `strip_tashkeel`, `strip_harakat`, `strip_lastharaka`, `strip_tatweel`, `strip_shadda`, `strip_diacritics`, `normalize_hamza`, `normalize_ligature`, `normalize_spellerrors`, `reduce_tashkeel`, `reduce_tatweel` | `normalize` | note upstream spelling **`strip_lastharaka`** (not `…_haraka`): keep it for parity and add `#[doc(alias = "strip_last_haraka")]` |
| `araby.py` (vocalization) | `separate`, `joint`, `vocalizedlike`, `vocalized_similarity` | `diacritics` | `separate`/`joint` are the (letters, marks) core used by the tashkeel codec |
| `araby.py` (tokenization) | `tokenize`, `sentence_tokenize`, `tokenize_with_location`, `fix_spaces`, `autocorrect`, `spellit` | `tokenize` | we add an allocation-free `Tokenizer` iterator and `Token { text: &str, start, end, kind }`; `spellit` needs name tables (phase 3) |
| `araby.py` (misc) | `name(char)`, `glyph(char)` | `unicode::names` | static tables; phase 3 |
| `kalima.py` | `Kalima` (`letters`, `marks`, `word`, `has_shadda`, `is_vocalized`, `is_arabic`) | `kalima::Word` | explicit accessors instead of `Index`/`Deref` tricks; `Eq`/`Hash` over `(letters, marks)` |
| `unshape.py` | `UNSHAPING_TABLE`, `unshaping_word`, `unshaping_line`, `unshaping_text` | `unshape` | upstream mixes de-shaping with character/word reversal; we expose `to_base_letters` and reversal separately, and cover all four ligature pairs (FEF5/FEF6, FEF7/FEF8, FEF9/FEFA, FEFB/FEFC) |
| `normalize.py` | `strip_tashkeel`, `strip_tatweel`, `normalize_hamza`, `normalize_lamalef`, `normalize_spellerrors`, `normalize_searchtext` | `normalize` | upstream duplicates these across modules; we keep one canonical location (`normalize_lamalef` ≡ `normalize_ligature` alias) |
| `number.py` | `ArNumbers` (`int2str`, `set_feminine`, `set_format`), `number2text`/`number2letters`, `number2ordinal`, `detect_numbers`, `detect_number_words`, `extract_number_phrases`, `detect_number_phrases_position`, `pre_tashkeel_number` | `numbers` | grammar-sensitive (gender, case); pure functions instead of a stateful class; largest test matrix |
| `trans.py` | `translate`, `convert`, `utf2tim`, `tim2utf8`, `tim2sampa`, `normalize_digits`, `segment_language`, `delimite_language`, `encode_tashkeel`, `decode_tashkeel` | `trans` (+ `diacritics` for the tashkeel codec) | `normalize_digits(text, source, out)` with `west`/`east`/`persian`/`all`; use enums, not strings |
| `soundex.py` | `soundex`, `soundex_map_code`, `soundex_trim_rep` | `soundex` (phase 3) | small, self-contained parity win |
| `named.py`, `named_const.py`, `propernouns.py`, `stack.py`, `arabrepr.py` | — | **out of scope** (`P12`) | record in `DIVERGENCES.md` so users know what is intentionally missing |

Naming rule: prefer the upstream snake_case name verbatim so parity stays greppable. When we must
differ, document it with `/// Mirrors PyArabic `araby.strip_tashkeel`.` **and** an entry in
`docs/DIVERGENCES.md`.

### 3.3 Public API design rules

* **Shape of the API.** Use these canonical signatures (consistent across the whole crate):

  ```rust
  // 1. character predicates: const, total, panic-free
  pub const fn is_shadda(c: char) -> bool;

  // 2. whole-text transforms: borrow when unchanged, allocate only when needed
  pub fn strip_tashkeel(text: &str) -> Cow<'_, str>;
  pub fn strip_tashkeel_into(text: &str, out: &mut String) -> &mut String; // clears `out`, then writes

  // 3. structural transforms that cannot alias the input
  pub fn separate(word: &str) -> (String, String); // (letters, marks)

  // 4. scanning: allocation-free iterator first, Vec convenience second
  pub fn tokens(text: &str) -> Tokenizer<'_>;
  pub fn tokenize(text: &str) -> Vec<Token<'_>>;
  ```

  `_into` variants **clear the destination and then append** the result (documented in Rustdoc); they
  return `&mut String` so calls can be chained. They exist for hot loops and must never allocate more
  than the output requires (reserve with `String::with_capacity`/`reserve` when the bound is known).
* **Never panic on user input** (`P2`). Every public function has a `# Panics` section stating
  "never panics" or naming the exact precondition; all byte-index arithmetic is derived from
  `char_indices()`/`is_char_boundary` checks, never from hand-counted offsets.
* **`#[must_use]`** on every pure function and iterator adapter; **`#[inline]`** only on tiny hot
  predicates and only with benchmark evidence.
* **Traits:** derive `Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default` where
  semantically valid (`Token`, `Word`, enums); implement `Display` where there is a canonical textual
  form. Never implement `Deref` (no "smart pointer" apologetics), never `Index` (use accessors), never
  silent `From` conversions that allocate.
* **Enums instead of strings** for closed option sets: `DigitSet { West, East, Persian }`,
  `TransliterationScheme { Arabic, Tim, Sampa, ... }`, `HamzaMethod { Uniform, Tasheel }`. Add
  `#[non_exhaustive]` to enums we may extend (`Error`, `TokenKind`, option enums) so growth is not a
  breaking change. Provide `FromStr` + `Display` when a CLI/config string form is genuinely useful.
* **Options builders** for >3 orthogonal options: `TokenizeOptions::default().with_…()`; do not grow
  boolean parameter lists (`fn tokenize(text: &str, a: bool, b: bool)` is forbidden).
* **Crate lints** in `src/lib.rs` (top of file):

  ```rust
  #![forbid(unsafe_code)]
  #![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
  #![warn(clippy::pedantic)]
  #![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::todo,
          clippy::unimplemented, clippy::dbg_macro, clippy::print_stdout)]
  #![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))] // tests may assert
  #![allow(clippy::module_name_repetitions)] // `normalize::normalize_searchtext` mirrors upstream
  ```

  Pedantic lints are `warn` (visibility) but CI promotes warnings to errors, so effectively deny —
  except for a small, explicit, commented `allow` list that must be justified in the PR description.
* **Feature flags:** `std` (default) and nothing else for now; core logic must stay `alloc`-compatible
  (`core` + `alloc` paths) as a Tier-2 goal. Adding a feature that pulls a dependency is banned (`P1`).
* **Semver discipline:** no breaking change to any `pub` item without (a) an ADR, (b) a `!` marker plus
  a `BREAKING CHANGE:` footer on the commit — that footer is what surfaces the change in `CHANGELOG.md`
  (§8.2) — and (c) a note in `docs/DIVERGENCES.md` if it moves away from upstream behaviour.

### 3.4 Error model (`src/error.rs`)

Small, hand-written, dependency-free (`P1`/`P4` — no `thiserror`):

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A transliteration/conversion direction is not supported.
    UnsupportedConversion { from: TransliterationScheme, to: TransliterationScheme },
    /// `joint`/`decode_tashkeel` received marks whose length cannot match the letters.
    TashkeelLengthMismatch { letters: usize, marks: usize },
    /// Encoded tashkeel string is malformed or not decimal.
    InvalidTashkeelEncoding,
    /// Number-to-words input outside the supported domain (negative or too large).
    NumberOutOfRange { value: i64 },
    /// The function is undefined for empty input (e.g. `soundex`).
    EmptyInput,
}

impl fmt::Display for Error { /* user-facing sentences, no jargon */ }
impl std::error::Error for Error {}          // `source()` → None; never `unwrap`
pub type Result<T, E = Error> = core::result::Result<T, E>;
```

Rules: variants are added only with a real call site **and** a test; `Display` messages are
actionable sentences naming the offending value; error types are `Clone + Eq` so tests can assert on
them; nothing ever `panic!`s to signal an error. Declared policy for inputs that are *not* errors:

| Input situation | Policy (must be documented in the Rustdoc of the function) |
|---|---|
| Non-Arabic characters (Latin, CJK, emoji) | pass through unchanged in transforms; `TokenKind::Other` when tokenizing |
| Directional marks `U+200E/U+200F/U+061C`, ZWJ/ZWNJ `U+200D/U+200C`, BOM `U+FEFF` | preserved by transforms; classified explicitly (never silently dropped); tokenizer behaviour is pinned by tests |
| Tatweel `U+0640` | preserved except by tatweel-stripping functions; never treated as a letter boundary in `separate()` |
| Lone combining mark (ḥaraka before any base letter) | preserved by transforms; tokenizer attaches trailing marks to the preceding token, and a leading orphan mark becomes its own diacritic token |
| Empty input | `Ok` with unchanged `Cow::Borrowed("")` for transforms; `Err(Error::EmptyInput)` only where the definition excludes empty input |

### 3.5 Unicode handling rules (the core of this crate — read twice)

**Iteration model.** All scanning is done with `char_indices()`/`chars()`; all slicing uses byte
offsets that came from those iterators (or from `find`/`match_indices`), and every offset is validated
in debug builds with `debug_assert!(text.is_char_boundary(i))`. Byte arithmetic by hand is forbidden
(`P5`). Canonical shape of a text transform:

```rust
pub fn strip_tashkeel(text: &str) -> Cow<'_, str> {
    match text.char_indices().find(|&(_, ch)| is_tashkeel(ch)) {
        None => Cow::Borrowed(text),                       // zero-copy fast path
        Some((first, _)) => {                              // `first` is a char boundary
            let mut out = String::with_capacity(text.len());
            out.push_str(&text[..first]);                  // copy the clean prefix as one memcpy
            out.extend(text[first..].chars().filter(|ch| !is_tashkeel(*ch)));
            Cow::Owned(out)
        }
    }
}
```

This "scan for first change → memcpy prefix → filter the rest" pattern is the required template for
every `strip_*`/`normalize_*` function: it gives O(n) time, one allocation only when needed, and
`Cow::Borrowed` for already-clean input.

**Class matching.** Classes are sets of `char`, matched with `matches!` ranges when contiguous
(e.g. `matches!(c, '\u{064B}'..='\u{0652}' | '\u{0654}' | '\u{0655}')`) and with **sorted**
`&'static [(char, char)]` range tables + binary search otherwise. `regex`, `char::is_alphanumeric`
and ad-hoc string literals for membership tests are all forbidden. Facts you must respect:

* Arabic letters live in `U+0600–U+06FF` (Arabic), `U+0750–U+077F` (Arabic Supplement),
  `U+08A0–U+08FF` (Arabic Extended-A), and presentation forms in `U+FB50–U+FDFF` / `U+FE70–U+FEFF`.
  See Appendix A for the code points we actually handle.
* `char::is_alphabetic()` is **not** "is an Arabic letter": it follows Unicode `Alphabetic`, which
  includes Arabic letters *and* many other scripts. Use our explicit tables for parity (`P7`).
* Tatweel `U+0640` is a joiner, not a letter and not a ḥaraka: it must never split a word in
  `separate()`/`tokenize()`, and only tatweel-stripping functions remove it.
* Ḥarakāt (`U+064B`–`U+0652`), shadda `U+0651`, superscript alef `U+0670`, ZWJ/ZWJN and
  directional marks are **combining/formatting code points**: they are separate `char`s with their own
  bytes; never assume one `char` per displayed glyph, and never assume an Arabic word is `len()` bytes.

**Length-changing operations (the classic bug source).** These transformations change the number of
`char`s and/or bytes, so output positions must never be reused as input positions:

| Operation | Effect | Consequence for API design |
|---|---|---|
| `normalize_ligature` (LAM_ALEF `U+FEF5–U+FEFC` → LAM + ALEF) | 1 char → 2 chars | output byte range ≠ input byte range; functions that map positions must say so |
| `separate` (letters/marks split) | same count of chars, but split across two buffers | marks index = letter index; documented, tested |
| digit-set conversion (`٠١٢٣٤٥٦٧٨٩` ↔ `0123456789` ↔ `۰۱۲۳۴۵۶۷۸۹`) | same char count, **different byte length** (2 bytes ↔ 1 byte) | only char positions survive; never byte positions |
| `strip_*`, `reduce_*` | fewer chars | positions shift left; ranges are recomputed, never adjusted arithmetically |

**Marks are never reordered.** We do not implement NFC/NFD/NFKC (non-goal §1.3): combining marks keep
the order the caller supplied (so `SHADDA` + `FATHA` stays `SHADDA`+`FATHA` even when a normalizer
would swap them). Every public function that does more than pass text through states this in a
`# Unicode` Rustdoc section, and this is a headline item in `docs/DIVERGENCES.md`.

**Presentation forms.** `unshape` maps `U+FE70–U+FEFC` (isolated/initial/medial/final forms and the
four lam-alef ligature pairs) to base letters; the ligature pairs expand to two `char`s, so `unshape`
is *not* position-preserving and *not* reversible in isolation. Text reversal is a separate,
explicitly named function (`unshape` must not silently reverse, unlike upstream — see
`docs/DIVERGENCES.md`).

**Tables are data, and data gets validated.** Every `const` table that is binary-searched must be
sorted; a unit test asserts (a) sortedness, (b) no duplicate ranges, (c) every entry is a valid
`char`, and (d) round-trip consistency for maps (`map`/`inverse_map` must be consistent). When a table
grows beyond ~30 entries, add a benchmark for the lookup path.

### 3.6 Allocation & performance rules

* **Return `Cow` when identity is plausible**, `String`/`Vec` only when the output structurally cannot
  alias the input (`separate`, tokenization, number-to-words).
* **Iterator-first scanning**: the `Tokenizer` iterator yields borrowed slices with zero allocation;
  `tokenize()` is the convenience wrapper that collects. Never build an intermediate `Vec<String>`.
* **Two-phase algorithms**: find the first difference, copy the prefix in one `memcpy`, then transform
  the remainder (§3.5). Never allocate a fresh `String` for input that needs no change.
* **Buffer reuse**: offer `*_into(&mut String)` for every `-> Cow` transform likely to be called in a
  loop; document that it clears and rewrites the buffer. Batch APIs take `&mut Vec<T>` or return
  iterators instead of allocating per call where practical.
* **Capacity**: pre-reserve using the input length as an upper bound only when the operation cannot
  grow the string; otherwise reserve `text.len() + expected_growth`. Never use `format!` in library
  code (always allocates, hides the bound).
* **Lookup strategy**: binary search over sorted `const` tables, `matches!` for dense ranges, `match`
  jump tables for maps. `HashMap`/`BTreeMap` are forbidden for fixed tables; tables are
  `const`/`static` so they land in `.rodata` with no runtime initialisation (no `lazy_static`).
* **ASCII/Latin fast paths** only with benchmark evidence, and always paired with an equivalence test
  that compares fast and slow paths on the same input.
* **Bounds and overflow**: no truncating `as usize`; use `usize::try_from`/checked arithmetic where
  input length feeds a computation (e.g. number-to-words), guarded by `Error::NumberOutOfRange`.
* **Measurement rules** (`P10`): benchmarks use `black_box(input)`, pre-build inputs outside the timed
  loop, exercise three sizes (tiny/realistic/1 MiB), and never assert on timing in a test (noisy CI).
  CI compiles benches (`cargo bench --no-run`) but does not run them; run them locally when touching a
  hot path and paste the summary table into the completion report (§5.6).
* **Avoid**: `Vec<String>` joins, `to_string()` on `char` inside loops, `push_str(&format!(...))`,
  closure-allocating sorts, recursion where iteration works, and cloning anything a borrow could do.

### 3.7 Naming & code style

* **Naming:** mirror upstream snake_case names exactly where parity matters (`strip_tashkeel`,
  `normalize_ligature`, `strip_lastharaka`, `sentence_tokenize`). Types are `UpperCamelCase`
  (`Token`, `Tokenizer`, `TokenKind`, `Word`, `DigitSet`); constants mirror upstream SCREAMING_SNAKE
  (`HAMZA`, `ALEF_MAKSURA`, `SHADDA`); internals are descriptive and private.
* **Domain vocabulary** is preferred over invented synonyms: alef/beh/teh/teh_marbuta/theh/jeem/hah/
  khah/dal/thal/reh/zain/seen/sheen/sad/dad/tah/zah/ain/ghain/feh/qaf/kaf/lam/meem/noon/heh/waw/yeh,
  hamza, madda, wasla, tatweel (kashida), shadda, sukun, fatha, damma, kasra, tanwin, harakat,
  tashkeel, kalima.
* Spelling: use the Unicode names for code points (`ALEF`, not `ALIF`) and the upstream spelling for
  functions. Document Arabic terms with script on first use ("shadda (شَدَّة)").
* `rustfmt` defaults are authoritative (100 columns, 4-space indent, `max_width` untouched). A
  `rustfmt.toml` is added only if the plan proposes it, and stays limited to `edition`/`style_edition`.
* Comments explain **why** ("upstream excludes this case; see DIVERGENCES #4"), never restate the code.
  Every `#[allow(...)]` carries a trailing comment with its reason.
* Functions stay small and single-purpose (target < 40 lines); extract private helpers instead of
  nesting `if`/`match` deeper than three levels. Prefer early returns, `let ... else`, and exhaustive
  `match` (no wildcard `_ =>` where a new variant *should* break the build).
* Tests may use `unwrap()`/`expect()`/`assert!` freely: `lib.rs` carries
  `#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]`, so the deny list
  applies to library code only — while every assertion still carries a message naming the expectation.
* No `unsafe` of any kind (`P2`): no `#[allow(unsafe_code)]`, no `transmute`, no raw pointers, no
  `MaybeUninit`, no FFI.

### 3.8 Rustdoc standards

Every public item follows this structure (omit empty sections, never leave placeholders):

````rust
/// Removes all ḥarakāt (short vowels and tanwīn) **and** shadda from `text`.
///
/// Mirrors PyArabic `araby.strip_tashkeel`.
///
/// # Examples
///
/// ```
/// use r_abic::normalize::strip_tashkeel;
///
/// assert_eq!(strip_tashkeel("الْعَرَبِيَّةُ"), "العربية");
/// // Zero-copy when there is nothing to strip:
/// assert!(matches!(strip_tashkeel("العربية"), std::borrow::Cow::Borrowed(_)));
/// ```
///
/// # Unicode
///
/// Operates on `char`s; combining marks are removed individually and never reordered.
///
/// # Performance
///
/// Single pass, O(n). Allocates only when a removable mark is found; otherwise borrows `text`.
///
/// # Parity
///
/// Same result as upstream for all inputs; related upstream spelling aliased below.
#[must_use]
#[doc(alias = "strip_last_haraka")]
pub fn strip_tashkeel(text: &str) -> Cow<'_, str> { /* … */ }
````

Rules: `# Examples` is mandatory and every example is an executable doctest (never `no_run` unless the
example truly cannot run, with an inline justification); examples `assert_eq!` their expectations so
they cannot rot; `# Panics`, `# Errors`, `# Performance`, `# Unicode`, `# Parity` appear whenever they
carry information. Module docs (`//!`) open with one sentence on the module's purpose plus a minimal
example. Crate docs in `lib.rs` hold the curated quick start, the PyArabic parity statement, and the
license. Before finishing, run `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` and keep
`docs/*.md` in sync with behaviour changes.

## 4. Testing & automation standards (TDD)

### 4.1 The mandatory TDD loop (per task)

1. **Clarify the contract first**: in the plan, state the exact signature(s), documented behaviour,
   error cases, the upstream symbol being mirrored, and the divergence policy if any.
2. **Red — tests before implementation.** Add the unit tests (including the relevant §4.3 edge cases)
   and the doctest **first**, then run `cargo test <filter>` and paste the failing output in the
   report. Tests must exist and fail before implementation code is written in the same change; a test
   that fails merely to compile does not count as evidence for logic.
3. **Green — minimal implementation.** Only what the tests require: no speculative options, no unused
   helpers, no dead `pub` items.
4. **Refactor while green** (names, duplication, docs). Behaviour and assertions must not change.
5. **Harden**: add the property test, the `Cow::Borrowed` assertion, and char-boundary assertions
   where they apply (§4.5).
6. **Gate**: run the full quality gate (§4.8) and copy the output into the report.

Never write implementation first "because it's obvious". Never weaken, skip (`#[ignore]`) or delete a
test to make the gate pass: fix the code, or change the contract explicitly and document why.

### 4.2 Test layout & taxonomy

| Level | Location | Purpose and rules |
|---|---|---|
| Unit | `src/<module>.rs` → `#[cfg(test)] mod tests` | private helpers, table validation, single-function behaviour. Prefer table-driven tests: `const CASES: &[(&str, &str)] = &[...]` + a loop whose assertion message includes the input (`assert_eq!(actual, expected, "input: {input:?}")`). |
| Integration | `tests/<feature>.rs` | public API only (`use r_abic::…`), end-to-end flows: normalize → separate → transliterate; mixed Arabic/Latin documents; multi-line corpora. Mirror the upstream test *topics* (letters, unicode, tokenize, trans, numbers) with our own cases. |
| Doc tests | Rustdoc `# Examples` | every public item; run via `cargo test --doc`; must pass with `--all-features` too. |
| Property | `tests/properties.rs` | invariants that must hold for all inputs (§4.5): char-boundary validity, idempotence, monotonic length relations, codec round-trips. |
| Benchmarks | `benches/*.rs` | Criterion (default) or the `Instant` fallback (§4.7). Compiled in CI; run locally when a hot path changes. |
| Examples | `examples/*.rs` | tiny, runnable, compile in CI (`cargo build --examples` + a smoke `cargo run --example …`). |

Rules: test names are behaviour sentences (`strip_tatweel_removes_only_u0640_and_keeps_letters`); one
behaviour per test; arrange/act/assert separated by blank lines; **no** sleeps, network, filesystem or
time dependence; fixtures live in `tests/fixtures/` and are read with `include_str!` so tests are
hermetic; assertions are order-independent (never depend on `HashMap` iteration — sort or use
`BTreeMap`); every test that pins an upstream parity decision carries a comment naming the upstream
symbol and (if relevant) the `DIVERGENCES.md` entry.

### 4.3 Mandatory edge-case matrix

Every module's tests must cover the rows that apply to it, and reference the ID in the test name or a
comment (`// E7: mixed script`). Missing coverage for an applicable row is a review blocker.

| ID | Case | Example input | Required behaviour |
|---|---|---|---|
| **E1** | Empty string | `""` | transforms → `Cow::Borrowed("")`; `tokenize` → empty; `separate` → `("","")`; only `EmptyInput`-guarded functions (`soundex`) return `Err` |
| **E2** | Whitespace only (incl. NBSP `U+00A0`, `\t`, `\n`, `\r\n`) | `"   \t\n"` | whitespace tokens/segments; transforms borrow the input unchanged |
| **E3** | Ḥarakāt only, no base letter | `"\u{064E}\u{0651}"` | `strip_tashkeel` → `""`; never panics; never produces invalid UTF-8 |
| **E4** | Text starting with a combining mark | `"\u{064E}ب"` | transforms preserve it; tokenizer follows the orphan-mark rule (§3.4) |
| **E5** | Shadda + ḥaraka in both orders | `"الشَّمس"`, `"الشَّمس"` | input mark order is preserved (no reordering); `strip_tashkeel` removes all marks in both |
| **E6** | Tatweel / kashida | `"العــــربية"`, `"ـــ"` | `strip_tatweel` removes only `U+0640`; a tatweel-only run is its own token and is **never** a `Word`; tatweel never splits a word in `separate()` |
| **E7** | Mixed Arabic + Latin | `"العربية Book"` | transforms leave Latin untouched; tokenizer classifies script per token; round-trip keeps bytes of the Latin part identical |
| **E8** | Digits in all three sets | `"12 ١٢ ۱۲"` | `normalize_digits(source, out)` for `west`/`east`/`persian`/`all`; char positions survive, byte positions do not |
| **E9** | Arabic and ASCII punctuation | `"مرحبا، كيف؟"`, `"…"`, `"؛"` | punctuation classified as such and never folded into words; `U+060C/U+061B/U+061F/U+06D4` all covered |
| **E10** | Astral (4-byte) characters / emoji | `"مرحبا 🎉"` | no split inside a code point; every returned range satisfies `is_char_boundary` |
| **E11** | ZWJ / ZWNJ | `"ب\u{200D}ت"`, `"ب\u{200C}ت"` | preserved by transforms; explicit, tested tokenizer classification (documented, not accidental) |
| **E12** | Directional/formatting marks | `U+200E`, `U+200F`, `U+061C` | preserved; classified explicitly; never silently dropped |
| **E13** | BOM | `"\u{FEFF}مرحبا"` | preserved by transforms; tokenizer gives it its own token (documented) |
| **E14** | Presentation forms (single char) | `"\u{FE8D}\u{FE8E}"` | de-shaped to `"اا"`; position-preserving 1:1 mapping |
| **E15** | Lam-alef ligatures | `"\u{FEFB}"`, `"\u{FEF5}\u{FEF7}"` | expand to 2 chars (`لا`, `لآ`, `لأ`, `لإ`); a 1→2 expansion must not corrupt the surrounding text; consecutive ligatures both expand |
| **E16** | Mark directly after a ligature or presentation form | `"\u{FEFB}\u{0651}"` | shadda survives de-shaping and stays attached to the expanded pair's last letter |
| **E17** | CRLF vs LF, multi-line | `"أ\r\nب"`, `"أ\nب"` | identical results modulo the line terminator itself; newline count preserved exactly (upstream's `unshaping_text` adds a spurious leading `\n` — we do not) |
| **E18** | Large input | 1 MiB of repeated prose | O(n) time, no recursion, no stack overflow, no quadratic re-allocation (benchmark + test) |
| **E19** | Redundant/stacked marks | `"\u{064E}\u{064E}"`, `"\u{0651}\u{0651}"` | `strip_tashkeel` removes all; `reduce_tashkeel`/`autocorrect` behaviour is documented and pinned by tests |
| **E20** | Unicode form mixing | NFC vs NFD input for the same word | behaviour is identical (we do not normalize) and documented in `# Unicode`; test asserts we neither add nor remove marks |
| **N1** | Number boundaries | `0, 1, 2, 3, 10, 11, 19, 20, 21, 99, 100, 101, 110, 199, 200, 1000, 2000, 11_000, 1_000_000` | correct agreement/gender per documented grammar; `NumberOutOfRange` outside the supported domain |
| **N2** | Ordinals & gender | `1..=30`, masculine and feminine flags | `number2ordinal` (and its feminine variant) tested for both genders |
| **N3** | Tashkeel codec round-trip | `"هَارِبًا"` ↔ `("هاربا", "a0iA0")` | `encode_tashkeel`/`decode_tashkeel` round-trip; malformed input → `Error::InvalidTashkeelEncoding`; length mismatch → `TashkeelLengthMismatch` |

### 4.4 Fixture & golden-data policy (provenance-safe)

* Fixtures are **ours**: derived from the Unicode Standard and from documented behaviour, with a header
  comment recording their origin and the justification for every expectation.
* **Never** paste upstream (GPL) test bodies, docstring examples, or corpus files (§2.1). Importing a
  corpus from another project requires a license note in the file header and a maintainer decision.
* Layout: `tests/fixtures/<feature>/`, loaded with `include_str!` (hermetic, no filesystem at test
  time). Prefer one case per line in a small TSV with `#` comments:
  `input<TAB>expected<TAB>edge-case-id<TAB>note`; multi-line cases use `\n` escapes explicitly.
  A shared loader lives in `tests/common/mod.rs` (integration tests only — never add a test-only
  module to `src/`).
* Every case cites the matrix ID (`E1`…`E20`, `N1`…) that motivates it. If an expected value came from
  running an external tool, record the tool and version in a comment; if that tool is upstream
  PyArabic, the case must be re-justified from the standard or dropped.
* Keep committed fixtures small (< 100 KB total). Bulky corpora belong in a separate benchmark repo,
  not in the library.

### 4.5 Property & invariant tests

Without adding dependencies, build deterministic pseudo-random inputs from a **fixed seed** (a tiny
documented xorshift in `tests/common/mod.rs`) drawn from a curated alphabet: Arabic letters, every
ḥaraka, shadda, tatweel, presentation forms, the four ligature pairs, digits in all three sets, ASCII
letters/punctuation, spaces, `\n` and `\r\n`, ZWJ/ZWNJ, directional marks, BOM, one emoji and one CJK
character (4-byte and 3-byte UTF-8).

Mandatory properties (where applicable to the module under test):

1. **Boundary safety** — for every returned range, `text.get(start..end).is_some()` and
   `text.is_char_boundary(start)` / `(…end)`.
2. **Totality** — a few thousand generated inputs drive every public function without a panic.
3. **Idempotence** — `f(f(x)) == f(x)` for `strip_*`, `normalize_*`, `to_base_letters`,
   `reduce_tatweel`.
4. **No character creation/destruction** — `strip_*` never adds characters; `strip_harakat` keeps
   shadda while `strip_tashkeel` removes it; `normalize_ligature(x).chars().count() >= x.chars().count()`.
5. **Round-trips** — `decode_tashkeel(x, encode_tashkeel(x).1) == x`; `joint(separate(w)) == w`; digit
   conversions are pairwise inverses; `convert(text, A, B)` then `convert(_, B, A)` is identity **only**
   for documented lossless pairs (hamza folding is lossy and must be excluded explicitly).
6. **Tokenization covers the input** — concatenating all token slices in order reproduces the input
   byte-for-byte (nothing dropped, nothing reordered, whitespace and ZWJ included).
7. **Borrowed fast path** — `matches!(f(clean), Cow::Borrowed(_))` for inputs already in the target
   form.

Failures must print the counterexample:
`assert_eq!(actual, expected, "counterexample: {input:?}")`.

### 4.6 Documentation-test rules

* Every public item has ≥ 1 doctest; every module doc has one too. `cargo test --doc` (and
  `--all-features`) is part of the gate.
* Coverage per item: happy path; one edge case from §4.3 where relevant; the zero-copy behaviour for
  `Cow`-returning functions; **the error path** for `Result`-returning functions using an explicit
  harness:

````rust
/// ```
/// # fn main() -> Result<(), r_abic::Error> {
/// use r_abic::trans::{convert, TransliterationScheme};
/// assert!(matches!(
///     convert("abc", TransliterationScheme::Arabic, TransliterationScheme::Arabic),
///     Err(r_abic::Error::UnsupportedConversion { .. })
/// ));
/// # Ok(())
/// # }
/// ```
````
* `no_run`/`ignore`/`compile_fail` are discouraged; each use needs an inline justification, and
  `ignore` requires an ADR-level reason (it hides broken examples).
* Keep examples minimal and assertion-based (`assert_eq!`), with `# ` hidden lines only for plumbing.
* Doctest text is **ours**: never paste upstream docstring examples (§2.1) — write the same idea in our
  own words and with our own inputs.

### 4.7 Benchmarks

* Default harness: **Criterion** (dev-dependency) with `harness = false`:

  ```toml
  [[bench]]
  name = "normalize"
  harness = false
  ```

  Add it with `cargo add --dev criterion` so the resolved version is pinned in `Cargo.toml`, and keep
  the version MSRV-compatible. Report-style output is not committed (`target/criterion/` is ignored).
* **Fallback harness** when Criterion is unusable (offline, MSRV conflict): a `harness = false`
  `main()` that warms up, runs N iterations with `std::hint::black_box`, and prints a duration table.
  State the choice and why in the file header — never mix harnesses inside one bench file.
* Required bench coverage (add more as hot paths appear): `normalize::strip_*`, `normalize::normalize_*`,
  `diacritics::separate`, `diacritics::joint`, `tokenize::tokenize` and `tokenize::tokens`,
  `unshape::to_base_letters`, `trans::convert`, `trans::normalize_digits`, `numbers::number2text`.
* Rules: inputs are built once outside the timed closure; three sizes (tiny / realistic paragraph /
  1 MiB); no `println!` inside timed code; no timing assertions anywhere in tests; benches must
  compile on every CI target (`cargo bench --no-run`).
* Workflow when touching a hot path: `cargo bench -- --save-baseline before`, make the change,
  `cargo bench -- --baseline before`, then paste the comparison table into the completion report
  (§5.6). A >10% regression must be justified in the commit message or reverted (`P10`).

### 4.8 The Quality Gate — run **all** of it, in order, before declaring any task done

| Gate | Command | Pass criterion |
|---|---|---|
| **G1** | `cargo fmt --all -- --check` | no diff |
| **G2** | `cargo clippy --all-targets --all-features -- -D warnings` | zero warnings (`--all-targets` covers tests/benches/examples) |
| **G3** | `cargo test --all-features` | `test result: ok` for unit, integration **and** doc tests |
| **G4** | `cargo test --doc --all-features` | all doctests pass (explicit, never assumed) |
| **G5** | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features` | builds clean, no broken intra-doc links |
| **G6** | `cargo bench --no-run` | every bench compiles |
| **G7** | `cargo tree -e normal --depth 1` | output contains only `r_abic` (zero runtime deps, `P1`) |
| **G8** *(optional)* | `cargo run --example quickstart` · `cargo deny check` · `cargo miri test` | smoke/audit only; run when the tool is available |
| **G9** | `git log -1 --format=%s` matches the Conventional Commits pattern (§5.2) · `cargo metadata --no-deps --format-version 1` parses | the commit you are about to make (or just made) produces the intended release (§8.2); the manifest is valid |

Windows PowerShell (this environment):

```powershell
cd 'd:\projects\r_abic'
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
$env:RUSTDOCFLAGS = '-D warnings'; cargo doc --no-deps --all-features; Remove-Item Env:\RUSTDOCFLAGS
cargo bench --no-run
cargo tree -e normal --depth 1
git log -1 --format=%s    # G9: must be a Conventional Commit (§5.2, §8.2)
cargo metadata --no-deps --format-version 1 > $null   # G9: manifest parses
```

POSIX shells (`bash`/`zsh`, for CI parity):

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features && cargo test --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo bench --no-run
cargo tree -e normal --depth 1
git log -1 --format=%s && cargo metadata --no-deps --format-version 1 > /dev/null   # G9
```

Rules: fix formatting first, then lints, then tests, then docs. **Nothing is committed and nothing is
reported as complete while any gate fails.** If a gate cannot run in this environment (missing tool, no
network for a new dev-dependency), that is a blocker for claiming success — report it as
"NOT VERIFIED" with the exact error, never as "should pass". Evidence: paste the summary/tail lines of
each gate into the completion report (§5.6), including the `test result:` line.

## 5. Workflow & agent execution rules

### 5.1 Plan-first protocol (mandatory before implementation)

Publish a plan with exactly these headings (short, complete, no filler):

1. **Goal** — the user-visible outcome, in behavioural terms, one paragraph.
2. **Parity target** — the upstream symbol(s) mirrored, plus the divergence policy if behaviour differs.
3. **Public API** — exact signatures, types, error variants, `#[non_exhaustive]` decisions, semver impact.
4. **Files** — every file to add or touch, with a one-line purpose each (must match §3.1 layout).
5. **Tests first** — the list of tests to write before implementation, mapped to §4.3 IDs and §4.5 properties.
6. **Unicode/edge analysis** — which `E*`/`N*` rows apply, with the exact expected output for the tricky
   ones (ligature expansion, tatweel-only runs, orphan marks, CRLF, digit sets).
7. **Allocation/perf plan** — `Cow` vs `String`, `_into` variants, fast paths, the benchmark to run.
8. **Docs** — Rustdoc sections to fill, plus updates to `README.md`, `docs/PARITY.md` and
   `docs/DIVERGENCES.md` (never to version numbers or released changelog sections, §8.7).
9. **Risks / open questions** — anything requiring a user decision, asked **before** implementing.
10. **Definition of Done** — the concrete gate commands (§5.3).

If the plan needs more than roughly one screen, the task is too big: split it (§5.2) and plan the first
slice only. In act mode, proceed from plan → tests without waiting for approval, unless the change is
API-breaking, scope-changing, license-affecting or dependency-adding — those always require user
confirmation first.

### 5.2 Task granularity, commits & history hygiene

* One logical feature per task, e.g. "`normalize::strip_tashkeel` + tests + docs", never
  "implement the whole `normalize` module".
* Recommended landing order per area (each its own commit-sized task):
  1. `unicode`: constants, predicates, table-validation tests
  2. `normalize`: `strip_tashkeel`, `strip_harakat`, `strip_lastharaka`, `strip_tatweel`, `strip_shadda`, `strip_diacritics`
  3. `normalize`: `normalize_hamza`, `normalize_ligature`/`normalize_lamalef`, `normalize_spellerrors`, `normalize_searchtext`
  4. `normalize`: `reduce_tashkeel`, `reduce_tatweel`, `autocorrect`, `fix_spaces`
  5. `diacritics`: `separate`, `joint`, then the tashkeel codec (`encode_tashkeel`/`decode_tashkeel`)
  6. `tokenize`: word tokenizer (iterator + `Vec`), then `sentence_tokenize`/`tokenize_with_location`
  7. `unshape`: de-shaping, then reversal helpers
  8. `trans`: transliteration tables & `convert`, then `normalize_digits`, then `segment_language`
  9. `numbers`: cardinal, then ordinal, then detection/vocalization helpers
  10. `kalima::Word` and remaining niceties (`soundex`, `unicode::names`)
* **Conventional Commits — mandatory and CI-enforced** (`.github/workflows/conventional-commits.yml`):
  every PR title and every commit subject must match
  `^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(scope\))?!?: <subject ≤ 72 chars>`.
  Lowercase type, imperative mood, scope = module or area (`normalize`, `tokenize`, `unicode`, `ci`,
  `docs`), body explains *why*, footers for `BREAKING CHANGE:` / `Refs:`.
  * **Types that move the version**: `feat` → MINOR, `fix` → PATCH, `!` / `BREAKING CHANGE` → MAJOR
    (MINOR while the crate is `0.x`). `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`,
    `chore` never release on their own — full table in §8.2.
  * **The PR title becomes the squash-merge commit subject**, so write it as the commit you want in
    history — a mislabelled title silently changes the next version.
  * Good examples: `feat(normalize): add strip_tashkeel with zero-copy fast path`,
    `test(normalize): cover E1–E6 and idempotence`,
    `docs(normalize): document parity with pyarabic.araby.strip_tashkeel`,
    `fix(unshape)!: stop reversing text in to_base_letters`
    (`BREAKING CHANGE: to_base_letters no longer reverses the input`),
    `chore(ci): promote clippy warnings to errors`.
  * Bad examples (CI rejects them): `Added strip_tashkeel`, `feat add x`, `feature: x`, `fix(x)!:`,
    `Merge branch …` as a hand-written subject.
  * A user-visible breaking change **must** carry `!` and a `BREAKING CHANGE:` footer, and the
    completion report must state the computed release impact (`none`/`patch`/`minor`/`major`).
* One purpose per commit — implementation + tests + docs for the same feature travel together, and
  **every commit leaves the gate green** (the red phase is evidence in the report, not a committed
  state). Never mix refactoring with behaviour changes.
* Review the diff before committing (`git --no-pager diff --stat`, then the full diff); never
  `git add -A` blindly; never commit `target/`, criterion output, editor files or secrets.
* Branch naming when branches are used: `feat/<scope>`, `fix/<scope>`, `docs/<scope>`, `chore/<scope>`.
  Never rewrite published history and never force-push `main`. Do not commit unless the user asked for
  a commit **or** the task plan explicitly included "commit" as a deliverable step.
* **Human-facing mirror**: `CONTRIBUTING.md` is the on-ramp for *people* (setup, branch/commit/PR rules,
  the local gate). Whenever you change branch, commit, PR or gate rules, update `CONTRIBUTING.md` in the
  same commit — a contributor guide that contradicts CI is worse than none (§8.3).
* Do **not** hand-write release notes: `CHANGELOG.md` is generated by release-plz from the commit
  messages above (§8.3). Only the `## [Unreleased]` section may be edited by hand, and only for
  something a commit cannot express (rare). Behaviour changes are tracked in `docs/PARITY.md` and
  `docs/DIVERGENCES.md`, not in the changelog.

### 5.3 Definition of Done (all boxes, for every task)

- [ ] Plan published **before** implementation (§5.1) and the task is a single logical feature (§5.2).
- [ ] Tests were written first; the red output was captured and, in the report, the green run is shown.
- [ ] Every applicable §4.3 edge case has a test; applicable §4.5 properties are covered.
- [ ] Public items have Rustdoc with a runnable `# Examples` doctest, plus `# Panics`/`# Errors`/
      `# Performance`/`# Unicode`/`# Parity` where they carry information.
- [ ] API shape follows §3.3 (`Cow`/`_into`/iterator variants, `#[must_use]`, enums over strings).
- [ ] No `unsafe`, no `unwrap`/`expect`/`panic!` in library code, no new dependency (`P1`, `P2`).
- [ ] Byte-boundary safety is asserted and tested (`P5`).
- [ ] `docs/PARITY.md` updated for the symbol(s) implemented; `docs/DIVERGENCES.md` updated for any
      intentional difference; **no hand-written version number or release-notes entry** (§8.7).
- [ ] The commit message (or PR title) is a valid Conventional Commit and the report states the release
      impact (`none`/`patch`/`minor`/`major`) per §8.2.
- [ ] Benchmarks added/updated when a hot path changed, and the comparison was run.
- [ ] **Full quality gate (§4.8) passes**, with the raw output included in the report.
- [ ] The completion report (§5.6) is posted, including anything "NOT VERIFIED" and why.
- [ ] Every commit message and git operation was approved by the user before it ran, or the task carried
      explicit pre-authorisation (§5.7).
- [ ] Working tree contains only intended changes (`git --no-pager status`, `git --no-pager diff --stat`).

### 5.4 Red lines — never do these

1. **Never** add a runtime dependency or a feature that pulls one without explicit user approval and an
   ADR (`P1`).
2. **Never** write `unsafe` (`P2`).
3. **Never** ship library code that can panic on user input, and never use `unwrap`/`expect`/`panic!` in
   `src/` outside `#[cfg(test)]` (`P2`).
4. **Never** slice a `&str` with a hand-computed byte offset, and never assume one byte per Arabic
   character (`P5`).
5. **Never** copy code, docstrings, tables or tests from GPL upstream sources (§2.1).
6. **Never** weaken, `#[ignore]`, `#[allow]`-away or delete a failing test to make the gate pass — fix
   the code or change the contract explicitly.
7. **Never** declare a task complete with failing gates, or claim a gate passed when it did not run
   (state "NOT VERIFIED" instead).
8. **Never** mix unrelated refactors, renames or reformatting into a feature change (`P11`).
9. **Never** commit generated artifacts (`target/`, criterion output), editor files, secrets, or
   machine-specific paths.
10. **Never** break a public API silently: it needs an ADR, a `!`/`BREAKING CHANGE:` commit (which is
    what records it in the changelog, §8.2) and, where relevant, a `DIVERGENCES` note plus a deprecation
    path when feasible.
11. **Never** introduce locale/time/environment dependence, global mutable state or `HashMap`-iteration
    order dependence (`P6`).
12. **Never** present performance claims without benchmark output, or add a cache/optimization "because
    it feels faster" (`P10`).
13. **Never** silently "fix" upstream behaviour: if you deviate, document it in `docs/DIVERGENCES.md`
    with the rationale and the test that pins it (`P7`).
14. **Never** invent upstream API names or semantics: verify against upstream source/docs, and mark
    anything unverified in `docs/PARITY.md` as `TBD` rather than assuming.
15. **Never** state that something works without having executed it in this environment (or clearly
    reporting that execution was impossible).
16. **Never** hand-edit `[package] version` in `Cargo.toml`, an already-released `CHANGELOG.md`
    section, or create/rename/delete a git tag — release-plz owns versioning, tags and changelogs
    (§8.7). The only exception is the documented one-time `v0.1.0` bootstrap (§8.6).
17. **Never** merge a release PR, push a tag, or publish to crates.io on your own initiative; those are
    maintainer decisions, and publishing is disabled in `release-plz.toml` on purpose (§8.5).
18. **Never** write a commit or PR title that does not follow Conventional Commits (§5.2): CI rejects
    it, and release-plz would derive the wrong next version (§8.2).
19. **Never** disable, `continue-on-error:` or "temporarily" weaken `conventional-commits.yml`,
    `release.yml` or their checks in `ci.yml` (§8.4).
20. **Never** run a history- or remote-changing git command (`commit`, `branch`, `tag`, `rebase`, `reset`,
    `push`, …) without an approved proposal — exact commands, verbatim message, expected state (§5.7).

### 5.5 Ambiguity, divergences & decision records

**Ask the user (before implementing)** when: the spec is ambiguous; upstream behaviour conflicts with
the Unicode Standard; the change is API-breaking; a dependency is involved; licensing/provenance is
unclear; or scope would grow beyond the plan. Ask with 2–4 concrete options **and** state the default
you would take — never leave a question open-ended and never guess silently.

**Document divergences** in `docs/DIVERGENCES.md`, one section per item, in this shape:

```markdown
## D-004 — `unshape` does not reverse text

- **Upstream:** `pyarabic.unshape.unshaping_word(text)` maps presentation forms **and** reverses the
  character order (visual-order rendering hack).
- **r_abic:** `unshape::to_base_letters(text)` maps presentation forms only; reversal is a separate,
  explicitly named function.
- **Rationale:** implicit reversal corrupts any byte/char metadata the caller holds and makes the
  function non-composable with tokenization; the two concerns are unrelated.
- **Decision:** ADR-0003 · **Pinned by:** `tests/unshape.rs::to_base_letters_preserves_character_order`
- **Status:** intentional, permanent
```

**Record decisions** in `docs/adr/000N-short-title.md` with `Status` / `Context` / `Decision` /
`Consequences` / `Alternatives considered`. ADRs are append-only: to change a decision, add a new ADR
that supersedes the old one and update its status line to `Superseded by ADR-000M`.

**Undefined upstream behaviour** (e.g. what `strip_last_haraka` does with a shadda-only word): choose
the behaviour that is consistent with the Unicode Standard and the rest of our API, document it in
`DIVERGENCES.md`, pin it with a test, and list it under "needs maintainer confirmation" in the report.

**Track parity** in `docs/PARITY.md` as a table, updated in the same commit as the code:

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `pyarabic.araby.strip_tashkeel` | `r_abic::normalize::strip_tashkeel` | `todo` / `wip` / `done` / `divergent` / `out-of-scope` | D-00N if divergent |

Statuses are limited to those five values. `TBD` in Notes means "upstream semantics not yet verified" —
never mark something `done` without a test and verified parity.

### 5.6 Completion report format (end of every task)

```markdown
## Task
<one line: the feature/fix, with scope>

## Plan
<the published plan, abbreviated to the decisions that mattered>

## Tests first (red → green)
Red: `<exact command>` → <failure summary, e.g. "3 tests failed: …">
Green: `<exact command>` → `test result: ok. N passed; 0 failed`

## Implementation
- `src/normalize.rs` — strip_tashkeel (+ `_into`), Cow fast path
- `src/lib.rs` — module wiring, lints
- Key decisions: <e.g. "kept upstream name strip_lastharaka for parity, added doc alias">

## Quality gate (raw, abbreviated)
- G1 fmt: clean
- G2 clippy: clean (`Finished`, 0 warnings)
- G3 tests: `test result: ok. 41 passed; 0 failed` (unit + integration + doc)
- G4 doctests: `test result: ok. 12 passed; 0 failed`
- G5 docs: clean with `-D warnings`
- G6 benches: compiled
- G7 deps: `r_abic v0.1.0` only
- G8: not run (cargo-deny not installed) — **NOT VERIFIED**

## Docs & tracking updated
`docs/PARITY.md` (2 rows → done), `docs/DIVERGENCES.md` (D-004 added), no hand-written CHANGELOG entry

## Release impact
`minor` — driven by `feat(normalize): add strip_tatweel`; no version, tag or released-CHANGELOG edit
by hand (§8.2, §8.7)

## Not verified / caveats / follow-ups
<explicit list; state anything you could not execute and why>

## Next logical task
<one line suggestion, e.g. "strip_harakat (keeps shadda) + E5/E19 coverage">
```

Rules: never claim a gate passed without pasting its result; never report "done" with a failing gate;
keep it skimmable; mention every file you created or modified; if you could not run something, say so in
"Not verified".

### 5.7 Git operations require a human green light

**Default: no history- or remote-changing git command runs until the user has seen exactly what will
happen and approved it.**

* **Allowed without asking (read-only):** `status`, `diff`, `log`, `show`, `branch`, `ls-files`, `grep`,
  `ls-remote`, `remote -v`, `rev-parse`, `for-each-ref`. Report anything surprising they reveal.
* **Allowed but must be announced:** `fetch` (it only moves remote-tracking refs) and `add` /
  `restore --staged` (index only, reversible — they are how a proposal is assembled).
* **Requires explicit approval, every time:** `commit`, `commit --amend`, `rebase`, `cherry-pick`,
  `merge`, `revert`, `reset` (any mode), `switch`/`checkout` that changes branch, `stash
  pop/apply/drop`, `branch -d/-D`, `tag`, `mv`, every `push` (including `--force-with-lease`),
  `push --delete`, `remote add/remove/set-url`, repo-local `config` writes, and any history rewrite.
* **A proposal must contain:** (1) the exact commands in order; (2) the exact paths affected; (3) the
  **verbatim commit message** (subject *and* body); (4) the expected resulting state (branch,
  divergence, what lands where, what is pushed); (5) every destructive effect named explicitly
  (e.g. "`reset --hard` discards the 3 uncommitted edits in `src/x.rs`").
* **The green light must be explicit and specific to that proposal.** "ok", "thanks", "continue" or a
  new question are *not* approval. If the proposal changes by a byte — reworded subject, extra file,
  different base branch — ask again.
* **Pre-authorisation is per task:** "commit and push when the gate is green" approves exactly that,
  expires with the task, and never covers tags, force-pushes, merging protected branches or releases
  (§8.7).
* **Before stacking** any commit on an existing branch, `fetch` and confirm it is not already merged
  (`git branch --merged origin/main`); if it is, cut a fresh branch from `origin/main` instead.
* **Ambiguity resolves to waiting.** An agent that asks is never wrong; an agent that puts the wrong
  message into tagged history is. Proactivity means preparing the change, the message and the evidence —
  not running the git command.

## 6. Roadmap & parity tracker

Work proceeds in phases; a phase is done only when its exit criteria are met **and** the gate is green.
The authoritative per-symbol tracker is `docs/PARITY.md` (§5.5), seeded in Phase 0 from the module map
in §3.2 with every symbol marked `todo`.

### Phase 0 — foundation (do this first)

Deliverables: `src/error.rs` (`Error`/`Result`); `src/unicode/{mod,constants,tables,classify}.rs` with
code-point constants, class groups, predicates and the table-validation tests; crate lints + curated
crate docs in `lib.rs`; `Cargo.toml` metadata (`rust-version`, `description`, `keywords`, `categories`,
`repository`, `documentation`, `#![doc]`-friendly docs); `docs/PARITY.md`; `docs/DIVERGENCES.md`;
`docs/adr/0001-record-architecture-decisions.md` and `0002-licensing-and-provenance.md`;
`.github/workflows/{ci,conventional-commits,release}.yml` (§7, §8), `release-plz.toml` (§8.3);
`README.md` quick start; `CHANGELOG.md` with an `Unreleased` section.

Exit criteria: gate green on all gates that exist at that point; `cargo doc` renders constants and
predicates with examples; every upstream symbol appears in `PARITY.md` with a status.

### Phase 1 — core text hygiene (MVP)

Deliverables: `normalize::strip_tashkeel`, `strip_harakat`, `strip_lastharaka`, `strip_tatweel`,
`strip_shadda`, `strip_diacritics`, `reduce_tatweel`, `reduce_tashkeel`; `normalize_hamza`,
`normalize_ligature` (+ `normalize_lamalef` alias), `normalize_spellerrors`, `normalize_searchtext`;
`diacritics::separate`, `joint`; `tokenize::tokens`, `tokenize`, `fix_spaces`.

Exit criteria: E1–E13 and E18–E20 covered for these functions; zero-copy assertions present; benchmarks
for `strip_tashkeel`/`separate`/`tokenize` exist and are recorded; every symbol `done` or `divergent` in
`PARITY.md`.

### Phase 2 — shaping, transliteration, tashkeel codec

Deliverables: `unshape::to_base_letters` (all four ligature pairs, E14–E16) + reversal helpers (E17);
`trans::convert` (Arabic ↔ TIM/Buckwalter ↔ SAMPA), `normalize_digits` (E8),
`segment_language`/`delimite_language`; `diacritics::encode_tashkeel`/`decode_tashkeel` (N3);
`diacritics::vocalizedlike`/`vocalized_similarity`.

Exit criteria: round-trip property tests pass; lossy conversions documented in `DIVERGENCES.md`.

### Phase 3 — numbers, words, extras

Deliverables: `numbers` (`number2text`/`number2letters`, `number2ordinal`, digit helpers, phrase
detection, vocalization aids) with N1/N2 coverage; `kalima::Word`; `soundex`; `unicode::names`
(`name`, `glyph`, `spellit`).

Exit criteria: grammar tables reviewed against the documented rules; `NumberOutOfRange` behaviour tested;
`numbers` benchmark recorded.

### Phase 4 — hardening & 0.1.0 release

Deliverables: MSRV + beta CI legs green; `no_std + alloc` feasibility ADR (Tier-2, optional); extended
fuzz-lite corpus; benchmark baselines committed as documentation (numbers only, not HTML); docs.rs
metadata verified; `README` examples executed in CI; `CONVENTIONS` review against this file; the first
automated release cut by release-plz (git tag + GitHub Release, §8.6) with `publish` still `false`
unless an ADR says otherwise (§8.5).

Exit criteria: all gates green on Linux/Windows/macOS × stable/beta/MSRV; `cargo publish --dry-run`
succeeds (or is reported NOT VERIFIED with the reason).

---

## 7. CI specification

`.github/workflows/ci.yml` (created in Phase 0; treat as a contract — changes need a `chore(ci)` commit
and a note in the report):

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:

permissions:
  contents: read

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -D warnings

jobs:
  quality:
    name: fmt + clippy + docs
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets --all-features -- -D warnings
      - run: RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features

  test:
    name: test (${{ matrix.os }}, ${{ matrix.rust }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - { os: ubuntu-latest,  rust: stable }
          - { os: windows-latest, rust: stable }
          - { os: macos-latest,   rust: stable }
          - { os: ubuntu-latest,  rust: beta }
          - { os: ubuntu-latest,  rust: "1.85" }   # MSRV from Cargo.toml
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@master
        with:
          toolchain: ${{ matrix.rust }}
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --all-features
      - run: cargo test --doc --all-features
      - run: cargo bench --no-run
```

### 7.1 Conventional Commits enforcement (`.github/workflows/conventional-commits.yml`)

Two jobs, both plain `bash` on purpose (no third-party action to audit or pin):

* `pr-title` — validates `github.event.pull_request.title` against the pattern in §5.2, on `opened`,
  `edited`, `synchronize` and `reopened`.
* `commit-subjects` — validates every commit subject of the PR range (`base.sha..head.sha`) and, on
  `push`, of the pushed range (`before..sha`); on a brand-new branch it validates the tip commit. Merge
  commits and git's default `Revert "…"` message are exempt (prefer `revert: …`).

The PR title is passed through `env:` and referenced as `"$TITLE"` — never interpolated with `${{ … }}`
inside a `run:` block, which would be a shell-injection vector (a PR title is attacker-controlled text).

### 7.2 Release automation (`.github/workflows/release.yml` + `release-plz.toml`)

Runs on `push` to `main` and on `workflow_dispatch`; two jobs following the official release-plz
pattern:

| Job | Command | Permissions | Effect |
|---|---|---|---|
| `release-plz-release` | `release-plz release` | `contents: write`, `pull-requests: read` | tags the version already in `Cargo.toml` and creates the GitHub Release — only once the release PR is merged (`release_always = false`) |
| `release-plz-pr` | `release-plz release-pr` | `contents: write`, `pull-requests: write` | opens/updates the release PR: bumps `Cargo.toml` + `Cargo.lock`, regenerates `CHANGELOG.md` |

Both jobs check out with `fetch-depth: 0` (release-plz needs the full history *and* the tags) and
`persist-credentials: false`, skip forks (`if: github.repository_owner == 'imadfen'`), and the PR job
runs with `concurrency: cancel-in-progress: false` so a queued release is never cancelled.

Unlike `ci.yml` (embedded above as a contract), `release.yml` is **not** duplicated in this document:
the workflow file plus `release-plz.toml` are the contract, and §8 is the policy they implement. When
changing release behaviour, change those files (and this section) in one `chore(ci)` / `docs` commit.

Notes: the MSRV leg pins the `rust-version` value declared in `Cargo.toml` — if you raise it, update both
files in the same commit. CI compiles benches but never runs them (timing noise). `RUSTFLAGS: -D warnings`
applies to all jobs so that even `rustc` warnings fail the build. Optional extra job (add with an ADR):
`cargo deny check` for licenses/advisories. This environment has no GitHub runner, so workflow execution
must be reported as NOT VERIFIED here — the *local* equivalent of every job is §4.8.

## 8. Versioning & releases (automated)

Releases are **derived from history, never typed by hand**: git tags, `Cargo.toml` versions,
`Cargo.lock` and `CHANGELOG.md` are produced by [`release-plz`](https://release-plz.dev) from
Conventional Commit messages. `release-plz.toml` + `.github/workflows/release.yml` are the
implementation; this section is the policy.

### 8.1 Single source of truth

* The version lives **only** in `Cargo.toml` (`[package] version`) and is mirrored by `Cargo.lock`, the
  git tag (`vX.Y.Z`) and the top `CHANGELOG.md` section. release-plz keeps all four in sync; nobody
  else touches them (§8.7).
* No version numbers in prose: `README.md`, Rustdoc and `docs/*.md` describe behaviour, not releases.
  Write "as of the current release" or link to the changelog instead of hard-coding `0.1.0`.
* SemVer applies to the **public API** (§3.3). Adding a variant to a `#[non_exhaustive]` enum is not
  breaking; adding a `pub` item is `MINOR`; changing or removing one is `MAJOR`; a behaviour change that
  silently changes output for existing input is `MAJOR` for `1.x` and `MINOR` for `0.x`.
* Pre-1.0 (`0.y.z`) the crate is explicitly unstable: `MAJOR` bumps are emulated by `MINOR` bumps
  (SemVer §4). This is why `BREAKING CHANGE` while `0.x` produces `0.(y+1).0`, not `1.0.0`.

### 8.2 Commit type → release impact

| Commit / PR title | Version bump | `CHANGELOG.md` section |
|---|---|---|
| `feat: …`, `feat(scope): …` | **MINOR** (`0.1.0 → 0.2.0`) | `### Added` |
| `fix: …`, `fix(scope): …` | **PATCH** (`0.1.0 → 0.1.1`) | `### Fixed` |
| `feat!: …`, `fix(scope)!: …`, or any commit with a `BREAKING CHANGE:` footer | **MAJOR** (`1.2.3 → 2.0.0`); **MINOR** while `0.x` | the type's section (never skipped) |
| `perf: …` | none on its own | `### Performance` |
| `refactor: …` | none | `### Changed` |
| `revert: …` | none | `### Reverted` |
| `docs: …` | none | `### Documentation` |
| `chore(deps): …` | none | `### Dependencies` |
| `test:`, `style:`, `ci:`, `chore:`, `build:` | none | skipped (`release-plz.toml`) |

Consequences to internalise:

* A release happens **only** when at least one `feat`, `fix` or breaking commit landed since the last
  tag. A docs-only or CI-only change produces **no** release PR — intended, not a bug.
* The **PR title becomes the commit subject** on squash-merge, so a mislabelled PR silently changes the
  next version. That is exactly why `.github/workflows/conventional-commits.yml` checks both (§7.1).
* Bump decisions live in the commit message, not in a file. If a change is breaking for users, `!` plus
  a `BREAKING CHANGE:` footer is mandatory, and the completion report states the impact (§5.6).

### 8.3 Tooling and files

| File | Role |
|---|---|
| `release-plz.toml` | configuration: changelog generation and grouping, PR labels/prefix, `release_always = false`, `publish = false`, `semver_check = false` |
| `.github/workflows/release.yml` | `release-plz release` (tag + GitHub Release) and `release-plz release-pr` (version bump + changelog) on every push to `main` (§7.2) |
| `.github/workflows/conventional-commits.yml` | rejects non-conventional PR titles and commit subjects (§7.1) |
| `.github/workflows/ci.yml` | the §4.8 gate; runs on every PR, including the release PR |
| `CHANGELOG.md` | managed by release-plz (generated sections); only `## [Unreleased]` is hand-editable |
| `CONTRIBUTING.md` | human on-ramp for the rules above; must stay in sync with §5.2/§7/§8 |

**Why release-plz and not `cargo-release`:** release-plz does the whole loop (compute the next version,
update `Cargo.toml` + `Cargo.lock`, regenerate the changelog with git-cliff, tag, create the GitHub
Release, and expose it all as a reviewable dry-run PR) with no local state, and its GitHub Action is the
one this repository is wired against. Bumping `release-plz/action@v0.5` or the `actions/checkout` major
is a `chore(ci)` task that must update **all** workflows in the same commit, with the new behaviour
described in this section.

### 8.4 Secrets, permissions and repository settings

| Setting | Value | Why |
|---|---|---|
| `Settings → Actions → General → Workflow permissions` | **Allow GitHub Actions to create and approve pull requests** | release-plz has to open the release PR |
| Secret `RELEASE_PLZ_TOKEN` *(recommended)* | fine-grained PAT with `contents: write` + `pull-requests: write` on this repo | events created with `GITHUB_TOKEN` do **not** trigger other workflows, so the release PR would never run CI |
| Secret `CARGO_REGISTRY_TOKEN` *(only when publishing)* | crates.io token scoped `publish-new` + `publish-update` | `cargo publish` |
| Branch protection on `main` | require the `ci.yml` and `conventional-commits.yml` checks | keeps `main` releasable at all times |
| Tag protection on `v*` | forbid deletion / non-fast-forward updates | tags are immutable release history |

The workflow falls back to `secrets.GITHUB_TOKEN` when `RELEASE_PLZ_TOKEN` is absent, so the pipeline
works on a fresh repository — but then the release PR starts without CI, which is a review hazard, not a
feature. Never echo a secret, never add `set -x` to a step that receives one, and never write a token
into a file that survives the step.

### 8.5 Publishing policy (crates.io)

`release-plz.toml` ships with **`publish = false`**: the automation creates the git tag and the GitHub
Release but does **not** upload to crates.io. Enabling publishing requires, in one reviewed change:
`publish = true`, `semver_check = true`, a `docs/adr/000N-enable-crates-io-publishing.md` record, and a
maintainer performing the **first** `cargo publish` manually (crates.io cannot create a brand-new crate
through trusted publishing). Until then, `cargo publish --dry-run` is the only publishing-flavoured
command an agent may run, and `cargo publish` itself is a red line (§5.4 item 17).

### 8.6 The release flow (what actually happens)

```text
feature branch ──PR (conventional title)──► squash-merged into main
                                              │
              ┌───────────────────────────────┴──────────────────────────────┐
              │ push to main → release.yml                                   │
              │  1. release-plz release     : nothing to publish yet         │
              │  2. release-plz release-pr  : opens/updates the release PR   │
              │     "chore: release vX.Y.Z" — bumps Cargo.toml + Cargo.lock, │
              │     regenerates CHANGELOG.md from the commits since the tag  │
              └───────────────────────────────┬──────────────────────────────┘
                                              │ review + merge the release PR
                                              ▼
              release-plz release → git tag vX.Y.Z + GitHub Release (+ crates.io
              only when `publish = true`)
```

Rules for humans and agents:

* Review the release PR like any other PR; its diff must be exactly `Cargo.toml`, `Cargo.lock`,
  `CHANGELOG.md`. Anything else means a stray change slipped into `main` — investigate before merging.
* Never edit the release PR by hand. Push a new conventional commit to `main` and release-plz updates
  the PR (its branch is force-updated by the tool, never by you).
* If the release PR proposes the wrong bump, fix the commit type/`!` marker (or the PR title), not the
  version number (§8.7).
* **One-time bootstrap** (this repository currently has no tags): after the first commits land, merge
  the first release PR (release-plz tags whatever version `Cargo.toml` holds) or — if the very first
  release must be `v0.1.0` exactly — tag it by hand **once**, then reconcile the hand-written
  `## [Unreleased]` notes into the generated `## [0.1.0]` section and delete them. State that you did
  this in the completion report; afterwards tagging is exclusively release-plz's job.
* A failed `release-plz release` run is retried by re-running the workflow (`workflow_dispatch`), never
  by pushing tags manually.

### 8.7 What agents must never do

* Never change `[package] version` in `Cargo.toml` by hand — release-plz owns it; a manual bump either
  creates a bogus release or leaves a version that no tag matches.
* Never run `git tag`, `git push --tags`, `git tag -d`, `git push --force` on a tag, or move an existing
  tag. The only exception is the documented one-time bootstrap (§8.6), and only when the user asks.
* Never hand-edit an already-released `CHANGELOG.md` section. Notes go under `## [Unreleased]` and are
  folded into the next release PR (or become a `docs:` commit when they are documentation only).
* Never merge a release PR, cut a release or publish to crates.io on your own initiative.
* Never rewrite `main`'s history once a tag exists: a tag pins commits, and rewriting them breaks SemVer
  for everyone who pinned a version.
* Never disable, `continue-on-error:` or "temporarily weaken" `conventional-commits.yml`, `release.yml`
  or their required checks in `ci.yml`.
* Never add a `version`-bumping tool (`cargo set-version`, `cargo-release`, `cargo-edit`, `sed -i` on
  `Cargo.toml`) to scripts, docs or workflows: two tools writing the same field is how releases rot.

### 8.8 Local dry-runs (optional, when the tools are installed)

```powershell
cargo install release-plz --locked        # same binary the action runs
release-plz update --dry-run              # prints the computed next version + changelog diff, changes nothing
git log --oneline "$(git describe --tags --abbrev=0)..HEAD"   # commits since the last tag
git --no-pager tag --sort=-v:refname      # tags release-plz has created, newest first
```

If `release-plz` is not installed (it is not part of this development environment), say **NOT VERIFIED**
in the report instead of guessing the next version: the commit types in §8.2 are the ground truth, and
the authoritative computation happens in CI.

---

## Appendix A — Arabic / Unicode quick reference

**Blocks that matter** (anything outside these is *unclassified* → passed through unchanged):

| Block | Range | What we use it for |
|---|---|---|
| Arabic | `U+0600–U+06FF` | punctuation, letters, ḥarakāt, digits, tatweel |
| Arabic Supplement | `U+0750–U+077F` | extended letters — not classified today, documented as pass-through |
| Arabic Extended-A | `U+08A0–U+08FF` | Qur'anic marks — not classified today, documented as pass-through |
| Arabic Presentation Forms-A | `U+FB50–U+FDFF` | ligatures (only the lam-alef pairs are handled today) |
| Arabic Presentation Forms-B | `U+FE70–U+FEFF` | contextual forms; `unshape` handles `U+FE80–U+FEFC` |
| Combining Diacritical Marks | `U+0300–U+036F` | may appear in NFD input; never classified as Arabic marks → preserved |

**Letters** (all `2` bytes in UTF-8): `HAMZA U+0621`, `ALEF_MADDA U+0622`, `ALEF_HAMZA_ABOVE U+0623`,
`WAW_HAMZA U+0624`, `ALEF_HAMZA_BELOW U+0625`, `YEH_HAMZA U+0626`, `ALEF U+0627`, `BEH U+0628`,
`TEH_MARBUTA U+0629`, `TEH U+062A`, `THEH U+062B`, `JEEM U+062C`, `HAH U+062D`, `KHAH U+062E`,
`DAL U+062F`, `THAL U+0630`, `REH U+0631`, `ZAIN U+0632`, `SEEN U+0633`, `SHEEN U+0634`,
`SAD U+0635`, `DAD U+0636`, `TAH U+0637`, `ZAH U+0638`, `AIN U+0639`, `GHAIN U+063A`,
`TATWEEL U+0640`, `FEH U+0641`, `QAF U+0642`, `KAF U+0643`, `LAM U+0644`, `MEEM U+0645`,
`NOON U+0646`, `HEH U+0647`, `WAW U+0648`, `ALEF_MAKSURA U+0649`, `YEH U+064A`,
`ALEF_WASLA U+0671`.

**Marks — the classes the crate is built around:**

| Class | Members | Meaning |
|---|---|---|
| `HARAKAT` | `FATHATAN U+064B`, `DAMMATAN U+064C`, `KASRATAN U+064D`, `FATHA U+064E`, `DAMMA U+064F`, `KASRA U+0650`, `SUKUN U+0652` | short vowels + tanwīn + sukūn |
| `TASHKEEL` | `HARAKAT` + `SHADDA U+0651` | everything a "full" diacritization contains |
| `SHORTHARAKAT` | `FATHA`, `DAMMA`, `KASRA`, `SUKUN` | short ḥarakāt without tanwīn/shadda |
| `TANWIN` | `FATHATAN`, `DAMMATAN`, `KASRATAN` | the three tanwīn marks |
| extra marks | `MADDA_ABOVE U+0653`, `HAMZA_ABOVE U+0654`, `HAMZA_BELOW U+0655`, `MINI_ALEF/SUPERSCRIPT_ALEF U+0670`, `SMALL_WAW U+06E5`, `SMALL_YEH U+06E6` | combining forms used by `strip_diacritics`/hamza normalization |

**Digits**: west/ASCII `0–9` (`U+0030–U+0039`, 1 byte), Arabic-Indic `٠–٩` (`U+0660–U+0669`, 2 bytes),
Extended Arabic-Indic (Persian/Urdu) `۰–۹` (`U+06F0–U+06F9`, 2 bytes). The same char count, different
byte length — never carry byte offsets across digit conversion.

**Punctuation & typography**: `COMMA U+060C`, `SEMICOLON U+061B`, `QUESTION U+061F`, `PERCENT U+066A`,
`DECIMAL_SEPARATOR U+066B`, `THOUSANDS_SEPARATOR U+066C`, `ARABIC_STAR U+066D`, `FULL_STOP U+06D4`.

**Formatting / invisible**: `ALM U+061C`, `ZWNJ U+200C`, `ZWJ U+200D`, `LRM U+200E`, `RLM U+200F`,
`BOM/ZWNBSP U+FEFF`. Never dropped implicitly; classification is explicit and tested (E11–E13).

**Lam-alef ligatures**: `U+FEF5/U+FEF6` (madda above), `U+FEF7/U+FEF8` (hamza above),
`U+FEF9/U+FEFA` (hamza below), `U+FEFB/U+FEFC` (plain) — each expands to `LAM` + the corresponding
alef form. Contextual forms: `U+FE80–U+FEFC` (isolated/initial/medial/final).

**UTF-8 sizes you must remember**: Arabic base letter 2 bytes · ḥaraka 2 bytes · Arabic-Indic digit
2 bytes · presentation form 3 bytes · lam-alef ligature 3 bytes · astral char (emoji) 4 bytes. A
diacritized word like `الْعَرَبِيَّةُ` is *not* one byte per visible letter; never use `len()` as
"number of letters" and never index with arithmetic — use `chars().count()` and `char_indices()`.

**Class groups kept for parity** (`P7`): `LIGUATURES` (4), `HAMZAT` (7), `ALEFAT` (7: alef, alef-madda,
alef-hamza-above, alef-hamza-below, alef-wasla, alef-maksura, small-alef), `LETTERS` (36), `WEAK`,
`YEHLIKE`, `WAWLIKE`, `TEHLIKE`, `SMALL`, `MOON` (18 solar-assimilation-free letters: `ء آ أ إ ا ب ج ح خ
ع غ ف ق ك م ه و ي`), `SUN` (14: `ت ث د ذ ر ز س ش ص ض ط ظ ل ن`), plus `ALPHABETIC_ORDER` and `NAMES`.

## Appendix B — PyArabic → `r_abic` parity checklist

Names below were extracted from upstream `master` sources and the published docs on **2026-10-31**.
Entries marked **`(verify)`** must be re-checked against a local upstream checkout before being marked
`done` (red line #14) — do this once in Phase 0 and correct this appendix if anything differs:

```powershell
# one-time, OUTSIDE the crate, never committed, never vendored (GPL — see §2.1)
New-Item -ItemType Directory -Force tools/oracle | Out-Null
Invoke-WebRequest 'https://raw.githubusercontent.com/linuxscout/pyarabic/master/pyarabic/araby.py' `
  -OutFile tools/oracle/araby.py
rg -n '^(def|class) ' tools/oracle/araby.py
```

Mirror the same command for `normalize.py`, `number.py`, `trans.py`, `unshape.py`, `kalima.py`,
`soundex.py`, then reconcile `docs/PARITY.md` line by line. Keep `tools/oracle/` git-ignored.

### B.1 `unicode` — constants & letter classification

* [ ] `araby.WAW`-family single-letter constants (`COMMA`…`YEH`) → `unicode::constants::*` (`P0`)
* [ ] `araby.HAMZA`, `ALEF_MADDA`, `ALEF_HAMZA_ABOVE`, `WAW_HAMZA`, `ALEF_HAMZA_BELOW`, `YEH_HAMZA`
* [ ] `araby.TEH_MARBUTA`, `ALEF_MAKSURA`, `TATWEEL`, `ALEF_WASLA`, `MINI_ALEF` (=`SMALL_ALEF`)
* [ ] ḥaraka constants `FATHATAN`, `DAMMATAN`, `KASRATAN`, `FATHA`, `DAMMA`, `KASRA`, `SHADDA`, `SUKUN`
* [ ] digit constants `ZERO`…`NINE` (east), `ZERO_W`…`NINE_W` (west), `ZERO_P`…`NINE_P` (persian)
* [ ] typography `PERCENT`, `DECIMAL`, `THOUSANDS`, `STAR`, `FULL_STOP`, `SEMICOLON`, `QUESTION`
* [ ] `BYTE_ORDER_MARK`, `SMALL_WAW`, `SMALL_YEH`, `MADDA_ABOVE`, `HAMZA_ABOVE`, `HAMZA_BELOW`
* [ ] group tuples `HARAKAT`, `TASHKEEL`, `SHORTHARAKAT`, `TANWIN`, `NOT_DEF_HARAKA`
* [ ] `LIGUATURES`, `HAMZAT`, `ALEFAT`, `LETTERS`, `WEAK`, `YEHLIKE`, `WAWLIKE`, `TEHLIKE`, `SMALL`, `MOON`, `SUN`
* [ ] string forms `HAMZAT_STRING`, `HARAKAT_STRING`, `TASHKEEL_STRING` (**not** ported as strings — we
      match on `char` classes; document the mapping in `DIVERGENCES.md`)
* [ ] `ALPHABETIC_ORDER`, `NAMES` maps → `unicode::tables` (phase 3, `NAMES` depends on `unicode::names`)
* [ ] `is_hamza(char)`, `is_hamza_above(char)`, `is_hamza_below(char)` **(verify spelling)**
* [ ] `is_vowel(char)`, `is_shadda(char)`, `is_sukun(char)`, `is_tatweel(char)`, `is_ligature(char)` **(verify)**
* [ ] `is_letter(char)` **(verify)** and `is_arabicletter`/`is_arabicletterrange` **(verify)**
* [ ] `is_arabicrange(text)` — all characters in the Arabic ranges
* [ ] `is_arabicword(word)` — word-restricted variant (no spaces/digits/punctuation) **(verify)**
* [ ] `is_arabicstring(text)` **(verify)** — legacy camelCase aliases (`isArabicrange`, `isArabicword`,
      `isArabicstring`) are intentionally **not** ported → `DIVERGENCES.md`
* [ ] `has_shadda(word)` **(verify)** and legacy `hasShadda` (not ported)
* [ ] `is_vocalized(word)` **(verify)** and legacy `isVocalized` (not ported)

### B.2 `normalize` and `diacritics`

* [ ] `strip_tashkeel(text)` → `normalize::strip_tashkeel` (+ `_into`) — template implementation (`P0`)
* [ ] `strip_harakat(text)` → keeps shadda, removes the other ḥarakāt
* [ ] `strip_lastharaka(text)` → `normalize::strip_lastharaka`; keep upstream spelling, add
      `#[doc(alias = "strip_last_haraka")]`
* [ ] `strip_tatweel(text)` → `normalize::strip_tatweel`
* [ ] `strip_shadda(text)` → `normalize::strip_shadda`
* [ ] `strip_diacritics(text)` → `normalize::strip_diacritics` (ḥarakāt **and** small letters: tanwīn,
      shadda, sukūn, superscript alef, small waw/yeh)
* [ ] `reduce_tashkeel(text)` → `normalize::reduce_tashkeel` (upstream deletes "evident" cases)
* [ ] `reduce_tatweel(text)` → `normalize::reduce_tatweel`
* [ ] `normalize_hamza(word, method="uniform")` → `normalize::normalize_hamza` with
      `HamzaMethod { Uniform, Tasheel }` — **verified** against a local `tools/oracle/` checkout of
      `araby.py`: `"tasheel"` (the Arabic literal `"تسهيل"` is accepted too) folds each hamza onto its
      carrier — alef-madda, alef-hamza and the combining hamzas → `ALEF`, `WAW_HAMZA` → `WAW`,
      `YEH_HAMZA` → `YEH`; every other value, **including the `"uniform"` default**, replaces the whole
      `HAMZAT` class with bare `HAMZA` and expands `ALEF_MADDA` → `HAMZA`+`HAMZA`, with a special case
      for a word that starts with alef-madda. Upstream is *word*-scoped (it indexes `word[1]`/`word[2]`)
      and its docstring wrongly claims it also simplifies lam-alef — that is `normalize_ligature`'s job
      (pin both facts with tests when implementing)
* [ ] `normalize_ligature(text)` → `normalize::normalize_ligature`
* [ ] `normalize_lamalef(text)` → `normalize::normalize_lamalef` (alias of `normalize_ligature`, as upstream)
* [ ] `normalize_spellerrors(text)` → `normalize::normalize_spellerrors` (teh marbuta → heh, alef maksura → yeh)
* [ ] `normalize_searchtext(text)` → `normalize::normalize_searchtext` (pipeline: tashkeel → tatweel →
      ligature → hamza → spellerrors; order is part of the contract and tested)
* [ ] `fix_spaces(text)` **(verify owner module)** → `normalize::fix_spaces` or `tokenize::fix_spaces`
* [ ] `separate(word, extract=False)` → `diacritics::separate` (`(letters, marks)`), plus `separate_into`
* [ ] `joint(letters, marks)` → `diacritics::joint` (length mismatch → `Error::TashkeelLengthMismatch`)
* [ ] `vocalizedlike(word1, word2, extract=False)` **(verify signature)** → `diacritics::vocalizedlike`
* [ ] `vocalized_similarity(word1, word2)` → `diacritics::vocalized_similarity` (score semantics documented)

### B.3 `tokenize`, `kalima`, `unshape`, `trans`, `numbers`, `soundex`

**`tokenize`**
* [ ] `tokenize(text, conditions=None, morphs=None)` **(verify extra params)** → `tokenize::tokenize`
* [ ] `sentence_tokenize(text)` **(verify)** → `tokenize::sentence_tokenize`
* [ ] `tokenize_with_location(text)` **(verify name)** → `tokenize::tokens` (iterator with `start`/`end`)
      and `tokenize_with_location` (convenience `Vec` with byte ranges)
* [ ] `fix_spaces(text)` → `tokenize::fix_spaces` (collapse superfluous spaces; never touch inside words)
* [ ] `autocorrect(text)` → `tokenize::autocorrect` (duplicate ḥarakāt, tanwīn on alef, sukūn on
      alef/maksura/teh-marbuta, ḥaraka before shadda — each rule documented and pinned by a test)
* [ ] `spellit(word, lang="ar")` → `unicode::names::spellit` (phase 3; depends on `NAMES`)

**`kalima`**
* [ ] `Kalima(word, marks="")` → `kalima::Word::new` / `Word::from_letters_and_marks`
* [ ] `Kalima.letters`, `.marks`, `.word` → accessors + `letters()`/`marks()`/`vocalized()`
* [ ] `Kalima.has_shadda()` → `Word::has_shadda`
* [ ] `Kalima.is_vocalized()` → `Word::is_vocalized`
* [ ] `Kalima.is_arabic()` → `Word::is_arabic`
* [ ] `__eq__`/`__hash__` semantics (letters + marks) → `PartialEq`/`Eq`/`Hash` derive
* [ ] `__ge__` "similar vocalization" comparison → `Word::sounds_like`/documented partial order
      (Python operator overloading is not ported verbatim — document in `DIVERGENCES.md`)

**`unshape`**
* [ ] `UNSHAPING_TABLE` → `unshape::DE_SHAPING_TABLE` (all `U+FE80–U+FEFC`, **including** the pairs
      upstream omits: `U+FEF6`, `U+FEF8`, `U+FEFA`, `U+FEFB`)
* [ ] `unshaping_word(word)` → `unshape::to_base_letters` (**mapping only** — no reversal; `DIVERGENCES.md`)
* [ ] `unshaping_line(line)` → `unshape::to_base_letters` per line + explicit word-order reversal helper
* [ ] `unshaping_text(text)` → `unshape::to_base_letters` preserving newlines exactly (no spurious
      leading `\n` like upstream — pin with E17)
* [ ] text reversal ("قلب النصوص") → `unshape::reverse_chars` / `reverse_words` (explicit names, tested)

**`trans`**
* [ ] `translate(text, table)` → `trans::translate(text, &Table)` (internal, table-driven, no regex)
* [ ] `convert(text, src, dst)` → `trans::convert(text, TransliterationScheme, TransliterationScheme)`;
      unsupported pair → `Error::UnsupportedConversion`
* [ ] `tim2utf8` / `utf82tim` → `trans::tim_to_arabic` / `trans::arabic_to_tim` **(verify spellings)**
* [ ] `tim2sampa` → `trans::tim_to_sampa`
* [ ] `normalize_digits(nd_text, source="all", out="east")` → `trans::normalize_digits(text, DigitSet, DigitSet)`
* [ ] `segment_language(text)` → `trans::segment_language(text) -> Vec<Segment>` (downstream: language label)
* [ ] `delimite_language(text, start, end, language=None)` → `trans::delimit_language`
      (fix the upstream typo only with a `#[doc(alias = "delimite_language")]` and a `DIVERGENCES` entry)
* [ ] `encode_tashkeel(word, method="ascii")` → `diacritics::encode_tashkeel` (+ `TashkeelEncoding` enum)
* [ ] `decode_tashkeel(word, marks, method="ascii")` → `diacritics::decode_tashkeel` (N3)

**`numbers`**
* [ ] `ArNumbers().int2str(number)` → `numbers::number2text` / `number2letters` (pure functions, no state)
* [ ] `set_feminine(value)` / `set_format(value)` → typed options struct (`Gender`, `Case`) instead of
      mutable flags; invalid values → documentation of the enum's domain (no `bool` return hedge)
* [ ] `get_feminine()` / `get_format()` → read back from the options value
* [ ] `number2text(anumber)` / `number2letters` **(verify which exists)** → `numbers::number2text`
* [ ] `number2ordinal(anumber, feminin=False)` → `numbers::number2ordinal(value, Gender)` (N2)
* [ ] `detect_numbers`, `detect_number_words`, `extract_number_phrases`,
      `detect_number_phrases_position`, `pre_tashkeel_number`, `vocalize_number` → `numbers::detect_*`,
      `numbers::extract_number_phrases`, `numbers::pre_tashkeel_number`, `numbers::vocalize_number`
      *(all `(verify)` — module was only partially readable during the Phase 0 survey; extract the exact
      list with the command above before marking any of these `done`)*
* [ ] number tables from `number_const.py` (`INDIVIDUALS`, `COMPLICATIONS`, `UNITS_ORDINAL_WORDS`,
      `UNITS_ORDINAL_WORDS_FEMININ`, …) → re-derived `const` tables in `numbers::tables`
      (**re-derive from Arabic grammar, do not copy the upstream literals**)

**`soundex`**
* [ ] `soundex(word, length=6)` → `soundex::soundex(word, length)` (empty input → `Error::EmptyInput`)
* [ ] `soundex_map_code(word)` → `soundex::soundex_map_code`
* [ ] `soundex_trim_rep(word)` → `soundex::soundex_trim_rep`
* [ ] `SOUNDEX_CODE`, `SOUNDEX_TRANSLATION` → `soundex::tables`

**Out of scope (`P12`)** — record in `docs/DIVERGENCES.md` as `out-of-scope`, do not implement:
`named.py`, `named_const.py` (`RAFE3_LIST`, `JAR_LIST`, `NOUN_NASEB_LIST`, `PROPER_NOUNS`),
`propernouns.py` (~630 KB of lists), `stack.py` (Python-only helper; Rust iterators replace it),
`arabrepr.py` (rendering helper, not a text algorithm).

## Appendix C — Glossary

| Term | Meaning in this project |
|---|---|
| **ṭashkīl** (تَشْكِيل) | The full set of diacritical marks, i.e. `HARAKAT` + `SHADDA` (`TASHKEEL`). |
| **ḥarakāt** (حَرَكَات) | Short-vowel marks and sukūn/tanwīn: `FATHATAN…SUKUN` (excluding shadda). |
| **shadda** (شَدَّة) | `U+0651`, the gemination mark; kept by `strip_harakat`, removed by `strip_tashkeel`. |
| **sukūn** (سُكُون) | `U+0652`, the "no vowel" mark. |
| **tanwīn** (تَنْوِين) | The three nunation marks (`FATHATAN`, `DAMMATAN`, `KASRATAN`). |
| **tatweel / kashida** (تَطْوِيل / كَشِيدَة) | `U+0640`, a pure elongation joiner; never a letter, never a word boundary. |
| **hamza** (هَمْزَة) | `U+0621`; also appears carried on alef/waw/yeh (`ALEF_HAMZA_*`, `WAW_HAMZA`, `YEH_HAMZA`) and as combining `HAMZA_ABOVE/BELOW` (`U+0654/U+0655`). |
| **tasheel** (تَسْهِيل) | Hamza simplification: fold hamza onto its carrier letter (upstream `normalize_hamza(word, method="tasheel")`); the `"uniform"` default instead maps every hamza form to bare `HAMZA`. |
| **madda** (مَدَّة) | `U+0653`; `ALEF_MADDA` `U+0622` is alef + madda. |
| **wasla** (وَصْلَة) | `ALEF_WASLA U+0671`. |
| **kalima** (كَلِمَة) | "Word": the `(letters, marks)` pair type in `kalima::Word`. |
| **unshaping** | Converting presentation forms (`U+FE80–U+FEFC`) back to base letters. |
| **shaping / presentation form** | The contextual isolated/initial/medial/final glyph code points some producers emit. |
| **ligature** | Single code point representing two letters, here specifically the lam-alef pairs `U+FEF5–U+FEFC`. |
| **sentence_tokenize** | Splitting text into sentences (upstream name; kept for parity). |
| **soundex** | Phonetic encoding for homophone matching (upstream `soundex.py`). |
| **CoW / `Cow`** | Copy-on-write: `Cow::Borrowed` when the input is already in the target form, `Cow::Owned` otherwise. |
| **gate** | The §4.8 command set that must pass before a task may be reported as done. |
| **parity** | Behavioural equivalence with the corresponding PyArabic symbol(s). |
| **divergence** | An intentional, documented difference from upstream (`docs/DIVERGENCES.md`). |
| **ADR** | Architecture Decision Record in `docs/adr/`. |

---

## Appendix D — Command cheat sheet

```powershell
# --- environment -------------------------------------------------------------
cd 'd:\projects\r_abic'
rustc --version; cargo --version; cargo clippy --version   # 1.98.0 / 1.98.0 / 0.1.98 (2026-10-31)

# --- the gate (§4.8) --------------------------------------------------------
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
$env:RUSTDOCFLAGS = '-D warnings'; cargo doc --no-deps --all-features; Remove-Item Env:\RUSTDOCFLAGS
cargo bench --no-run
cargo tree -e normal --depth 1

# --- apply formatting, run one module's tests, list tests -------------------
cargo fmt --all
cargo test normalize::            # unit tests whose path matches
cargo test --test tokenize        # one integration test file
cargo test -- --nocapture         # show println output while debugging

# --- docs & benches ---------------------------------------------------------
cargo doc --no-deps --open
cargo bench -- --save-baseline before
cargo bench -- --baseline before

# --- repo hygiene -----------------------------------------------------------
git --no-pager status
git --no-pager diff --stat
git --no-pager log --oneline -n 15

# --- release / versioning (§8) ----------------------------------------------
git log -1 --format=%s                    # G9: the subject release-plz will parse
git --no-pager tag --sort=-v:refname | Select-Object -First 5   # latest tags
git log --oneline "$(git describe --tags --abbrev=0)..HEAD"     # since last tag (needs a tag)
cargo metadata --no-deps --format-version 1 > $null             # G9: manifest parses
cargo publish --dry-run                   # the only publishing command an agent may run (§8.5)
release-plz update --dry-run              # if installed: next version + changelog diff

# --- optional tooling -------------------------------------------------------
cargo add --dev criterion          # only with user approval (§2.2)
cargo llvm-cov --all-features --summary-only
cargo deny check
cargo miri test
```

Reminder: `cargo test --doc` is not implied by a plain `cargo test --lib`, and clippy without
`--all-targets` does not lint tests/benches/examples. Always run the exact gate commands above.

## Appendix E — Changelog of this document

| Version | Date | Change |
|---|---|---|
| **1.4.1** | 2026-11-03 | PATCH: corrected upstream facts that Phase 0 had left marked `(verify)`, after a one-time local `tools/oracle/` checkout of the GPL upstream. `normalize_hamza` really is `normalize_hamza(word, method="uniform")` with the methods `uniform`/`tasheel` — not `HamzaMethod { Remove, Tasheel }` — so §3.3's enum example, the Appendix B checklist item and the **tasheel** glossary entry were corrected; the header status line moved to Phase 1. No rule, gate or test requirement changed. |
| **1.4.0** | 2026-11-01 | MINOR: **§5.7 "Git operations require a human green light"** — no history- or remote-changing git command runs until the user has approved a proposal containing the exact commands, paths, verbatim commit message, expected state and destructive effects; read-only commands are allowlisted, `fetch`/`add` are announce-only, and "ok"/"thanks" are explicitly not approval. Added red line 20, a Definition of Done item, one bullet in each vendor binding and a note in `CONTRIBUTING.md`. |
| **1.3.0** | 2026-11-01 | MINOR: the canonical ruleset was renamed `CLAUDE.md` → **`AGENTS.md`** (no vendor-named copy ships; vendor files are bindings), and §0.5 ("The ruleset file and its bindings") records the naming policy as a rule. All 20 files referencing the old name were updated, including Rustdoc in `src/**` and the workflow comments. Also fixed the header block, which had lagged at v1.1.0. |
| **1.2.0** | 2026-11-01 | MINOR: added `CONTRIBUTING.md`, the human on-ramp (setup, branch/PR model, commit grammar with the release-impact table, what may and may not be touched, tests/docs expectations, local gate, release flow, troubleshooting, licensing of contributions). New rule: §5.2 requires `CONTRIBUTING.md` to be updated in the same commit as any branch/commit/PR/gate rule change. Also recorded in the §3.1 layout and the §8.3 file table; `README.md` points newcomers at it. |
| **1.1.0** | 2026-11-01 | MINOR: automated semantic versioning & releases. Adds **§8** (single source of truth, commit→version table, tooling/files, secrets/permissions/branch protection, crates.io publishing policy, release flow, agent red lines, local dry-runs); rewrites **§5.2** with the CI-enforced commit grammar and release-impact rules; adds gate **G9** (§4.8), red lines **16–19** (§5.4), a release-impact item in §5.3/§5.6, **§7.1** (`conventional-commits.yml`) and **§7.2** (`release.yml`); updates §6 Phase 0/4 and Appendix D. Repository: adds `release-plz.toml`, `.github/workflows/release.yml`, `.github/workflows/conventional-commits.yml`. |
| **1.0.1** | 2026-10-31 | PATCH: clarified §3.1 that `unicode::classify` holds the public `const fn` predicates (as §3.2 already stated) and that `unicode/mod.rs` only wires submodules; `.cursorrules` and `.github/copilot-instructions.md` re-synced. No rule changed. |
| **1.0.0** | 2026-10-31 | Initial document. Establishes principles `P1`–`P12`, licensing/provenance policy (GPL caution), dependency & MSRV policy (edition 2024, zero runtime deps), target layout and the PyArabic → `r_abic` module map, public-API/error/Unicode/allocation conventions, Rustdoc standard, mandatory TDD loop with the `E1`–`E20`/`N1`–`N3` edge-case matrix and property-test list, benchmark & doctest rules, quality gate `G1`–`G8` (with PowerShell and POSIX command sets), plan-first protocol, commit rules, Definition of Done, red lines, divergence/ADR process, phase roadmap, CI workflow spec, and appendices A–E. Upstream facts recorded from a survey of `linuxscout/pyarabic` `master` on the same date (modules `araby`, `araby_const`, `normalize`, `kalima`, `unshape`, `trans`, `number`, `soundex`, `named*`, `propernouns`, `stack`, `arabrepr`); toolchain recorded as rustc/cargo 1.98.0, clippy 0.1.98, rustfmt 1.9.0-stable. |

**How to maintain this file.** Any change to rules requires: (1) a new row here with a bumped version,
(2) the mirror files `.cursorrules` and `.github/copilot-instructions.md` updated in the same commit,
(3) an ADR when the change relaxes or overrides a principle (`P*`), and (4) a `chore(docs)` Conventional
Commit. Never delete a rule silently: mark it superseded and point at the replacement.

**Version bumping:** `MAJOR` = a principle changes meaning or is removed; `MINOR` = a new rule/section,
a new gate, or a new mandatory test category; `PATCH` = wording, typos, clarified examples.

---

*End of `AGENTS.md` v1.4.1 — if you are an agent reading this at the start of a task, re-read §0.2, the
plan template in §5.1, the edge-case matrix in §4.3 and the gate in §4.8 before you write a single line
of code or a single test.*

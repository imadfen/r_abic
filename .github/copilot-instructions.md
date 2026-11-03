# Copilot / AI assistant instructions for `r_abic`

> **Canonical ruleset:** [`AGENTS.md`](../AGENTS.md) (v1.4.1, agent-neutral) at the repository root. Read it
> fully before suggesting or writing code. This file is a Copilot binding that repeats only the rules that
> must never be violated; if there is any conflict, `AGENTS.md` is authoritative.

## What this repository is
A high-performance, memory-efficient, **dependency-free** Rust library for Arabic text processing
(normalization, tashkeel/ḥarakāt handling, tokenization, unshaping, transliteration, Arabic numbers),
built as a behavioural-parity alternative to the Python library **PyArabic**. Edition 2024.

## Always
* Write **tests first**; run and show the failing output, then implement, then run the whole quality gate.
* Return `Cow<'_, str>` when the input may be unchanged; provide `*_into(&mut String)` and iterator-based
  APIs for hot paths; prefer zero-copy slices over owned `String`s.
* Iterate with `chars()`/`char_indices()`; every byte range you produce must be a valid char boundary.
* Give every `pub` item Rustdoc with a runnable `# Examples` doctest (`cargo test --doc` must pass).
* Follow snake_case names that mirror upstream PyArabic symbols (`strip_tashkeel`, `strip_lastharaka`,
  `sentence_tokenize`, `normalize_digits`, `encode_tashkeel`) and note them in `docs/PARITY.md`.
* Update `docs/PARITY.md`, `docs/DIVERGENCES.md` and `CHANGELOG.md` in the same change when behaviour,
  parity status or the public API changes.
* Cite rule IDs (`P3`, `§3.5`, `E11`) when explaining decisions or reporting violations.

## Never
* Never add a runtime dependency (`[dependencies]` stays empty) or use `unsafe`.
* Never call `unwrap()`/`expect()`/`panic!()` in library code, and never let user input cause a panic.
* Never slice a `&str` at a hand-computed byte offset (multi-byte Arabic letters, combining ḥarakāt and
  ligatures make it a bug waiting to happen).
* Never copy code, docstrings, tables or tests from the GPL upstream project into this MIT/Apache-2.0 crate.
* Never weaken, skip or delete a test to make the build pass; never report success without running the gate.
* Never run a git operation (commit, branch, tag, rebase, reset, push) without the user approving the exact
  commands and the verbatim commit message — `AGENTS.md` §5.7.

## Quality gate (must pass, with output pasted into the report)
```
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo bench --no-run
cargo tree -e normal --depth 1        # must list only r_abic
```

## Commits & releases (CI-enforced — `AGENTS.md` §5.2, §8)

* Commit subjects and PR titles must follow Conventional Commits
  (`feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`), with an
  optional `(scope)`, an optional `!` for breaking changes, and a subject of at most 72 characters.
* release-plz derives the next SemVer version from those messages: `feat` → MINOR, `fix` → PATCH,
  `!`/`BREAKING CHANGE` → MAJOR (MINOR while `0.x`); everything else produces no release on its own.
* Never edit `[package] version`, never hand-write a released `CHANGELOG.md` section, never create, move
  or delete git tags, never publish to crates.io. `release-plz.toml` and
  `.github/workflows/release.yml` own versioning, tagging, releases and the changelog.
* Report the release impact (`none`/`patch`/`minor`/`major`) of a change, and only run
  `cargo publish --dry-run` when publishing is discussed.

## Domain gotchas enforced by tests
* `U+0640` TATWEEL is a joiner: not a letter, not a ḥaraka, never a word boundary.
* `strip_tashkeel` removes shadda; `strip_harakat` keeps it; `strip_lastharaka` removes only the last
  ḥaraka; `strip_diacritics` also removes `U+0670`, `U+06E5`, `U+06E6`.
* Lam-alef ligatures `U+FEF5–U+FEFC` expand 1 char → 2 chars, so byte offsets do not survive.
* Digit-set conversion keeps the char count but changes the byte length (2 ↔ 1 bytes per digit).
* No NFC/NFD/NFKC: combining marks are never reordered — document this in `# Unicode`.
* Upstream `unshaping_*` also reverses text; ours maps only (reversal is a separate, explicit function).

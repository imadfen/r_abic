# R_abic

Arabic text processing library for Rust, inspired by
[PyArabic](https://github.com/linuxscout/pyarabic).

> Status: **Phase 1** (in progress) — the `unicode` module (code points, class tables, classification
> predicates) and the `normalize` strip family (`strip_tashkeel`, `strip_harakat`, `strip_lastharaka`,
> `strip_tatweel`, `strip_shadda`, `strip_small`) are implemented. The remaining transforms,
> tokenization, unshaping, transliteration and numbers follow; see [`AGENTS.md`](AGENTS.md) §6.

## Quick start

```rust
use r_abic::normalize::{strip_harakat, strip_lastharaka, strip_tashkeel};
use r_abic::unicode::{classify, constants, tables};

// Code points are exposed under their PyArabic names.
assert_eq!(constants::ALEF, '\u{0627}');

// Classes are `const` slices; predicates are `const fn`s.
assert!(classify::is_haraka(constants::FATHA));
assert!(!classify::is_haraka(constants::SHADDA)); // shadda is tashkeel, not a haraka
assert!(classify::is_arabicword("العربية"));
assert_eq!(tables::HARAKAT.len(), 7);
assert_eq!(tables::alphabetic_order(constants::ALEF), Some(1));

// Text transforms borrow their input when nothing has to change (zero-copy fast path).
assert_eq!(strip_tashkeel("الْعَرَبِيَّةُ"), "العربية");
assert!(matches!(
    strip_tashkeel("العربية"),
    std::borrow::Cow::Borrowed(_)
));

// `strip_harakat` is the same, except that the shadda survives.
assert_eq!(strip_harakat("الْعَرَبِيَّةُ"), "العربيّة");

// `strip_lastharaka` removes every tanwīn plus only the last ḥaraka:
assert_eq!(strip_lastharaka("الْعَرَبِيَّةُ"), "الْعَرَبِيَّة");
```

## Build

```sh
cargo build
cargo test
cargo bench                                # criterion benchmarks
```

## Development

New here? Start with **[`CONTRIBUTING.md`](CONTRIBUTING.md)** — setup, the branch/commit/PR rules, and the
gate to run before pushing.

Project rules live in **[`AGENTS.md`](AGENTS.md)** — the canonical, agent-neutral ruleset — with condensed
bindings for Cursor ([`.cursorrules`](.cursorrules)) and Copilot
([`.github/copilot-instructions.md`](.github/copilot-instructions.md)).
Read `AGENTS.md` before making changes: it fixes the architecture, Unicode/allocation conventions, the
mandatory TDD workflow, the edge-case matrix and the release phases.

Before opening a change, run the full quality gate:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo bench --no-run                       # once benches exist
```

Behavioural parity target: [PyArabic](https://github.com/linuxscout/pyarabic) — its API is mapped
module-by-module in `AGENTS.md` §3.2 / Appendix B, and intentional differences are recorded in
`docs/DIVERGENCES.md` (created in Phase 0, see `AGENTS.md` §6).

## Releases

Releases are fully automated from [Conventional Commits](https://www.conventionalcommits.org/):
[release-plz](https://release-plz.dev) computes the next SemVer version from the commits since the last
tag, bumps `Cargo.toml` + `Cargo.lock`, regenerates the changelog, and — once the release PR is merged —
creates the git tag and the GitHub Release (`.github/workflows/release.yml`, `release-plz.toml`; policy
in [`AGENTS.md`](AGENTS.md) §8).

| Commit / PR title | Version bump |
|---|---|
| `feat: …` | **MINOR** |
| `fix: …` | **PATCH** |
| `feat!: …`, `fix!: …`, `BREAKING CHANGE:` footer | **MAJOR** (a **MINOR** bump while pre-1.0) |
| `docs:` / `style:` / `refactor:` / `perf:` / `test:` / `build:` / `ci:` / `chore:` | none on its own |

`0.x` releases are unstable by definition: a breaking change bumps the minor version until `1.0.0`. The
crate is **not** published to crates.io yet (`publish = false` in `release-plz.toml`).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this
crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

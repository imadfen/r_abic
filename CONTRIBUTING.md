# Contributing to `r_abic`

Thanks for helping build this. This document is the **human on-ramp**: setup, branches, commits, pull
requests, and what happens after your PR is merged.

The engineering rules live in [`AGENTS.md`](AGENTS.md) — that file is canonical for agents **and**
humans. If anything here contradicts it, `AGENTS.md` wins and this file must be fixed in the same
change. It is deliberately not named after any tool, so that every agent and every human reads the same
file; the vendor files listed in §6 are thin bindings that defer to it.

---

## 1. Know these two project rules before you write code

`r_abic` is a high-performance, dependency-free Rust library for Arabic text processing, built as an
independent alternative to [PyArabic](https://github.com/linuxscout/pyarabic). Two constraints shape
everything:

* **Zero runtime dependencies.** `[dependencies]` in `Cargo.toml` stays empty (principle `P1`). Adding
  one requires the owner's approval plus an ADR in `docs/adr/`. Dev-dependencies are allowed but must be
  justified.
* **Provenance.** PyArabic is GPL-licensed; `r_abic` is MIT/Apache-2.0. **Never paste PyArabic source
  code, docstrings, data tables or tests into this repository** — not even "just as a reference".
  Re-derive behaviour from the documented API and the Unicode Standard, and record intentional
  differences in `docs/DIVERGENCES.md` (`AGENTS.md` §2.1).

Two more that reviewers will hold you to:

* **No `unsafe`** anywhere, and no `unwrap`/`expect`/`panic!` in library code outside tests.
* **Never** slice a `&str` at a hand-computed byte offset: Arabic letters, ḥarakāt and ligatures are
  multi-byte, so every offset must come from `char_indices()`/`find` (`P5`).

## 2. Setup

Prerequisites: Rust ≥ **1.85** (edition 2024; development happens on the stable toolchain, currently
1.98) with `rustfmt` and `clippy`:

```sh
rustup component add rustfmt clippy
```

```sh
git clone git@github.com:imadfen/r_abic.git    # or https://github.com/imadfen/r_abic.git
cd r_abic
cargo build
cargo test --all-features
```

Line endings in the repository are **LF** (`.gitattributes`). On Windows your working copy may show
CRLF — that is expected; do not "fix" it by committing CRLF files, and do not add `core.autocrlf`
exceptions to the repo.

## 3. The branch → PR → squash-merge model

`main` is protected: no direct pushes (not even for the owner), no force pushes, no deletions, **linear
history required** (so only squash merges), and the CI checks must pass.

```text
main ─► short-lived branch ─► pull request (conventional title) ─► green CI ─► squash-merge ─► delete branch
```

Branch naming (short-lived: rebase on `main` rather than merging `main` into your branch):

| Prefix | Use |
|---|---|
| `feat/<area>` | new functionality, e.g. `feat/normalize-strip-tatweel` |
| `fix/<area>` | bug fix |
| `docs/<area>` | documentation only |
| `test/<area>` | tests only |
| `refactor/<area>` | behaviour-preserving restructure |
| `perf/<area>` | performance work (must ship with a benchmark) |
| `chore/<area>`, `ci/<area>` | tooling, dependencies, CI |

Typical loop:

```sh
git switch main && git pull            # or: git fetch origin && git reset --hard origin/main
git switch -c feat/normalize-strip-tatweel
# … write tests first, then code (see §6) …
git fetch origin && git rebase origin/main
git push -u origin feat/normalize-strip-tatweel
```

> If you are not a collaborator, **fork** the repository and open the PR from your fork. CI still runs;
> you will not have access to the repository secrets, and you never need them.

## 4. Commit messages (enforced by CI)

Every commit subject **and** every PR title must match this pattern
(`.github/workflows/conventional-commits.yml`):

```text
^(feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)(\(scope\))?!?: <subject ≤ 72 chars>
```

Rules of thumb:

* imperative mood, lowercase type, no trailing period: `add`, not `added`/`adds`;
* `scope` is the area you touched: `normalize`, `tokenize`, `unicode`, `trans`, `numbers`, `docs`, `ci`;
* the body explains **why** (the diff already shows *what*); wrap at ~72 columns;
* footers: `BREAKING CHANGE: <what breaks and how to migrate>`, `Refs: #12`;
* one purpose per commit — implementation + its tests + its docs travel together; never mix a refactor
  with a behaviour change.

| Good | Bad |
|---|---|
| `feat(normalize): add strip_tatweel with Cow fast path` | `Added strip_tatweel` |
| `fix(tokenize): keep ZWJ attached to the preceding token` | `fix add stuff` |
| `docs(unicode): document parity with pyarabic.araby.is_arabicrange` | `feature: docs` |
| `test(normalize): cover E1–E6 and idempotence` | `feat: ` (empty subject) |
| `refactor(diacritics): extract the char-class matcher` | `Update README` |

### Why the types matter: they drive the release

`release-plz` reads these messages to compute the next version — **nobody types a version number by
hand**:

| Commit / PR title | Version bump | Changelog section |
|---|---|---|
| `feat: …` | **MINOR** (`0.1.0 → 0.2.0`) | Added |
| `fix: …` | **PATCH** (`0.1.0 → 0.1.1`) | Fixed |
| `feat!: …` / `fix(scope)!: …` / any `BREAKING CHANGE:` footer | **MAJOR** (a **MINOR** bump while the crate is `0.x`) | the type's section, always kept |
| `perf: …`, `refactor: …`, `revert: …`, `docs: …` | no release on their own | Performance / Changed / Reverted / Documentation |
| `test:`, `style:`, `ci:`, `chore:`, `build:` | no release | not listed |

**The PR title becomes the squashed commit subject on `main`**, so a mislabelled title silently ships
the wrong version. Write the title as the commit you want in the history.

A **user-visible breaking change** needs all three: the `!` marker, a `BREAKING CHANGE:` footer, and an
ADR in `docs/adr/` (`AGENTS.md` §3.3, §8.2).

## 5. Opening a pull request

Before you push:

- [ ] the branch is rebased on the latest `main` and contains **only** your intended changes
      (`git --no-pager diff --stat origin/main...HEAD`);
- [ ] the local gate is green (§8);
- [ ] tests were written first and cover the applicable edge-case IDs (§7);
- [ ] public items have Rustdoc with a runnable example; `docs/PARITY.md` and `docs/DIVERGENCES.md` are
      updated if behaviour or parity changed;
- [ ] the PR title is a valid Conventional Commit (§4).

The PR body should state: **what** changed, **why**, the **edge cases** you covered, and the **release
impact** (`none` / `patch` / `minor` / `major`) implied by your commit types. A reviewer must be able to
verify your claims from the diff and the CI panel alone.

Keep PRs small — one logical change. A 2 000-line PR mixing three features will sit unreviewed; three
focused PRs get merged.

> **If you work through a coding agent:** `AGENTS.md` §5.7 requires it to get your explicit green light —
> the exact commands, the verbatim commit message and the expected resulting state — before it creates a
> commit, moves a branch or pushes anything. Read the proposal before approving it; that gate exists
> because an unreviewed commit message is what silently ships the wrong version (§4).

## 6. What you may change, and what you must not touch

| Path | Rule |
|---|---|
| `src/**`, `tests/**`, `benches/**`, `examples/**` | yours to change, following `AGENTS.md` §3 |
| `CONTRIBUTING.md`, `README.md`, `docs/PARITY.md`, `docs/DIVERGENCES.md` | yours to change; keep them true |
| `AGENTS.md`, `.cursorrules`, `.github/copilot-instructions.md` | propose changes, but the owner reviews them closely — they govern every future contribution |
| `docs/adr/**` | you may **add** ADRs; never edit an accepted one (add a superseding ADR instead) |
| `CHANGELOG.md` | release-plz owns the released sections; only `[Unreleased]` is hand-editable |
| `Cargo.toml` | never the `[package] version` field; `[dependencies]` stays empty; dev-deps need the owner + an ADR |
| `Cargo.lock` | only if a dev-dependency actually changed (commit it together with that change) |
| `release-plz.toml`, `.github/workflows/**` | owner-only changes, unless you are fixing a bug you found — then say so explicitly in the PR body |

## 7. Tests and documentation expectations

Work **test-first** (`AGENTS.md` §4.1): write the failing test, show it failing, then implement. Reviewers
look at the tests as much as the code.

* **Edge cases are named, not improvised.** `AGENTS.md` §4.3 lists the mandatory matrix; reference the ID
  in the test name or a comment. For text transforms that usually means at minimum:

  | ID | Case |
  |---|---|
  | `E1` | empty string (`Cow::Borrowed("")`, no panic) |
  | `E2` | whitespace only, including NBSP / `\r\n` |
  | `E3`/`E5` | marks-only input; shadda+ḥaraka in both orders (never reordered) |
  | `E4` | a combining mark at the very start |
  | `E6` | tatweel/kashida `U+0640` (a joiner, never a letter) |
  | `E7`/`E8`/`E9` | mixed Arabic+Latin, the three digit sets, Arabic vs ASCII punctuation |
  | `E10`–`E13` | astral/emoji, ZWJ/ZWNJ, RTL marks, BOM |
  | `E14`–`E17` | presentation forms, lam-alef ligatures (1 char → 2), marks after ligatures, CRLF vs LF |
  | `E18`–`E20` | 1 MiB input, stacked marks, NFC/NFD input handled identically |

* **Zero-copy assertions** for `Cow`-returning functions:
  `assert!(matches!(strip_x(clean), Cow::Borrowed(_)))`.
* **Byte-boundary safety**: every returned range must satisfy `str::is_char_boundary`.
* **Doctests are mandatory** for every public item (runnable, asserting); `cargo test --doc` is part of
  the gate.
* **Benchmarks** for hot paths (Criterion, `benches/`): compare with
  `cargo bench -- --save-baseline before` / `--baseline before` and mention the delta in the PR — a >10%
  regression must be justified or reverted.
* **Tracking docs**: update `docs/PARITY.md` (status of the PyArabic symbol you implemented) and
  `docs/DIVERGENCES.md` (any intentional difference). Fixtures must be your own — see §1 on provenance.

## 8. Run the gate locally before you push

```powershell
# Windows / PowerShell
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo test --doc --all-features
$env:RUSTDOCFLAGS = '-D warnings'; cargo doc --no-deps --all-features; Remove-Item Env:\RUSTDOCFLAGS
cargo bench --no-run
cargo tree -e normal --depth 1        # must list only r_abic (zero runtime deps)
git log -1 --format=%s                # must be a Conventional Commit
```

```bash
# Linux / macOS
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features && cargo test --doc --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo bench --no-run
cargo tree -e normal --depth 1
```

CI runs the same commands with `RUSTFLAGS: -D warnings`, so any `rustc` warning fails the build. The
checks you will see on a PR:

| Check | What it proves |
|---|---|
| `fmt + clippy + docs` | formatting, lints (pedantic), rustdoc builds with `-D warnings` |
| `dependency audit` | the runtime dependency list is still empty (`P1`) |
| `test (ubuntu-latest, stable)` / `(windows-latest, stable)` / `(macos-latest, stable)` / `(beta)` / `(1.85)` | tests + doctests + bench compilation on every platform and the MSRV |
| `PR title` | the PR title is a Conventional Commit |
| `Commit subjects` | every commit in the PR is a Conventional Commit |

If a check never appears and GitHub says *"Expected — waiting for status to be reported"*, the workflow
did not run for that event: tell the owner instead of bypassing it.

## 9. Releases are automatic — don't help them

```text
your PR merged (squash) ─► release-plz opens/updates "chore: release vX.Y.Z" PR
                          ─► owner reviews + merges it
                          ─► git tag vX.Y.Z + GitHub Release are created automatically
```

* A release PR exists whenever there is a releasable change (`feat`, `fix`, breaking). `chore`/`docs`/`ci`
  changes do not create one.
* **Never** edit `[package] version`, an already-released `CHANGELOG.md` section, or a git tag; **never**
  publish to crates.io; **never** merge the release PR (that is the owner's call). Publishing is disabled
  on purpose (`publish = false` in `release-plz.toml`).
* Never rewrite `main`'s history once a tag exists.

## 10. When something goes wrong

| Symptom | Fix |
|---|---|
| `remote: error: GH006: Protected branch update failed` | You are pushing to `main`. Push a branch and open a PR (§3). |
| `! [rejected] … non-fast-forward` | `git fetch origin && git rebase origin/main`, then push your branch again (never `--force` on `main`). |
| CI fails on formatting | `cargo fmt --all`, commit as `style: apply rustfmt`. |
| CI fails on clippy | Run the exact CI command locally; fix the lint. Do not add crate-wide `#[allow]`s — a narrow, commented `#[allow]` is reviewed case by case. |
| CI fails on a commit subject | Before the PR is merged: `git rebase -i origin/main` and reword, then `git push --force-with-lease`. After it is merged: leave history alone. |
| A file shows as completely modified with no real change | Line endings. Confirm with `git diff --ignore-cr-at-eol`; `.gitattributes` is authoritative. |
| `cargo test` passes but CI fails on doctests | Run `cargo test --doc --all-features`. |
| Your merged PR broke something | Open a `revert: <subject>` PR (or a `fix:` PR). Never force-push. |

## 11. Questions, bugs, security, licence

* **Questions / ideas** — open an issue and say what you tried; a minimal reproducible snippet of Arabic
  text (in a fenced code block) plus `rustc --version` saves a round trip.
* **Bugs** — include the input, the actual output, the expected output, and the edge-case ID if you can
  map it.
* **Security** — use GitHub's *Report a vulnerability* (private advisory) rather than a public issue.
* **Licence of contributions** — the crate is `MIT OR Apache-2.0`; by submitting a PR you agree your
  contribution is dual-licensed under those terms, as stated in [`README.md`](README.md#license).
* Be straightforward and technical in reviews; critique the change, not the person.

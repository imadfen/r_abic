# Intentional divergences from PyArabic

Every entry documents a behaviour that intentionally differs from
[PyArabic](https://github.com/linuxscout/pyarabic). The format is defined in `AGENTS.md` §5.5; each
entry is pinned by at least one test and (where it matters) by an ADR.

Statuses: `intentional, permanent` · `intentional, temporary` · `needs maintainer confirmation`.

---

## D-001 — class groups are `&[char]`, not pre-joined strings

* **Upstream:** `araby_const` exposes both tuples of characters (`HARAKAT`, `TASHKEEL`, …) and
  pre-joined strings (`HAMZAT_STRING`, `HARAKAT_STRING`, `TASHKEEL_STRING`), which are then used to
  build regular expressions.
* **r_abic:** only the character groups exist (`unicode::tables`), plus `const fn` predicates
  (`unicode::classify`). No pre-joined string constants and no regular expressions.
* **Rationale:** string matching is O(n·m) and regexes are an unnecessary dependency (`P1`); the
  predicates are single range comparisons and are usable in `const` contexts.
* **Pinned by:** `src/unicode/classify.rs` tests (`predicates_agree_with_the_class_tables`,
  `class_implications_hold_across_latin_and_arabic_ranges`).
* **Status:** intentional, permanent.

## D-002 — `is_arabicword` uses a strict definition

* **Upstream:** `araby.is_arabicword(word)` accepts `U+0600–U+0652` plus the lam-alef ligature forms and
  then applies positional heuristics (the first char must be neither a ḥaraka nor `WAW_HAMZA`/
  `YEH_HAMZA`; `ALEF_MAKSURA` and `TEH_MARBUTA` must be last; no doubled shadda), so digits and
  punctuation inside that range pass. Verified against a local `tools/oracle/` checkout (2026-11-03).
* **r_abic:** `unicode::classify::is_arabicword` returns `true` only for a **non-empty** string made
  exclusively of base letters, `TASHKEEL` marks and `TATWEEL`. Spaces, digits, punctuation,
  presentation forms and formatting code points all yield `false`.
* **Rationale:** the strict reading is the only one consistent with the rest of the API (and with the
  tokenizer of Phase 1, which must be able to rely on it).
* **Pinned by:** `is_arabicword_requires_letters_marks_or_tatweel_only`.
* **Status:** intentional, permanent.

## D-004 — legacy camelCase aliases are not ported

* **Upstream:** `araby` keeps Python-2-era aliases (`hasShadda`, `isVocalized`, `isArabicrange`,
  `isArabicword`, `isArabicstring`, `stripLastHaraka`, …) next to the snake_case API.
* **r_abic:** only the snake_case names exist. Where an upstream snake_case spelling is unusual it is
  kept verbatim (`strip_lastharaka`, `delimite_language`) and documented with `#[doc(alias = "…")]`.
* **Rationale:** Rust naming conventions (`P2`, `AGENTS.md` §3.7); two spellings for one function is
  an API-consistency cost with no benefit for a new crate.
* **Pinned by:** absence — the symbols simply do not exist; `docs/PARITY.md` marks them
  `out-of-scope`.
* **Status:** intentional, permanent.

## D-005 — `is_ligature` also accepts the final forms

* **Upstream:** `araby_const.LIGUATURES` lists only the four **isolated** lam-alef forms (`U+FEF5`,
  `U+FEF7`, `U+FEF9`, `U+FEFB`), so an upstream membership test misses the final forms (`U+FEF6`,
  `U+FEF8`, `U+FEFA`, `U+FEFC`) that real-world text routinely contains.
* **r_abic:** `unicode::classify::is_ligature` matches the whole `U+FEF5..=U+FEFC` range (both forms of
  each of the four pairs). `unicode::tables::LIGUATURES` still mirrors upstream's four-element list, so
  parity checks stay greppable.
* **Rationale:** a de-shaping pass (`unshape`, Phase 2) that skipped final forms would leave ligatures
  in the output — a silent data bug. The predicate describes the phenomenon, while the table records
  the upstream list.
* **Pinned by:** `ligature_predicate_covers_all_eight_lam_alef_forms`.
* **Status:** intentional, permanent.

## D-006 — `is_hamza` matches the bare hamza only

- **Upstream:** `araby.is_hamza(archar)` is `archar in HAMZAT`, so it also returns `true` for the
  carriers (`WAW_HAMZA`, `YEH_HAMZA`, `ALEF_HAMZA_ABOVE`, `ALEF_HAMZA_BELOW`) and for the combining
  `HAMZA_ABOVE`/`HAMZA_BELOW`.
- **r_abic:** `unicode::classify::is_hamza(c)` is `c == HAMZA` (`U+0621`). Carriers are letters and are
  recognised by `is_arabic_letter`; the combining forms have the dedicated additions
  `is_hamza_above`/`is_hamza_below`.
- **Rationale:** "is this character the hamza letter" and "is this character one of the seven
  hamza-bearing forms" are different questions. Collapsing them (as upstream does) mixes a letter with
  its carriers and with combining marks, which is unusable for a classifier that feeds the tokenizer.
- **Pinned by:** `is_hamza_and_is_arabicrange_are_stricter_than_upstream`.
- **Status:** intentional, permanent.

## D-007 — `is_arabicrange` covers only the main Arabic block

- **Upstream:** `araby.is_arabicrange(text)` rejects a character unless it lies in `U+0600–U+06FF`,
  `U+0750–U+077F` (Arabic Supplement), `U+FB50–U+FDFF` (Presentation Forms-A) or `U+FE70–U+FEFF`
  (Presentation Forms-B); empty input is vacuously `true`.
- **r_abic:** `unicode::classify::is_arabicrange` accepts only `U+0600–U+06FF`. The other ranges are
  classified by purpose instead (`is_ligature` for the lam-alef pairs, `unshape` for presentation
  forms) and are documented as pass-through for the Phase 1 transforms.
- **Rationale:** one predicate cannot serve both "is this text plain Arabic letters and marks" and
  "does this text contain presentation forms"; the tokenizer needs the former, the de-shaper the
  latter.
- **Pinned by:** `is_hamza_and_is_arabicrange_are_stricter_than_upstream`.
- **Note:** alignment candidate, not yet settled — mirroring upstream's four ranges would make
  `is_arabicrange` return `true` for text that `unshape` is meant to rewrite. Changing the predicate
  needs an ADR plus a `!` commit, so it stays recorded here.
- **Status:** intentional, temporary.
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

* **Upstream:** `araby.is_arabicword(word)` documents "no spaces, digits and punctuation" but then
  returns "True if all characters are in the Arabic block", which admits digits and punctuation.
* **r_abic:** `unicode::classify::is_arabicword` returns `true` only for a **non-empty** string made
  exclusively of base letters, `TASHKEEL` marks and `TATWEEL`. Spaces, digits, punctuation,
  presentation forms and formatting code points all yield `false`.
* **Rationale:** the strict reading is the only one consistent with the rest of the API (and with the
  tokenizer of Phase 1, which must be able to rely on it).
* **Pinned by:** `is_arabicword_requires_letters_marks_or_tatweel_only`.
* **Status:** intentional, permanent.

## D-003 — `is_vocalized` counts a lone shadda as vocalization

* **Upstream:** `araby.isVocalized(word)` / `is_vocalized(word)` — the docstring says "checks if the
  word is vocalized" without specifying whether shadda alone qualifies.
* **r_abic:** `unicode::classify::is_vocalized` returns `true` when the word contains any `TASHKEEL`
  mark, i.e. a ḥaraka **or** a shadda. `has_harakat` is provided for the stricter question.
* **Rationale:** a shadda is a diacritic; treating it as "not vocalized" would make `الشّمس` and
  `الشمس` indistinguishable.
* **Pinned by:** `string_helpers_detect_marks`.
* **Status:** needs maintainer confirmation (verify against upstream once a local checkout is
  available, then promote to permanent or fix).

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
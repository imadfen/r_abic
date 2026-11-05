# PyArabic parity tracker

Living tracker for behavioural parity with PyArabic
(`linuxscout/pyarabic`, `master` as observed on 2026-10-31). Rules and formats: `AGENTS.md` §5.5.

**Statuses:** `todo` · `wip` · `done` · `divergent` · `out-of-scope`.
`TBD` in a note means "upstream semantics not verified yet" — such a symbol must never be marked
`done`.

**Last updated:** 2026-11-04 (Phase 1 continuing: the `normalize` strip family is complete; `normalize_ligature`/`normalize_lamalef` added).

---

## 1. `araby` / `araby_const` — code points and classes → `unicode`

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| single-letter constants `COMMA`…`YEH` | `unicode::constants::*` | done | values pinned by `code_point` tests |
| ḥaraka constants `FATHATAN`…`SUKUN` | `unicode::constants::*` | done | |
| `MADDA_ABOVE`, `HAMZA_ABOVE`, `HAMZA_BELOW` | `unicode::constants::*` | done | |
| digit constants `ZERO`…`NINE`, `*_W`, `*_P` | `unicode::constants::*` | done | three sets, contiguous (tested) |
| `PERCENT`, `DECIMAL`, `THOUSANDS`, `STAR`, `FULL_STOP`, `SEMICOLON`, `QUESTION` | `unicode::constants::*` | done | |
| `BYTE_ORDER_MARK` | `unicode::constants::BYTE_ORDER_MARK` | done | |
| `SMALL_ALEF`, `MINI_ALEF`, `SMALL_WAW`, `SMALL_YEH`, `ALEF_WASLA` | `unicode::constants::*` | done | `MINI_ALEF` is an alias |
| `LAM_ALEF`, `LAM_ALEF_HAMZA_ABOVE`, `LAM_ALEF_HAMZA_BELOW`, `LAM_ALEF_MADDA_ABOVE` | `unicode::constants::*` | done | |
| `SIMPLE_LAM_ALEF*` | `unicode::constants::*` | done | exposed as `&str` |
| `HARAKAT`, `TASHKEEL`, `SHORTHARAKAT`, `TANWIN`, `NOT_DEF_HARAKA` | `unicode::tables::*` | done | sizes + no-duplicates tested |
| `LIGUATURES`, `HAMZAT`, `ALEFAT`, `LETTERS`, `WEAK`, `YEHLIKE`, `WAWLIKE`, `TEHLIKE`, `SMALL`, `MOON`, `SUN` | `unicode::tables::*` | done | upstream order preserved |
| `NUMBERS_EAST`, `NUMBERS_WEST`, `NUMBERS_PERS` | `unicode::tables::*` | done | `[char; 10]` |
| `ALPHABETIC_ORDER` | `unicode::tables::ALPHABETIC_ORDER` + `alphabetic_order()` | done | sorted table + binary search |
| `HAMZAT_STRING`, `HARAKAT_STRING`, `TASHKEEL_STRING` | — | divergent | D-001: predicates replace pre-joined strings |
| `NAMES` (letter names) | — | todo | Phase 3, `unicode::names` |

### 1.1 Letter predicates (`araby.is_*`)

Everything below was re-verified on 2026-11-03 against a local `tools/oracle/` checkout of upstream
`master`. Upstream implements each predicate as a plain membership test over the class tuples
(`is_hamza` → `HAMZAT`, `is_alef` → `ALEFAT`, `is_yehlike` → `YEHLIKE`, `is_wawlike` → `WAWLIKE`,
`is_teh` → `TEHLIKE`, `is_small` → `SMALL`, `is_weak` → `WEAK`, `is_moon` → `MOON`, `is_sun` → `SUN`,
`is_tanwin` → `TANWIN`, `is_haraka` → `HARAKAT`, `is_shortharaka` → `SHORTHARAKAT`, `is_tashkeel` →
`TASHKEEL`), and `is_shadda`/`is_sukun`/`is_tatweel` are plain equality with their constant.

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `is_haraka` | `unicode::classify::is_haraka` | done | verified: the 7 `HARAKAT` marks, shadda excluded |
| `is_shortharaka` | `unicode::classify::is_short_haraka` | done | verified; upstream spelling kept as the doc alias |
| `is_tanwin` | `unicode::classify::is_tanwin` | done | verified: the three tanwīn marks |
| `is_tashkeel` | `unicode::classify::is_tashkeel` | done | verified: `HARAKAT` + shadda |
| `is_shadda`, `is_sukun`, `is_tatweel` | `unicode::classify::is_{shadda,sukun,tatweel}` | done | verified: equality with the constant, identical behaviour |
| `has_shadda` | `unicode::classify::has_shadda` | done | verified |
| `is_hamza` | `unicode::classify::is_hamza` | divergent | D-006: upstream accepts all 7 `HAMZAT` forms, i.e. the carriers too; ours tests `U+0621` only |
| `is_ligature` | `unicode::classify::is_ligature` | divergent | D-005: accepts all eight `U+FEF5..=U+FEFC` forms, not only the four isolated ones |
| `is_arabicrange` | `unicode::classify::is_arabicrange` | divergent | D-007: upstream accepts 4 ranges (`U+0600–U+06FF`, `U+0750–U+077F`, `U+FB50–U+FDFF`, `U+FE70–U+FEFF`); ours only the main block. Vacuous `true` for empty input, as upstream |
| `is_arabicword` | `unicode::classify::is_arabicword` | divergent | D-002: upstream accepts `U+0600–U+0652` ∪ the lam-alef forms plus positional heuristics (first char not a ḥaraka/`WAW_HAMZA`/`YEH_HAMZA`, alef maksura last, teh marbuta last, no doubled shadda); ours requires letters/marks/tatweel only |
| `is_vocalized` | `unicode::classify::is_vocalized` | done | **verified equivalent**: upstream's lone-shadda word does count (its `is_tashkeel` includes shadda), so the former D-003 divergence was withdrawn |
| `is_vocalizedtext` | `unicode::classify::has_harakat` | done | verified: upstream searches `HARAKAT` only — a lone shadda does **not** count |
| `is_arabicstring` | — | todo | verified: rejects any char outside `U+0600–U+0652` ∪ the 3 lam-alef forms ∪ whitespace ∪ Unicode decimal digits; vacuous `true` for empty input |
| `is_small` | `unicode::classify::is_small` | done | membership in `SMALL` (`U+0670`, `U+06E5`, `U+06E6`); pinned by a block-scan test against `tables::SMALL` + doctest |
| `is_alef`, `is_yehlike`, `is_wawlike`, `is_teh`, `is_weak`, `is_moon`, `is_sun` | — | todo | pure table membership; the `unicode::tables` groups themselves are `done` |
| `first_char`, `second_char`, `last_char`, `secondlast_char` | — | todo | trivial accessors; decide during Phase 1 whether to expose them |
| `order`, `name` | `unicode::tables::alphabetic_order`, `unicode::names` (Phase 3) | done / todo | `ALPHABETIC_ORDER` / `NAMES` lookups |
| `arabicrange()` (a string of the whole block) | — | out-of-scope | exists only to build regexes; predicates replace it (D-001) |
| `hasShadda`, `isVocalized`, `isArabicrange`, `isArabicword` (legacy camelCase) | — | out-of-scope | D-004 |

### 1.2 r_abic additions (no upstream equivalent)

| `r_abic` symbol | Status | Notes |
|---|---|---|
| `unicode::classify::is_vowel` | done | alias of `is_haraka`; **upstream has no `is_vowel`** |
| `unicode::classify::is_hamza_above`, `is_hamza_below` | done | **upstream has no such symbols**; upstream's `is_hamza` covers the combining hamzas |
| `unicode::classify::is_arabic_letter` | done | our name for exact `LETTERS` membership; **upstream has no `is_letter`/`is_arabicletter`** — callers test the `LETTERS` tuple themselves |
| `unicode::classify::has_harakat`, `has_tashkeel` | done | `has_harakat` is the exact match for upstream `is_vocalizedtext` |
| `unicode::classify::is_arabic_block_char`, `is_arabic_indic_digit`, `is_extended_arabic_indic_digit`, `is_arabic_digit` | done | needed by the tokenizer and `normalize_digits` |
| `unicode::classify::is_formatting` | done | ZWJ/ZWNJ/BOM/RTL marks (E11–E13) |
| `unicode::constants::{ZERO_WIDTH_JOINER, ZERO_WIDTH_NON_JOINER, LEFT_TO_RIGHT_MARK, RIGHT_TO_LEFT_MARK, ARABIC_LETTER_MARK}` | done | `r_abic` additions |
| `Error`, `Result` | done | Phase 0; `UnsupportedConversion` arrives with `trans` |

---

## 2. `normalize` / `araby` — text transforms → `normalize` (Phase 1)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `strip_tashkeel(text)` | `normalize::strip_tashkeel` (+ `strip_tashkeel_into`) | done | `Cow` fast path + `_into` buffer reuse; pinned by 10 unit + 4 property + 8 integration tests + 3 doctests; bench `benches/normalize.rs` |
| `strip_harakat(text)` | `normalize::strip_harakat` (+ `strip_harakat_into`) | done | verified: removes the 7 `HARAKAT` marks and **keeps shadda** — upstream's `is_vocalized` guard cannot change the result; pinned by 9 unit + 3 integration + 5 property tests and 2 doctests; bench `benches/normalize.rs` |
| `strip_lastharaka(text)` | `normalize::strip_lastharaka` (+ `strip_lastharaka_into`) | done | verified: removes every `TANWIN` mark anywhere plus **one** trailing `HARAKAT` (`[HARAKAT]$|[TANWIN]`), so it is deliberately **not idempotent**; the `$` anchor is mirrored exactly, including its match just before a single trailing `\n`; pinned by 6 unit + 3 integration + 1 property test and 2 doctests; bench `benches/normalize.rs` |
| `strip_tatweel(text)` | `normalize::strip_tatweel` (+ `strip_tatweel_into`) | done | verified: plain removal of `U+0640` (the `NOT_DEF_HARAKA` alias is the same code point); pinned by 7 unit + 3 integration + 1 property test and 2 doctests; bench `benches/normalize.rs` |
| `strip_shadda(text)` | `normalize::strip_shadda` (+ `strip_shadda_into`) | done | verified: plain removal of `U+0651` (upstream's `is_vocalized` guard cannot change the result); keeps every other mark, so `strip_shadda∘strip_harakat == strip_tashkeel`; pinned by 9 unit + 4 integration + 1 property test and 2 doctests; bench `benches/normalize.rs` |
| `strip_small(text)` | `normalize::strip_small` (+ `strip_small_into`) | done | verified: removes the `SMALL` class (`U+0670`, `U+06E5`, `U+06E6`); disjoint from tashkeel/tatweel, so it commutes with both; pinned by 8 unit + 3 integration + 1 property test and 2 doctests; bench `benches/normalize.rs` |
| `strip_diacritics(text)` | `normalize::strip_diacritics` (+ `strip_diacritics_into`) | done | decided: **mirror** the category rule — a hand-written `Mn` range table (`U+0610`–`U+061A`, `U+064B`–`U+065F`, `U+0670`, `U+06D6`–`U+06DC`, `U+06DF`–`U+06E4`, `U+06E7`–`U+06E8`, `U+06EA`–`U+06ED`) reproduces upstream with no dependency; supersets `strip_tashkeel`, keeps `U+06E5`/`U+06E6` (`Lm`) and out-of-block `Mn`, commutes with `strip_small`; pinned by 8 unit + 3 integration + 1 property test and 2 doctests; bench `benches/normalize.rs` |
| `reduce_tashkeel(text)` | `normalize::reduce_tashkeel` | todo | verified: six ordered rules — (1) drop every `FATHA`/`SUKUN` not preceded by `WAW`/`YEH`, (2) drop `DAMMA` before `WAW`, (3) drop `KASRA` before `YEH`, (4) drop `FATHA` before `ALEF`, (5) drop `FATHA` after a word-initial `WAW`/`YEH`, (6) drop `KASRA` after `ALEF_HAMZA_BELOW` |
| `normalize_teh(text)` | `normalize::normalize_teh` | todo | **new row**: `TEH_MARBUTA` → `HEH` (upstream keeps this apart from `normalize_spellerrors`) |
| `normalize_alef(text)` | `normalize::normalize_alef` | todo | **new row**: every `ALEFAT` form → `ALEF`, with the `SMALL_ALEF`↔`ALEF_MAKSURA` special cases removed first |
| `normalize_hamza(word, method="uniform")` | `normalize::normalize_hamza` | todo | verified against a local `araby.py` (`tools/oracle/`, dev-only): `HamzaMethod::{Uniform, Tasheel}`; `tasheel` folds each hamza onto its carrier (alef-madda/alef-hamza/combining hamzas → `ALEF`, `WAW_HAMZA` → `WAW`, `YEH_HAMZA` → `YEH`), `uniform` (the default) maps the whole `HAMZAT` class → `HAMZA` and expands `ALEF_MADDA` → `HAMZA`+`HAMZA` with a leading-madda special case; upstream is *word*-scoped (`word[1]`/`word[2]` indexing) and its docstring wrongly claims it also simplifies lam-alef. `normalize.py`'s no-argument twin folds the alef family to `ALEF` first, so the two upstream modules disagree — we keep one canonical location and pin the mirrored semantics with tests |
| `normalize_ligature(text)` | `normalize::normalize_ligature` (+ `normalize_ligature_into`) | divergent | expands all eight lam-alef forms (`U+FEF5`–`U+FEFC`, isolated + final) to `LAM` + alef, a **superset** of upstream's four isolated forms → D-008; pinned by 6 unit + 3 integration + 2 property tests and 2 doctests; bench `benches/normalize.rs` |
| `normalize_lamalef(text)` | `normalize::normalize_lamalef` (+ `normalize_lamalef_into`) | divergent | alias of `normalize_ligature` (as upstream) → D-008 applies; pinned by the same tests |
| `normalize_spellerrors(text)` | `normalize::normalize_spellerrors` | todo | teh marbuta → heh, alef maksura → yeh |
| `normalize_searchtext(text)` | `normalize::normalize_searchtext` | todo | pipeline order is part of the contract |
| `fix_spaces(text)` | `tokenize::fix_spaces` | todo | verified: upstream defines it in `araby` (`araby.py:1434`) and it collapses runs of spaces outside words; our home is `tokenize` per `AGENTS.md` §3.2 (corrected — was listed under `normalize`) |
| `autocorrect(text)` | `tokenize::autocorrect` | todo | verified: six documented regex rules (leading/isolated ḥarakāt, tanwīn on alef, misplaced sukūn, ḥaraka before shadda, repeated ḥarakāt) |
| `waznlike(word1, wazn, extract_root=False)` | `diacritics::waznlike` | todo | **new row**: compares a word against a vocalization pattern |
| `shaddalike(partial, fully)` | `diacritics::shaddalike` | todo | **new row**: compares two words ignoring shadda placement |

## 3. `araby` vocalization → `diacritics` (Phase 1–2)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `separate(word, extract_shadda=False)` | `diacritics::separate` | todo | verified: the flag is called `extract_shadda` (not `extract`) and, when set, the call returns a third element for the shadda |
| `joint(letters, marks)` | `diacritics::joint` | todo | mismatch → `Error::TashkeelLengthMismatch` |
| `vocalizedlike(word1, word2)` | `diacritics::vocalizedlike` | todo | verified: exactly two parameters — upstream has no `extract` flag |
| `vocalized_similarity(word1, word2)` | `diacritics::vocalized_similarity` | todo | score semantics documented |
| `encode_tashkeel(word, method)` | `diacritics::encode_tashkeel` | todo | Phase 2, N3 |
| `decode_tashkeel(word, marks, method)` | `diacritics::decode_tashkeel` | todo | Phase 2, N3 |

## 4. `araby` tokenization → `tokenize` (Phase 1)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `tokenize(text, …)` | `tokenize::tokenize` | todo | plus zero-allocation `tokens()` |
| `sentence_tokenize(text)` | `tokenize::sentence_tokenize` | todo | TBD: exact upstream behaviour |
| `tokenize_with_location(text)` | `tokenize::tokenize_with_location` | todo | TBD: name/return shape |
| `spellit(word, lang)` | `unicode::names::spellit` | todo | Phase 3, needs `NAMES` |

## 5. `kalima` → `kalima::Word` (Phase 3)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `Kalima(word, marks="")` | `kalima::Word::new` / `from_letters_and_marks` | todo | |
| `Kalima.has_shadda`, `is_vocalized`, `is_arabic` | `kalima::Word::{has_shadda, is_vocalized, is_arabic}` | todo | |
| `Kalima.__eq__`, `__hash__` | `PartialEq`/`Eq`/`Hash` over `(letters, marks)` | todo | |
| `Kalima.__ge__` (partial vocalization) | `kalima::Word::sounds_like` | todo | Python operator overloading not ported |

## 6. `unshape` → `unshape` (Phase 2)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `UNSHAPING_TABLE` | `unshape::DE_SHAPING_TABLE` | todo | include the four pairs upstream omits (`FEF6`, `FEF8`, `FEFA`, `FEFB`) |
| `unshaping_word(word)` | `unshape::to_base_letters` | todo | mapping only, **no** reversal (new D-00N) |
| `unshaping_line(line)` | `unshape::to_base_letters` + reversal helper | todo | |
| `unshaping_text(text)` | `unshape::to_base_letters` | todo | must not add a leading `\n` (E17) |
| text reversal ("قلب النصوص") | `unshape::reverse_chars` / `reverse_words` | todo | explicit names |

## 7. `trans` → `trans` (Phase 2)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `translate(text, table)` | `trans::translate` | todo | internal, table-driven |
| `convert(text, src, dst)` | `trans::convert` | todo | `TransliterationScheme` enum |
| `tim2utf8`, `utf82tim`, `tim2sampa` | `trans::tim_to_arabic`, `arabic_to_tim`, `tim_to_sampa` | todo | TBD: upstream spellings |
| `normalize_digits(text, source, out)` | `trans::normalize_digits` | todo | `DigitSet` enum |
| `segment_language(text)` | `trans::segment_language` | todo | |
| `delimite_language(text, start, end, language)` | `trans::delimit_language` | todo | planned: upstream typo fixed + `#[doc(alias)]`, recorded as a divergence when implemented |

## 8. `number` → `numbers` (Phase 3)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `ArNumbers.int2str(number)` | `numbers::number2text` | todo | pure functions, no state |
| `ArNumbers.set_feminine` / `set_format` | `numbers::Options { gender, case }` | todo | enums instead of mutable flags |
| `number2letters` | `numbers::number2letters` | todo | TBD: exists upstream? |
| `number2ordinal(anumber, feminin=False)` | `numbers::number2ordinal` | todo | N2 |
| `detect_numbers`, `detect_number_words`, `extract_number_phrases`, `detect_number_phrases_position`, `pre_tashkeel_number`, `vocalize_number` | `numbers::*` | todo | all TBD — re-extract with `rg -n '^def '` |
| `number_const` tables | `numbers::tables` | todo | re-derived from grammar, never copied (ADR-0002) |

## 9. `soundex` → `soundex` (Phase 3)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `soundex(word, length=6)` | `soundex::soundex` | todo | `Error::EmptyInput` for `""` |
| `soundex_map_code`, `soundex_trim_rep` | `soundex::{soundex_map_code, soundex_trim_rep}` | todo | |
| `SOUNDEX_CODE`, `SOUNDEX_TRANSLATION` | `soundex::tables` | todo | |

## 10. Out of scope (`P12`)

| Upstream module | Status | Reason |
|---|---|---|
| `named.py`, `named_const.py` (`RAFE3_LIST`, `JAR_LIST`, `NOUN_NASEB_LIST`, `PROPER_NOUNS`) | out-of-scope | grammatical word lists, not text algorithms |
| `propernouns.py` (~630 KB of names) | out-of-scope | data-heavy; would dominate the crate |
| `stack.py` | out-of-scope | Python-only helper, replaced by Rust iterators |
| `arabrepr.py` | out-of-scope | rendering helper, not a text algorithm |
| legacy camelCase aliases (`hasShadda`, `isVocalized`, `isArabicrange`, `isArabicword`) | out-of-scope | D-004 |

---

**How to update this file:** change the status of the affected row in the same commit as the code, add
the `D-00N` reference when the row becomes `divergent`, and never mark a row `done` without a test that
pins the behaviour (`AGENTS.md` §5.5).
# PyArabic parity tracker

Living tracker for behavioural parity with PyArabic
(`linuxscout/pyarabic`, `master` as observed on 2026-10-31). Rules and formats: `AGENTS.md` §5.5.

**Statuses:** `todo` · `wip` · `done` · `divergent` · `out-of-scope`.
`TBD` in a note means "upstream semantics not verified yet" — such a symbol must never be marked
`done`.

**Last updated:** 2026-10-31 (Phase 0 complete: `error` + `unicode`).

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

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `is_hamza`, `is_hamza_above`, `is_hamza_below` | `unicode::classify::is_hamza*` | done | exact upstream spelling TBD |
| `is_vowel` | `unicode::classify::is_vowel` | done | alias of `is_haraka`, verified against `HARAKAT` |
| `is_shadda`, `is_sukun`, `is_tatweel` | `unicode::classify::is_*` | done | |
| `is_ligature` | `unicode::classify::is_ligature` | divergent | D-005: accepts all eight `U+FEF5..=U+FEFC` forms, not only the four isolated ones |
| `is_letter` | `unicode::classify::is_arabic_letter` | done | exact `LETTERS` membership over the Arabic block (tested exhaustively) |
| `is_arabicrange` | `unicode::classify::is_arabicrange` | done | vacuous `true` for empty input, as upstream |
| `is_arabicword` | `unicode::classify::is_arabicword` | divergent | D-002: strict letters/marks/tatweel definition |
| `is_arabicstring` | — | todo | TBD: upstream semantics |
| `has_shadda` | `unicode::classify::has_shadda` | done | |
| `is_vocalized` | `unicode::classify::is_vocalized` | divergent | D-003: shadda alone counts (needs confirmation) |
| `hasShadda`, `isVocalized`, `isArabicrange`, `isArabicword` (legacy camelCase) | — | out-of-scope | D-004 |

### 1.2 r_abic additions (no upstream equivalent)

| `r_abic` symbol | Status | Notes |
|---|---|---|
| `unicode::classify::is_haraka`, `is_tanwin`, `is_short_haraka`, `is_tashkeel` | done | explicit classes for the Phase 1 transforms |
| `unicode::classify::is_arabic_block_char`, `is_arabic_indic_digit`, `is_extended_arabic_indic_digit`, `is_arabic_digit` | done | needed by the tokenizer and `normalize_digits` |
| `unicode::classify::is_formatting`, `unicode::classify::has_harakat`, `has_tashkeel` | done | ZWJ/ZWNJ/BOM/RTL marks (E11–E13) |
| `unicode::constants::{ZERO_WIDTH_JOINER, ZERO_WIDTH_NON_JOINER, LEFT_TO_RIGHT_MARK, RIGHT_TO_LEFT_MARK, ARABIC_LETTER_MARK}` | done | `r_abic` additions |
| `Error`, `Result` | done | Phase 0; `UnsupportedConversion` arrives with `trans` |

---

## 2. `normalize` / `araby` — text transforms → `normalize` (Phase 1)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `strip_tashkeel(text)` | `normalize::strip_tashkeel` | todo | template: `Cow` + `_into` |
| `strip_harakat(text)` | `normalize::strip_harakat` | todo | keeps shadda |
| `strip_lastharaka(text)` | `normalize::strip_lastharaka` | todo | upstream spelling kept, `#[doc(alias)]` |
| `strip_tatweel(text)` | `normalize::strip_tatweel` | todo | |
| `strip_shadda(text)` | `normalize::strip_shadda` | todo | |
| `strip_diacritics(text)` | `normalize::strip_diacritics` | todo | ḥarakāt + small letters |
| `reduce_tashkeel(text)` | `normalize::reduce_tashkeel` | todo | TBD: exact "evident case" rules |
| `reduce_tatweel(text)` | `normalize::reduce_tatweel` | todo | |
| `normalize_hamza(text, method)` | `normalize::normalize_hamza` | todo | `HamzaMethod::{Remove, Tasheel}`; TBD: parameter semantics |
| `normalize_ligature(text)` | `normalize::normalize_ligature` | todo | |
| `normalize_lamalef(text)` | `normalize::normalize_lamalef` | todo | alias of `normalize_ligature` |
| `normalize_spellerrors(text)` | `normalize::normalize_spellerrors` | todo | teh marbuta → heh, alef maksura → yeh |
| `normalize_searchtext(text)` | `normalize::normalize_searchtext` | todo | pipeline order is part of the contract |
| `fix_spaces(text)` | `normalize::fix_spaces` | todo | TBD: owner module upstream |
| `autocorrect(text)` | `normalize::autocorrect` | todo | one documented rule per behaviour |

## 3. `araby` vocalization → `diacritics` (Phase 1–2)

| Upstream symbol | `r_abic` symbol | Status | Notes |
|---|---|---|---|
| `separate(word, extract=False)` | `diacritics::separate` | todo | `(letters, marks)` |
| `joint(letters, marks)` | `diacritics::joint` | todo | mismatch → `Error::TashkeelLengthMismatch` |
| `vocalizedlike(word1, word2, extract=False)` | `diacritics::vocalizedlike` | todo | TBD: signature |
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
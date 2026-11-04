// SPDX-License-Identifier: MIT OR Apache-2.0
//! Character classification predicates for Arabic text.
//!
//! Every predicate is a `const fn` for single code points (usable in `const` contexts, no allocation,
//! no table lookups: contiguous ranges are matched with `matches!`, per `AGENTS.md` §3.5) and a plain
//! `fn` for whole-`&str` helpers.
//!
//! ```
//! use r_abic::unicode::{classify, constants};
//!
//! assert!(classify::is_arabic_letter(constants::BEH));
//! assert!(classify::is_haraka(constants::FATHA));
//! assert!(!classify::is_haraka(constants::SHADDA));
//! assert!(classify::has_shadda("الشَّمس"));
//! ```
//!
//! Nothing here panics; classification never depends on the caller's locale.

use super::constants::{
    HAMZA, HAMZA_ABOVE, HAMZA_BELOW, SHADDA, SMALL_ALEF, SMALL_WAW, SMALL_YEH, SUKUN, TATWEEL,
};

/// Returns `true` if `c` is a ḥaraka (`HARAKAT`: tanwīn, short vowels, sukūn), excluding shadda.
///
/// Mirrors `PyArabic` `araby.is_vowel` / the `HARAKAT` group.
///
/// # Examples
///
/// ```
/// use r_abic::unicode::{classify::is_haraka, constants::{FATHA, SHADDA, TATWEEL}};
///
/// assert!(is_haraka(FATHA));
/// assert!(!is_haraka(SHADDA), "shadda is tashkeel but not a haraka");
/// assert!(!is_haraka(TATWEEL));
/// ```
#[must_use]
pub const fn is_haraka(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{0650}' | '\u{0652}')
}

/// Returns `true` if `c` is one of the three tanwīn marks (`TANWIN`).
#[must_use]
pub const fn is_tanwin(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{064D}')
}

/// Returns `true` if `c` is one of the four short ḥarakāt (`SHORTHARAKAT`).
#[must_use]
pub const fn is_short_haraka(c: char) -> bool {
    matches!(c, '\u{064E}'..='\u{0650}' | '\u{0652}')
}

/// Returns `true` if `c` belongs to `TASHKEEL` (every ḥaraka **plus** shadda).
#[must_use]
pub const fn is_tashkeel(c: char) -> bool {
    matches!(c, '\u{064B}'..='\u{0652}')
}

/// Returns `true` if `c` is shadda (`U+0651`).
#[must_use]
pub const fn is_shadda(c: char) -> bool {
    c == SHADDA
}

/// Returns `true` if `c` is sukūn (`U+0652`).
#[must_use]
pub const fn is_sukun(c: char) -> bool {
    c == SUKUN
}

/// Returns `true` if `c` is tatweel/kashida (`U+0640`); a joiner, never a letter or a mark.
///
/// # Examples
///
/// ```
/// use r_abic::unicode::{classify::is_tatweel, constants::{ALEF, TATWEEL}};
///
/// assert!(is_tatweel(TATWEEL));
/// assert!(!is_tatweel(ALEF));
/// ```
#[must_use]
pub const fn is_tatweel(c: char) -> bool {
    c == TATWEEL
}

/// Returns `true` if `c` is a small (superscript) Arabic letter — the `SMALL` class: superscript
/// alef (`U+0670`), small waw (`U+06E5`) and small yeh (`U+06E6`).
///
/// Mirrors `PyArabic` `araby.is_small` (membership in `SMALL`).
///
/// # Examples
///
/// ```
/// use r_abic::unicode::{
///     classify::is_small,
///     constants::{ALEF, SMALL_ALEF, SMALL_WAW, SMALL_YEH},
/// };
///
/// assert!(is_small(SMALL_ALEF));
/// assert!(is_small(SMALL_WAW));
/// assert!(is_small(SMALL_YEH));
/// assert!(!is_small(ALEF));
/// ```
///
/// # Unicode
///
/// `U+0670` is a combining mark (general category `Mn`) while `U+06E5`/`U+06E6` are modifier
/// letters (`Lm`); upstream groups all three as `SMALL`, and so do we.
#[must_use]
pub const fn is_small(c: char) -> bool {
    matches!(c, SMALL_ALEF | SMALL_WAW | SMALL_YEH)
}

/// Returns `true` for the eight lam-alef ligature presentation forms (`U+FEF5`–`U+FEFC`).
///
/// This includes both the isolated and the final forms of each of the four pairs, so a de-shaping
/// pass never leaves a ligature behind (`AGENTS.md` §4.3, E15).
#[must_use]
pub const fn is_ligature(c: char) -> bool {
    matches!(c, '\u{FEF5}'..='\u{FEFC}')
}

/// Returns `true` if `c` is the hamza letter itself (`U+0621`).
///
/// The carriers (`ALEF_HAMZA_ABOVE`, `WAW_HAMZA`, `YEH_HAMZA`, …) are letters and return `false`.
#[must_use]
pub const fn is_hamza(c: char) -> bool {
    c == HAMZA
}

/// Returns `true` if `c` is the combining hamza above (`U+0654`).
#[must_use]
pub const fn is_hamza_above(c: char) -> bool {
    c == HAMZA_ABOVE
}

/// Returns `true` if `c` is the combining hamza below (`U+0655`).
#[must_use]
pub const fn is_hamza_below(c: char) -> bool {
    c == HAMZA_BELOW
}

/// Returns `true` if `c` is a short vowel; synonym of [`is_haraka`], matching upstream `is_vowel`.
#[must_use]
pub const fn is_vowel(c: char) -> bool {
    is_haraka(c)
}

/// Returns `true` if `c` is a base Arabic letter (the 36 letters of `LETTERS`).
///
/// Tatweel, ḥarakāt, digits, punctuation and presentation forms are **not** letters.
///
/// # Examples
///
/// ```
/// use r_abic::unicode::{classify::is_arabic_letter, constants::{ALEF, TATWEEL}};
///
/// assert!(is_arabic_letter(ALEF));
/// assert!(!is_arabic_letter(TATWEEL), "tatweel is a joiner");
/// ```
///
/// # Parity
///
/// The predicate is exactly `LETTERS.contains(&c)` over the Arabic block; a test pins that equality
/// for every code point of `U+0600..=U+06FF`.
#[must_use]
pub const fn is_arabic_letter(c: char) -> bool {
    matches!(c, '\u{0621}'..='\u{063A}' | '\u{0641}'..='\u{064A}')
}

/// Returns `true` if `c` belongs to the Arabic block (`U+0600`–`U+06FF`).
#[must_use]
pub const fn is_arabic_block_char(c: char) -> bool {
    matches!(c, '\u{0600}'..='\u{06FF}')
}

/// Returns `true` if `c` is an Arabic-Indic digit (`U+0660`–`U+0669`).
#[must_use]
pub const fn is_arabic_indic_digit(c: char) -> bool {
    matches!(c, '\u{0660}'..='\u{0669}')
}

/// Returns `true` if `c` is an Extended Arabic-Indic digit (`U+06F0`–`U+06F9`, Persian/Urdu).
#[must_use]
pub const fn is_extended_arabic_indic_digit(c: char) -> bool {
    matches!(c, '\u{06F0}'..='\u{06F9}')
}

/// Returns `true` if `c` is a digit of either Arabic-Indic set (east or Persian).
///
/// ASCII digits are not included; use `char::is_ascii_digit` for those.
#[must_use]
pub const fn is_arabic_digit(c: char) -> bool {
    is_arabic_indic_digit(c) || is_extended_arabic_indic_digit(c)
}

/// Returns `true` if `c` is an invisible formatting code point we classify explicitly.
///
/// Covers `U+061C` ALM, `U+200C` ZWNJ, `U+200D` ZWJ, `U+200E` LRM, `U+200F` RLM and `U+FEFF` BOM.
/// These are preserved by transforms and never silently dropped (`AGENTS.md` §3.4, E11–E13).
#[must_use]
pub const fn is_formatting(c: char) -> bool {
    matches!(c, '\u{061C}' | '\u{200C}'..='\u{200F}' | '\u{FEFF}')
}

/// Returns `true` if `text` contains at least one shadda.
#[must_use]
pub fn has_shadda(text: &str) -> bool {
    text.chars().any(is_shadda)
}

/// Returns `true` if `text` contains at least one ḥaraka (`HARAKAT`, shadda excluded).
#[must_use]
pub fn has_harakat(text: &str) -> bool {
    text.chars().any(is_haraka)
}

/// Returns `true` if `text` contains at least one `TASHKEEL` mark (ḥaraka or shadda).
#[must_use]
pub fn has_tashkeel(text: &str) -> bool {
    text.chars().any(is_tashkeel)
}

/// Returns `true` if `text` carries any diacritic (ḥaraka or shadda); upstream `isVocalized`.
///
/// A word carrying only a shadda counts as vocalized; see `docs/DIVERGENCES.md`.
#[must_use]
pub fn is_vocalized(text: &str) -> bool {
    has_tashkeel(text)
}

/// Returns `true` if every `char` of `text` lies in the Arabic block (`U+0600`–`U+06FF`).
///
/// Mirrors `PyArabic` `araby.is_arabicrange`, including its vacuous truth for empty input.
///
/// # Examples
///
/// ```
/// use r_abic::unicode::classify::is_arabicrange;
///
/// assert!(is_arabicrange("مرحبا"));
/// assert!(is_arabicrange(""), "vacuously true, as upstream");
/// assert!(!is_arabicrange("مرحبا abc"));
/// ```
#[must_use]
pub fn is_arabicrange(text: &str) -> bool {
    text.chars().all(is_arabic_block_char)
}

/// Returns `true` if `word` is a single Arabic word: letters, marks and tatweel only.
///
/// Spaces, digits, punctuation, presentation forms and formatting code points make it `false`, and the
/// empty string is **not** a word.
///
/// # Examples
///
/// ```
/// use r_abic::unicode::classify::is_arabicword;
///
/// assert!(is_arabicword("العربية"));
/// assert!(is_arabicword("العــــربية"), "tatweel is allowed inside a word");
/// assert!(!is_arabicword("العربية 123"));
/// assert!(!is_arabicword(""));
/// ```
///
/// # Parity
///
/// Upstream's docstring is self-contradictory (it excludes digits and punctuation while claiming to
/// accept everything in the Arabic block), so this predicate pins the stricter reading:
/// `docs/DIVERGENCES.md` D-002.
#[must_use]
pub fn is_arabicword(word: &str) -> bool {
    !word.is_empty()
        && word
            .chars()
            .all(|c| is_arabic_letter(c) || is_tashkeel(c) || is_tatweel(c))
}

#[cfg(test)]
#[allow(clippy::wildcard_imports)] // tests touch nearly every code point constant
mod tests {
    use super::{
        has_harakat, has_shadda, has_tashkeel, is_arabic_block_char, is_arabic_digit,
        is_arabic_indic_digit, is_arabic_letter, is_arabicrange, is_arabicword,
        is_extended_arabic_indic_digit, is_formatting, is_hamza, is_hamza_above, is_hamza_below,
        is_haraka, is_ligature, is_shadda, is_short_haraka, is_small, is_sukun, is_tanwin,
        is_tashkeel, is_tatweel, is_vocalized, is_vowel,
    };
    use crate::unicode::constants::*;
    use crate::unicode::tables::{self, LETTERS, SHORTHARAKAT, TANWIN, TASHKEEL};

    #[test]
    fn marks_are_classified_as_haraka_or_shadda_but_never_both() {
        // (code point, is_haraka, is_tashkeel)
        const CASES: &[(char, bool, bool)] = &[
            (FATHATAN, true, true),
            (DAMMATAN, true, true),
            (KASRATAN, true, true),
            (FATHA, true, true),
            (DAMMA, true, true),
            (KASRA, true, true),
            (SUKUN, true, true),
            (SHADDA, false, true),
            (MADDA_ABOVE, false, false),
            (HAMZA_ABOVE, false, false),
            (HAMZA_BELOW, false, false),
            (SMALL_ALEF, false, false),
            (ALEF, false, false),
            (TATWEEL, false, false),
            (' ', false, false),
            ('a', false, false),
        ];

        for &(c, haraka, tashkeel) in CASES {
            assert_eq!(is_haraka(c), haraka, "is_haraka({c:?})");
            assert_eq!(is_tashkeel(c), tashkeel, "is_tashkeel({c:?})");
            assert_eq!(
                is_vowel(c),
                haraka,
                "is_vowel must mirror is_haraka for {c:?}"
            );
        }
    }

    #[test]
    fn shadda_sukun_tanwin_and_short_harakat_are_individual_classes() {
        assert!(is_shadda(SHADDA));
        assert!(!is_shadda(FATHA));
        assert!(is_sukun(SUKUN));
        assert!(!is_sukun(KASRA));

        for &mark in TANWIN {
            assert!(is_tanwin(mark), "is_tanwin({mark:?})");
            assert!(is_haraka(mark));
        }
        assert!(!is_tanwin(FATHA), "fatha is not tanwin");
        assert!(!is_tanwin(SUKUN));

        for &mark in SHORTHARAKAT {
            assert!(is_short_haraka(mark), "is_short_haraka({mark:?})");
        }
        assert!(!is_short_haraka(FATHATAN), "tanwin is not a short haraka");
        assert!(!is_short_haraka(SHADDA));
    }

    #[test]
    fn letters_exclude_marks_joiners_digits_and_punctuation() {
        for &letter in LETTERS {
            assert!(is_arabic_letter(letter), "is_arabic_letter({letter:?})");
        }

        for c in [
            TATWEEL, FATHA, SHADDA, SUKUN, ZERO, PERCENT, COMMA, QUESTION, ALEF_WASLA,
        ] {
            assert!(
                !is_arabic_letter(c),
                "is_arabic_letter({c:?}) must be false"
            );
        }
        for c in ['a', 'Z', '1', ' ', '🎉'] {
            assert!(
                !is_arabic_letter(c),
                "is_arabic_letter({c:?}) must be false"
            );
        }
    }

    #[test]
    fn ligature_predicate_covers_all_eight_lam_alef_forms() {
        for &ligature in tables::LIGUATURES {
            assert!(is_ligature(ligature), "is_ligature({ligature:?})");
        }
        // The final forms of the same four pairs are ligatures too (E15 in `AGENTS.md` §4.3).
        for c in [LAM_ALEF, '\u{FEF6}', LAM_ALEF_HAMZA_ABOVE, '\u{FEF8}'] {
            assert!(is_ligature(c), "is_ligature({c:?})");
        }
        for c in [LAM, ALEF, '\u{FEF4}', '\u{FEFD}', '\u{FEFF}'] {
            assert!(!is_ligature(c), "is_ligature({c:?}) must be false");
        }
    }

    #[test]
    fn hamza_predicates_are_specific() {
        assert!(is_hamza(HAMZA));
        assert!(
            !is_hamza(ALEF_HAMZA_ABOVE),
            "a carrier is not the hamza letter"
        );
        assert!(!is_hamza(WAW_HAMZA));

        assert!(is_hamza_above(HAMZA_ABOVE));
        assert!(!is_hamza_above(HAMZA_BELOW));
        assert!(
            !is_hamza_above(ALEF_HAMZA_ABOVE),
            "ALEF_HAMZA_ABOVE is a letter, not a mark"
        );

        assert!(is_hamza_below(HAMZA_BELOW));
        assert!(!is_hamza_below(HAMZA_ABOVE));
    }

    #[test]
    fn arabic_block_and_digit_predicates() {
        assert!(is_arabic_block_char(ALEF));
        assert!(is_arabic_block_char(COMMA));
        assert!(is_arabic_block_char(ZERO));
        assert!(!is_arabic_block_char('a'));
        assert!(
            !is_arabic_block_char(LAM_ALEF),
            "presentation forms live outside the block"
        );
        assert!(!is_arabic_block_char('🎉'));

        assert!(is_arabic_indic_digit(ZERO));
        assert!(is_arabic_indic_digit(NINE));
        assert!(!is_arabic_indic_digit(ZERO_P));
        assert!(!is_arabic_indic_digit('0'));

        assert!(is_extended_arabic_indic_digit(ZERO_P));
        assert!(is_extended_arabic_indic_digit(NINE_P));
        assert!(!is_extended_arabic_indic_digit(ZERO));

        for c in [ZERO, FIVE, NINE, ZERO_P, FIVE_P, NINE_P] {
            assert!(is_arabic_digit(c), "is_arabic_digit({c:?})");
        }
        for c in ['0', '9', ALEF, COMMA] {
            assert!(!is_arabic_digit(c), "is_arabic_digit({c:?}) must be false");
        }
    }

    #[test]
    fn formatting_code_points_are_classified_explicitly() {
        for c in [
            ARABIC_LETTER_MARK,
            ZERO_WIDTH_NON_JOINER,
            ZERO_WIDTH_JOINER,
            LEFT_TO_RIGHT_MARK,
            RIGHT_TO_LEFT_MARK,
            BYTE_ORDER_MARK,
        ] {
            assert!(is_formatting(c), "is_formatting({c:?})");
        }
        for c in [ALEF, TATWEEL, ' ', 'a'] {
            assert!(!is_formatting(c), "is_formatting({c:?}) must be false");
        }
    }

    // Written with escapes so the test stays unambiguous about mark order (`AGENTS.md` §3.5, E5).
    const WORD_PLAIN: &str = "\u{0628}"; // ب
    const WORD_VOCALIZED: &str = "\u{0628}\u{064E}"; // بَ
    const WORD_SHADDA_ONLY: &str = "\u{0627}\u{0644}\u{0634}\u{0651}\u{0645}\u{0633}"; // الشّمس

    #[test]
    fn string_helpers_detect_marks() {
        assert!(has_shadda(WORD_SHADDA_ONLY));
        assert!(!has_shadda(WORD_PLAIN));
        assert!(!has_shadda(""));

        assert!(has_harakat(WORD_VOCALIZED));
        assert!(!has_harakat(WORD_PLAIN));
        assert!(!has_harakat(WORD_SHADDA_ONLY), "shadda is not a haraka");

        assert!(has_tashkeel(WORD_VOCALIZED));
        assert!(has_tashkeel(WORD_SHADDA_ONLY), "shadda is tashkeel");
        assert!(!has_tashkeel(WORD_PLAIN));

        assert!(is_vocalized(WORD_VOCALIZED));
        assert!(is_vocalized(WORD_SHADDA_ONLY));
        assert!(!is_vocalized(WORD_PLAIN));
        assert!(!is_vocalized(""));
    }

    #[test]
    fn is_arabicrange_is_vacuously_true_for_empty_input() {
        assert!(
            is_arabicrange(""),
            "mirrors upstream `all()` semantics on empty input"
        );
        assert!(is_arabicrange("مرحبا"));
        assert!(
            is_arabicrange("\u{0661}\u{0662}\u{0663}"),
            "Arabic-Indic digits are in the block"
        );
        assert!(is_arabicrange("\u{0640}"));
        assert!(!is_arabicrange("مرحبا abc"));
        assert!(
            !is_arabicrange("مرحبا 123"),
            "ASCII digits are outside the block"
        );
        assert!(
            !is_arabicrange("\u{FEFB}"),
            "presentation forms are outside the block"
        );
        assert!(!is_arabicrange("🎉"));
    }

    #[test]
    fn is_arabicword_requires_letters_marks_or_tatweel_only() {
        assert!(is_arabicword("مرحبا"));
        assert!(is_arabicword("العربية"));
        assert!(is_arabicword(WORD_SHADDA_ONLY));
        assert!(
            is_arabicword("العــــربية"),
            "tatweel is allowed inside a word (E6)"
        );
        assert!(!is_arabicword(""), "a word cannot be empty");
        assert!(!is_arabicword("مرحبا abc"));
        assert!(!is_arabicword("مرحبا123"));
        assert!(!is_arabicword("،"));
        assert!(!is_arabicword(" "));
        assert!(!is_arabicword("\u{200D}"), "a bare ZWJ is not a word");
        assert!(
            !is_arabicword("\u{FEFB}"),
            "presentation forms are not base letters"
        );
    }

    #[test]
    fn predicates_agree_with_the_class_tables() {
        for &mark in TASHKEEL {
            assert!(is_tashkeel(mark), "TASHKEEL member {mark:?}");
        }
        for &mark in tables::HARAKAT {
            assert!(is_haraka(mark), "HARAKAT member {mark:?}");
        }
        for &letter in LETTERS {
            assert!(is_arabic_letter(letter), "LETTERS member {letter:?}");
        }
        for &ligature in tables::LIGUATURES {
            assert!(is_ligature(ligature), "LIGUATURES member {ligature:?}");
        }
        for &c in tables::HAMZAT {
            assert!(is_arabic_block_char(c), "HAMZAT member {c:?}");
        }
        for &letter in tables::MOON.iter().chain(tables::SUN.iter()) {
            assert!(is_arabic_letter(letter), "{letter:?} must be a base letter");
        }
        for &small in tables::SMALL {
            assert!(
                !is_arabic_letter(small),
                "small letters are not base letters: {small:?}"
            );
        }
        assert_eq!(tables::NOT_DEF_HARAKA, TATWEEL);
        assert!(is_tatweel(tables::NOT_DEF_HARAKA));
    }

    #[test]
    fn is_hamza_and_is_arabicrange_are_stricter_than_upstream() {
        // D-006: only the bare hamza letter counts, not the carriers or the combining marks.
        assert!(is_hamza(HAMZA));
        for carrier in [WAW_HAMZA, YEH_HAMZA, ALEF_HAMZA_ABOVE, ALEF_HAMZA_BELOW] {
            assert!(!is_hamza(carrier), "carrier {carrier:?} is a letter");
            assert!(is_arabic_letter(carrier), "carrier {carrier:?} is a letter");
        }
        assert!(!is_hamza(HAMZA_ABOVE));
        assert!(!is_hamza(HAMZA_BELOW));

        // D-007: only the main Arabic block is `is_arabicrange`; the presentation forms are not.
        assert!(is_arabicrange("\u{0627}\u{064E}"));
        assert!(!is_arabicrange("\u{FEFB}"), "lam-alef ligature");
        assert!(!is_arabicrange("\u{FE8D}"), "alef isolated form");
        assert!(!is_arabicrange("\u{0750}"), "Arabic Supplement");
        assert!(is_arabicrange(""), "vacuously true, as upstream");
    }

    #[test]
    fn is_arabic_letter_is_exactly_the_letters_table_over_the_arabic_block() {
        for code in 0x0600u32..=0x06FF {
            let c = char::from_u32(code).expect("every code point in this range is a valid char");
            assert_eq!(
                is_arabic_letter(c),
                LETTERS.contains(&c),
                "letter classification of U+{code:04X}"
            );
        }
    }

    #[test]
    fn class_implications_hold_across_latin_and_arabic_ranges() {
        for code in 0u32..=0x07FF {
            let c = char::from_u32(code).expect("valid char");

            if is_haraka(c) {
                assert!(is_tashkeel(c), "haraka implies tashkeel: U+{code:04X}");
            }
            if is_tanwin(c) {
                assert!(is_haraka(c), "tanwin implies haraka: U+{code:04X}");
            }
            if is_short_haraka(c) {
                assert!(is_haraka(c), "short haraka implies haraka: U+{code:04X}");
            }
            if is_shadda(c) {
                assert!(is_tashkeel(c) && !is_haraka(c), "shadda: U+{code:04X}");
            }
            if is_sukun(c) {
                assert!(is_haraka(c), "sukun implies haraka: U+{code:04X}");
            }
            if is_tashkeel(c) {
                assert!(
                    is_arabic_block_char(c),
                    "tashkeel marks live in the block: U+{code:04X}"
                );
            }
            if is_arabic_letter(c) {
                assert!(
                    is_arabic_block_char(c),
                    "letters live in the block: U+{code:04X}"
                );
                assert!(!is_tashkeel(c), "a letter is not a mark: U+{code:04X}");
            }
            if is_tatweel(c) {
                assert!(
                    !is_arabic_letter(c) && !is_tashkeel(c),
                    "tatweel: U+{code:04X}"
                );
            }
            if is_arabic_digit(c) {
                assert!(is_arabic_indic_digit(c) || is_extended_arabic_indic_digit(c));
            }
            if is_ligature(c) {
                assert!(!is_arabic_block_char(c), "ligatures are presentation forms");
            }
            assert_eq!(
                is_vowel(c),
                is_haraka(c),
                "is_vowel mirrors is_haraka: U+{code:04X}"
            );
        }
    }

    #[test]
    fn is_small_is_exactly_the_small_table() {
        // `is_small` mirrors upstream `araby.is_small`: plain membership in `SMALL`.
        assert_eq!(
            tables::SMALL.len(),
            3,
            "superscript alef, small waw, small yeh"
        );
        for &small in tables::SMALL {
            assert!(is_small(small), "SMALL member {small:?}");
            assert!(
                !is_arabic_letter(small),
                "small letters are not base letters: {small:?}"
            );
        }
        for code in 0u32..=0x06FF {
            let Some(c) = char::from_u32(code) else {
                continue;
            };
            assert_eq!(
                is_small(c),
                tables::SMALL.contains(&c),
                "U+{code:04X} must be small iff it is in SMALL"
            );
        }
        // Explicit negatives: marks, tatweel, letters, Latin and astral are never "small".
        for not_small in [
            SHADDA,
            SUKUN,
            FATHA,
            TATWEEL,
            ALEF,
            ALEF_MAKSURA,
            'a',
            '\u{1F389}',
        ] {
            assert!(!is_small(not_small), "{not_small:?} is not small");
        }
    }
}

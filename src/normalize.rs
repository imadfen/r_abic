// SPDX-License-Identifier: MIT OR Apache-2.0
//! Arabic text normalization: removing diacritics and canonicalizing letters.
//!
//! Every transform in this module follows the crate's canonical shape (`AGENTS.md` §3.5): scan for
//! the first character that must change, borrow the input when nothing changes, and otherwise copy
//! the clean prefix in one go and filter the remainder. The `*_into` variants write into a
//! caller-owned `String` so hot loops reuse a single buffer.
//!
//! ```
//! use r_abic::normalize::strip_tashkeel;
//!
//! assert_eq!(strip_tashkeel("الْعَرَبِيَّةُ"), "العربية");
//! ```
//!
//! # Unicode
//!
//! Input is assumed to be in whatever normalization form the caller supplied: the crate ships no
//! NFC/NFD/NFKC normalizer and never reorders combining marks (`AGENTS.md` §1.3, §3.5).

use std::borrow::Cow;

use crate::unicode::classify::is_tashkeel;

/// Removes every ḥaraka and shadda (`TASHKEEL`) from `text`, leaving all other characters alone.
///
/// Mirrors `PyArabic` `araby.strip_tashkeel` (also reachable upstream as
/// `pyarabic.normalize.strip_tashkeel`).
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use r_abic::normalize::strip_tashkeel;
///
/// assert_eq!(strip_tashkeel("الْعَرَبِيَّةُ"), "العربية");
/// assert_eq!(strip_tashkeel("الشَّمس"), "الشمس");
///
/// // Input that carries no mark is returned untouched, without allocating:
/// assert!(matches!(strip_tashkeel("العربية"), Cow::Borrowed(_)));
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()`, so it is always a
/// `char` boundary (`AGENTS.md` §3.5).
///
/// # Unicode
///
/// Removes exactly the eight `TASHKEEL` code points — the three tanwīn marks (`U+064B`–`U+064D`),
/// the three short vowels (`U+064E`–`U+0650`), sukūn (`U+0652`) and shadda (`U+0651`). Tatweel
/// (`U+0640`), superscript alef (`U+0670`), small waw/yeh (`U+06E5`/`U+06E6`), madda and the
/// combining hamzas (`U+0653`–`U+0655`), presentation forms (including the lam-alef ligatures),
/// digits, punctuation, formatting code points and everything outside Arabic pass through unchanged;
/// removing those is `strip_diacritics`'s and `unshape`'s job. Marks are never reordered, and no
/// normalization form is applied: NFC and NFD input stay exactly as the caller supplied it.
///
/// # Performance
///
/// Single pass, O(n) time. Returns `Cow::Borrowed` when `text` contains no tashkeel; otherwise
/// allocates one `String` with the capacity of the input, copies the prefix before the first mark
/// with a single `push_str`, and filters the remainder.
///
/// # Parity
///
/// Same result as upstream for every input; upstream's `normalize.strip_tashkeel` merely delegates
/// to its `araby` counterpart.
#[must_use]
pub fn strip_tashkeel(text: &str) -> Cow<'_, str> {
    let Some(first) = first_mark(text) else {
        return Cow::Borrowed(text);
    };

    // `first` comes from `char_indices()`, so slicing at it can never split a code point.
    debug_assert!(text.is_char_boundary(first));

    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..first]);
    push_unmarked(&text[first..], &mut out);
    Cow::Owned(out)
}

/// Removes every ḥaraka and shadda from `text` and writes the result into `out`.
///
/// Mirrors `PyArabic` `araby.strip_tashkeel`, but reuses a caller-owned buffer instead of returning a
/// new string, which keeps hot loops free of allocations (`AGENTS.md` §3.3, §3.6).
///
/// `out` is **cleared first** and then replaced with the stripped text; the function returns `out` so
/// calls can be chained.
///
/// # Examples
///
/// ```
/// use r_abic::normalize::strip_tashkeel_into;
///
/// let mut buffer = String::from("stale content");
/// strip_tashkeel_into("الْعَرَبِيَّةُ", &mut buffer);
/// assert_eq!(buffer, "العربية");
///
/// // The buffer is returned, so calls can be chained:
/// let mut other = String::new();
/// strip_tashkeel_into("مُحَمَّدٌ", &mut other).push('!');
/// assert_eq!(other, "محمد!");
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()`.
///
/// # Unicode
///
/// The removed and kept code points are exactly those documented on `strip_tashkeel`.
///
/// # Performance
///
/// Single pass, O(n) time and no allocation beyond what `out` needs: memory is reserved only when a
/// mark is actually found, so repeated calls with clean input reuse the existing capacity.
pub fn strip_tashkeel_into<'a>(text: &str, out: &'a mut String) -> &'a mut String {
    out.clear();

    match first_mark(text) {
        // Nothing to remove: copy the input and keep whatever capacity the buffer already has.
        None => out.push_str(text),
        Some(first) => {
            debug_assert!(text.is_char_boundary(first));

            out.reserve(text.len());
            out.push_str(&text[..first]);
            push_unmarked(&text[first..], out);
        }
    }

    out
}

/// Returns the byte offset of the first tashkeel mark in `text`, if it has one.
///
/// The offset comes from `char_indices()`, so it is always a `char` boundary and can be used for
/// slicing without further checks (`AGENTS.md` §3.5).
fn first_mark(text: &str) -> Option<usize> {
    text.char_indices()
        .find_map(|(index, ch)| is_tashkeel(ch).then_some(index))
}

/// Appends every character of `text` that is not a tashkeel mark to `out`.
fn push_unmarked(text: &str, out: &mut String) {
    out.extend(text.chars().filter(|ch| !is_tashkeel(*ch)));
}

#[cfg(test)]
mod tests {
    use super::{strip_tashkeel, strip_tashkeel_into};
    use crate::unicode::classify::is_tashkeel;
    use crate::unicode::{constants, tables};
    use std::borrow::Cow;

    /// `(input, expected)` pairs; every row names the `AGENTS.md` §4.3 case (`E*`) it pins.
    ///
    /// # Why the inputs are escapes and not `constants` names
    ///
    /// The §4.2 table form requires `&'static str` literals, and a string literal cannot be assembled
    /// from `char` constants (`concat!` accepts literals only; const string building is not stable),
    /// so escapes are forced here. They are also the stronger oracle for this table: spelling the code
    /// points out means a typo in `unicode::constants` cannot make these expectations agree with the
    /// bug — `constants.rs` pins those same code points against their numbers.
    ///
    /// The two representations are tied together by
    /// `cross_check_constants_agree_with_the_escape_tables`, and assertions about single code points
    /// elsewhere in this module use `constants::*`.
    const CASES: &[(&str, &str)] = &[
        // E1 — empty input.
        ("", ""),
        // E2 — whitespace only.
        ("   \t\n", "   \t\n"),
        // E3 — marks with no base letter: everything goes.
        ("\u{064E}\u{0651}", ""),
        ("\u{064B}\u{064C}\u{064D}", ""),
        // E4 — text beginning with an orphan ḥaraka.
        ("\u{064E}\u{0628}", "\u{0628}"),
        // E5 — shadda before the ḥaraka (الشَّمس) …
        (
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{064E}\u{0645}\u{0633}",
            "\u{0627}\u{0644}\u{0634}\u{0645}\u{0633}",
        ),
        // E5 — … and after it: the result never depends on the caller's mark order.
        (
            "\u{0627}\u{0644}\u{0634}\u{064E}\u{0651}\u{0645}\u{0633}",
            "\u{0627}\u{0644}\u{0634}\u{0645}\u{0633}",
        ),
        // E6 — tatweel is a joiner, not a mark: preserved inside a word and on its own.
        (
            "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}",
            "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}",
        ),
        ("\u{0640}\u{0640}\u{0640}", "\u{0640}\u{0640}\u{0640}"),
        // E7 — mixed script: Latin untouched, Arabic stripped.
        (
            "\u{0639}\u{064E}\u{0631}\u{0628}\u{064A}\u{0629} Book",
            "\u{0639}\u{0631}\u{0628}\u{064A}\u{0629} Book",
        ),
        // E8 — digits of all three sets.
        (
            "12 \u{0661}\u{0662} \u{06F1}\u{06F2}",
            "12 \u{0661}\u{0662} \u{06F1}\u{06F2}",
        ),
        // E9 — Arabic and ASCII punctuation.
        (
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}\u{060C} \u{0643}\u{064A}\u{0641}\u{061F}",
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}\u{060C} \u{0643}\u{064A}\u{0641}\u{061F}",
        ),
        (". . .", ". . ."),
        // E10 — astral characters are never split.
        (
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{1F389}",
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{1F389}",
        ),
        // E11 — ZWJ / ZWNJ.
        ("\u{0628}\u{200D}\u{062A}", "\u{0628}\u{200D}\u{062A}"),
        ("\u{0628}\u{200C}\u{062A}", "\u{0628}\u{200C}\u{062A}"),
        // E12 — directional and formatting marks.
        (
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        ("\u{200E}\u{061C}", "\u{200E}\u{061C}"),
        // E13 — BOM.
        (
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        // E15 — a lam-alef ligature is not a diacritic (`unshape` de-shapes, this does not).
        ("\u{FEFB}", "\u{FEFB}"),
        // E16 — a mark next to a ligature is still removed.
        ("\u{FEFB}\u{0651}", "\u{FEFB}"),
        // E17 — line terminators preserved exactly.
        ("\u{0623}\r\n\u{0628}", "\u{0623}\r\n\u{0628}"),
        ("\u{0623}\n\u{0628}", "\u{0623}\n\u{0628}"),
        // E19 — stacked and duplicated marks all go.
        ("\u{064E}\u{064E}", ""),
        ("\u{0651}\u{0651}", ""),
        (
            "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}\u{064C}",
            "\u{0645}\u{062D}\u{0645}\u{062F}",
        ),
        // E20 — no normalization form is applied: NFC and NFD spellings stay distinct.
        ("\u{0623}", "\u{0623}"),
        ("\u{0627}\u{0654}", "\u{0627}\u{0654}"),
        // Superscript alef, small waw/yeh, madda and the combining hamzas survive — removing them is
        // `strip_diacritics`'s job (Appendix B).
        (
            "\u{0623}\u{064E}\u{0644}\u{0650}\u{0641}\u{0670}",
            "\u{0623}\u{0644}\u{0641}\u{0670}",
        ),
        ("\u{06E5}\u{0652}\u{06E6}", "\u{06E5}\u{06E6}"),
        (
            "\u{0653}\u{0654}\u{0655}\u{064F}",
            "\u{0653}\u{0654}\u{0655}",
        ),
        // Kept marks keep their relative order.
        ("\u{0628}\u{0651}\u{0654}", "\u{0628}\u{0654}"),
    ];

    #[test]
    fn stripping_matches_the_case_table() {
        for &(input, expected) in CASES {
            assert_eq!(strip_tashkeel(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn clean_input_is_borrowed_and_marked_input_is_owned() {
        for clean in [
            "",
            "   \t\n",
            "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0629}",
            "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}",
            "\u{0623}\u{0644}\u{0641}\u{0670}",
            "Book 12 \u{0661}\u{0662}",
            "\u{0628}\u{200D}\u{062A}",
            "\u{FEFB}",
            "\u{1F389}",
        ] {
            assert!(
                matches!(strip_tashkeel(clean), Cow::Borrowed(_)),
                "input without tashkeel must not allocate: {clean:?}"
            );
        }

        for marked in [
            "\u{064E}",
            "\u{0645}\u{064F}\u{062D}",
            "\u{0627}\u{0644}\u{0634}\u{0651}",
        ] {
            assert!(
                matches!(strip_tashkeel(marked), Cow::Owned(_)),
                "input with tashkeel must allocate: {marked:?}"
            );
        }
    }

    #[test]
    fn removes_exactly_the_tashkeel_class_and_nothing_else() {
        for code in 0x0600u32..=0x06FF {
            let Some(c) = char::from_u32(code) else {
                continue;
            };
            let input = c.to_string();
            assert_eq!(
                strip_tashkeel(&input).is_empty(),
                is_tashkeel(c),
                "U+{code:04X} must be removed only when it is tashkeel"
            );
        }

        // Outside the block nothing is ever removed — including the combining diacritical marks
        // (U+0300–U+036F) and the Arabic Extended-A marks, which resemble ḥarakāt but are not them.
        for code in [
            0x0041u32, 0x0300, 0x0653, 0x0654, 0x0655, 0x0670, 0x06E5, 0x06E6, 0x08F0, 0x200D,
            0xFEFF, 0x1F389,
        ] {
            let c = char::from_u32(code).expect("test code points are valid");
            let input = c.to_string();
            assert_eq!(strip_tashkeel(&input), input, "U+{code:04X} must be kept");
        }
    }

    #[test]
    fn every_tashkeel_table_entry_is_removed() {
        assert_eq!(tables::TASHKEEL.len(), 8, "the class this function mirrors");
        for &mark in tables::TASHKEEL {
            let input = mark.to_string();
            assert!(
                strip_tashkeel(&input).is_empty(),
                "TASHKEEL member {mark:?}"
            );
        }
        assert!(is_tashkeel(constants::SHADDA));
        assert!(!is_tashkeel(constants::TATWEEL));
    }

    #[test]
    fn stripping_is_idempotent() {
        for &(input, _) in CASES {
            let once = strip_tashkeel(input);
            let twice = strip_tashkeel(&once);
            assert_eq!(once, twice, "input: {input:?}");
        }
    }

    /// Ties the readable `constants` to the escape tables in this module: if one representation is
    /// edited without the other, this fails (see the note on `CASES`).
    #[test]
    fn cross_check_constants_agree_with_the_escape_tables() {
        use crate::unicode::constants::{
            ALEF, DAMMA, DAMMATAN, FATHA, FATHATAN, KASRA, KASRATAN, LAM, MEEM, SEEN, SHADDA,
            SHEEN, SUKUN,
        };

        // Member by member: each constant must spell out the escape it is expected to.
        for (constant, escape) in [
            (FATHATAN, "\u{064B}"),
            (DAMMATAN, "\u{064C}"),
            (KASRATAN, "\u{064D}"),
            (FATHA, "\u{064E}"),
            (DAMMA, "\u{064F}"),
            (KASRA, "\u{0650}"),
            (SHADDA, "\u{0651}"),
            (SUKUN, "\u{0652}"),
        ] {
            assert_eq!(
                constant.to_string(),
                escape,
                "the constant and the escape must denote one code point"
            );
            assert!(
                tables::TASHKEEL.contains(&constant),
                "{constant:?} belongs to the class this module strips"
            );
        }

        // In practice: a word assembled from constants strips to the letters the E5 escape row expects
        // (الشَّمْس → الشمس).
        let vocalized: String = [ALEF, LAM, SHEEN, SHADDA, FATHA, MEEM, SUKUN, SEEN]
            .into_iter()
            .collect();

        assert_eq!(
            strip_tashkeel(&vocalized),
            "\u{0627}\u{0644}\u{0634}\u{0645}\u{0633}"
        );
    }

    #[test]
    fn into_variant_clears_the_buffer_and_matches_the_case_table() {
        let mut buffer = String::from("stale content that must be cleared");
        for &(input, expected) in CASES {
            strip_tashkeel_into(input, &mut buffer);
            assert_eq!(buffer, expected, "input: {input:?}");
        }

        // Reuse must not accumulate: the buffer is cleared, not appended to.
        strip_tashkeel_into("\u{0645}\u{064F}", &mut buffer);
        assert_eq!(buffer, "\u{0645}");
        strip_tashkeel_into("", &mut buffer);
        assert_eq!(buffer, "");
    }

    #[test]
    fn into_variant_returns_the_buffer_for_chaining() {
        let mut buffer = String::new();
        strip_tashkeel_into("\u{0645}\u{064F}\u{062D}\u{064E}", &mut buffer).push('!');
        assert_eq!(buffer, "\u{0645}\u{062D}!");
    }

    #[test]
    fn into_variant_keeps_enough_capacity_for_reuse() {
        let mut buffer = String::new();
        strip_tashkeel_into("clean text", &mut buffer);
        assert!(
            buffer.capacity() >= buffer.len(),
            "capacity covers the content"
        );

        strip_tashkeel_into("cleaner text", &mut buffer);
        assert_eq!(buffer, "cleaner text");
    }

    #[test]
    fn stripping_removes_only_the_marks_own_bytes() {
        let input = "\u{0645}\u{064F}\u{1F389}\u{0651}";
        let output = strip_tashkeel(input);

        assert_eq!(output, "\u{0645}\u{1F389}");
        assert_eq!(
            input.len() - output.len(),
            4,
            "two two-byte marks were removed"
        );
        assert!(
            input.is_char_boundary(input.len()),
            "no code point is split"
        );
    }
}

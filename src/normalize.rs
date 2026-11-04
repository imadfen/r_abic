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

use crate::unicode::classify::{is_haraka, is_tanwin, is_tashkeel};

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
    strip_by(text, |_, ch| is_tashkeel(ch))
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
    strip_by_into(text, out, |_, ch| is_tashkeel(ch))
}

/// Removes every character `is_stripped` accepts, borrowing `text` when there is nothing to remove.
///
/// This is the single implementation behind the `strip_*` family (`AGENTS.md` §3.5): scan for the
/// first character that must go, copy the clean prefix with one `push_str`, then filter the rest.
/// The predicate receives each character's byte offset in `text`, so a transform may depend on the
/// position (as `strip_lastharaka` does); it is a generic parameter rather than a `fn` pointer, so
/// each caller keeps its own inlined copy of the loop.
fn strip_by<F>(text: &str, is_stripped: F) -> Cow<'_, str>
where
    F: Fn(usize, char) -> bool,
{
    let Some(first) = first_stripped(text, &is_stripped) else {
        return Cow::Borrowed(text);
    };

    // `first` comes from `char_indices()`, so slicing at it can never split a code point.
    debug_assert!(text.is_char_boundary(first));

    let mut out = String::with_capacity(text.len());
    out.push_str(&text[..first]);
    push_kept(&text[first..], first, &mut out, &is_stripped);
    Cow::Owned(out)
}

/// `strip_by` into a caller-owned buffer: clears `out`, then writes the stripped text and returns it.
///
/// Memory is reserved only when something is actually removed, so repeated calls with clean input
/// reuse the capacity the buffer already has (`AGENTS.md` §3.3, §3.6).
fn strip_by_into<'a, F>(text: &str, out: &'a mut String, is_stripped: F) -> &'a mut String
where
    F: Fn(usize, char) -> bool,
{
    out.clear();

    match first_stripped(text, &is_stripped) {
        // Nothing to remove: copy the input and keep whatever capacity the buffer already has.
        None => out.push_str(text),
        Some(first) => {
            debug_assert!(text.is_char_boundary(first));

            out.reserve(text.len());
            out.push_str(&text[..first]);
            push_kept(&text[first..], first, out, &is_stripped);
        }
    }

    out
}

/// Removes every ḥaraka (`HARAKAT`) from `text` and **keeps shadda**.
///
/// Mirrors `PyArabic` `araby.strip_harakat`: the "drop the short vowels but remember the gemination"
/// counterpart of [`strip_tashkeel`].
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use r_abic::normalize::strip_harakat;
///
/// assert_eq!(strip_harakat("الْعَرَبِيَّةُ"), "العربيّة");
///
/// // A shadda is not a ḥaraka, so it survives …
/// assert_eq!(strip_harakat("الشَّمْس"), "الشّمس");
/// // … which is exactly when this function does not have to allocate:
/// assert!(matches!(strip_harakat("الشّمس"), Cow::Borrowed(_)));
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()`, so it is always a
/// `char` boundary (`AGENTS.md` §3.5).
///
/// # Unicode
///
/// Removes exactly the seven `HARAKAT` code points — the three tanwīn marks (`U+064B`–`U+064D`), the
/// three short vowels (`U+064E`–`U+0650`) and sukūn (`U+0652`). Shadda (`U+0651`), tatweel,
/// superscript alef (`U+0670`), small waw/yeh (`U+06E5`/`U+06E6`), madda and the combining hamzas
/// (`U+0653`–`U+0655`), presentation forms (including the lam-alef ligatures), digits, punctuation,
/// formatting code points and everything outside Arabic pass through unchanged; removing those is
/// `strip_tashkeel`'s, `strip_diacritics`'s or `unshape`'s job. Marks are never reordered, and no
/// normalization form is applied: NFC and NFD input stay exactly as the caller supplied it.
///
/// The difference from [`strip_tashkeel`] is exactly one code point: the shadda.
///
/// # Performance
///
/// Single pass, O(n) time. Returns `Cow::Borrowed` when `text` carries no ḥaraka — including a
/// shadda-only word, the case that separates this function from [`strip_tashkeel`]; otherwise
/// allocates one `String` with the capacity of the input, copies the prefix before the first mark
/// with a single `push_str`, and filters the remainder.
///
/// # Parity
///
/// Same result as upstream for every input. Upstream also guards the work with its `is_vocalized`
/// check, which cannot change the outcome: a text without `HARAKAT` has nothing to remove.
#[must_use]
pub fn strip_harakat(text: &str) -> Cow<'_, str> {
    strip_by(text, |_, ch| is_haraka(ch))
}

/// Removes every ḥaraka from `text` and writes the result into `out`, keeping shadda.
///
/// Mirrors `PyArabic` `araby.strip_harakat`, but reuses a caller-owned buffer instead of returning a
/// new string, which keeps hot loops free of allocations (`AGENTS.md` §3.3, §3.6).
///
/// `out` is **cleared first** and then replaced with the stripped text; the function returns `out` so
/// calls can be chained.
///
/// # Examples
///
/// ```
/// use r_abic::normalize::strip_harakat_into;
///
/// let mut buffer = String::from("stale content");
/// strip_harakat_into("الْعَرَبِيَّةُ", &mut buffer);
/// assert_eq!(buffer, "العربيّة");
///
/// // The buffer is returned, so calls can be chained:
/// let mut other = String::new();
/// strip_harakat_into("كِتَابٌ", &mut other).push('!');
/// assert_eq!(other, "كتاب!");
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()`.
///
/// # Unicode
///
/// The removed and kept code points are exactly those documented on [`strip_harakat`].
///
/// # Performance
///
/// Single pass, O(n) time and no allocation beyond what `out` needs: memory is reserved only when a
/// ḥaraka is actually found, so repeated calls with clean input reuse the existing capacity.
pub fn strip_harakat_into<'a>(text: &str, out: &'a mut String) -> &'a mut String {
    strip_by_into(text, out, |_, ch| is_haraka(ch))
}

/// Removes every tanwīn mark and, at most, one trailing ḥaraka from `text`, keeping shadda.
///
/// Mirrors `PyArabic` `araby.strip_lastharaka` — note the upstream spelling (`..._haraka`, not
/// `..._harakat`). Despite the name, the upstream pattern is `[HARAKAT]$|[TANWIN]`: **every** tanwīn
/// mark is removed wherever it appears, while only **one** ḥaraka is removed, and only from the end.
///
/// # Examples
///
/// ```
/// use std::borrow::Cow;
/// use r_abic::normalize::strip_lastharaka;
///
/// // The trailing ḥaraka goes, the internal ones stay:
/// assert_eq!(strip_lastharaka("الْعَرَبِيَّةُ"), "الْعَرَبِيَّة");
/// assert_eq!(strip_lastharaka("مُحَمَّدٌ"), "مُحَمَّد");
///
/// // Nothing tanwīn and no trailing ḥaraka: the input is borrowed, not copied:
/// assert!(matches!(strip_lastharaka("كتاب"), Cow::Borrowed(_)));
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()` (`AGENTS.md` §3.5).
///
/// # Unicode
///
/// Exactly two things are removed: every `TANWIN` mark (`U+064B`–`U+064D`) anywhere, and a `HARAKAT`
/// mark (`U+064E`–`U+0650`, `U+0652`) **only** in the final position.
///
/// "Final position" follows Python's `$` anchor, which upstream's pattern relies on: the end of
/// `text`, or the character just before a **single trailing `\n`**. That second case is upstream
/// behaviour mirrored on purpose (the pattern *is* the parity target), so `strip_lastharaka("بَ\n")`
/// is `"ب\n"` even though the ḥaraka is not the last character; a `\r\n` ending does **not** trigger
/// it, because the `\r` sits between the ḥaraka and the final newline. Both cases are pinned by tests.
///
/// Unlike [`strip_harakat`], this is **not idempotent**: at most one ḥaraka is removed per call, so
/// `بََ` needs two calls to lose both marks (upstream behaves identically). Shadda, tatweel,
/// superscript alef, small waw/yeh, madda and the combining hamzas, presentation forms, digits,
/// punctuation and everything outside Arabic are untouched, and no normalization form is applied.
///
/// # Performance
///
/// Single pass, O(n) time. Returns `Cow::Borrowed` when nothing matches; otherwise allocates one
/// `String` with the capacity of the input, copies the prefix before the first match with a single
/// `push_str`, and filters the remainder.
///
/// # Parity
///
/// Same result as upstream for every input, including the trailing-newline case. Upstream also guards
/// the work with its `is_vocalized` check, which cannot change the outcome.
#[must_use]
#[doc(alias = "strip_last_haraka")]
pub fn strip_lastharaka(text: &str) -> Cow<'_, str> {
    strip_by(text, |index, ch| is_stripped_by_lastharaka(text, index, ch))
}

/// Removes every tanwīn mark and, at most, one trailing ḥaraka from `text`, writing into `out`.
///
/// Mirrors `PyArabic` `araby.strip_lastharaka`, but reuses a caller-owned buffer instead of returning
/// a new string (`AGENTS.md` §3.3, §3.6). `out` is **cleared first** and returned so calls can chain.
///
/// # Examples
///
/// ```
/// use r_abic::normalize::strip_lastharaka_into;
///
/// let mut buffer = String::from("stale content");
/// strip_lastharaka_into("الْعَرَبِيَّةُ", &mut buffer);
/// assert_eq!(buffer, "الْعَرَبِيَّة");
/// ```
///
/// # Panics
///
/// Never panics: every byte offset used internally comes from `char_indices()`.
///
/// # Unicode
///
/// The removed and kept code points are exactly those documented on [`strip_lastharaka`].
///
/// # Performance
///
/// Single pass, O(n) time and no allocation beyond what `out` needs: memory is reserved only when a
/// match is found, so repeated calls with clean input reuse the existing capacity.
pub fn strip_lastharaka_into<'a>(text: &str, out: &'a mut String) -> &'a mut String {
    strip_by_into(text, out, |index, ch| {
        is_stripped_by_lastharaka(text, index, ch)
    })
}

/// `true` when upstream's `[HARAKAT]$|[TANWIN]` pattern matches the character at `index`: every
/// tanwīn mark, plus any ḥaraka that sits where Python's `$` anchor matches ([`at_pattern_end`]).
fn is_stripped_by_lastharaka(text: &str, index: usize, ch: char) -> bool {
    is_haraka(ch) && (is_tanwin(ch) || at_pattern_end(text, index, ch.len_utf8()))
}

/// `true` when the character at byte offset `index` sits where Python's `$` anchor matches: at the
/// very end of `text`, or immediately before a single trailing `\n`.
///
/// Upstream's `strip_lastharaka` relies on `re`'s `$`, which also matches just before a final
/// newline; `\r\n` and a second newline do not. Mirroring that exactly is deliberate — the pattern is
/// the parity target — and both cases are pinned by tests in this module.
fn at_pattern_end(text: &str, index: usize, width: usize) -> bool {
    let after = index + width;

    after == text.len() || (after + 1 == text.len() && text.ends_with('\n'))
}

/// Returns the byte offset of the first character in `text` that `is_stripped` accepts, if any.
///
/// The offset comes from `char_indices()`, so it is always a `char` boundary and can be used for
/// slicing without further checks (`AGENTS.md` §3.5).
fn first_stripped<F>(text: &str, is_stripped: &F) -> Option<usize>
where
    F: Fn(usize, char) -> bool,
{
    text.char_indices()
        .find_map(|(index, ch)| is_stripped(index, ch).then_some(index))
}

/// Appends every character of `text` that `is_stripped` rejects to `out`.
///
/// `base` is the byte offset of `text` inside the string being processed, so the predicate always
/// sees offsets relative to the whole input rather than to the slice it is filtering.
fn push_kept<F>(text: &str, base: usize, out: &mut String, is_stripped: &F)
where
    F: Fn(usize, char) -> bool,
{
    out.extend(
        text.char_indices()
            .filter(|&(index, ch)| !is_stripped(base + index, ch))
            .map(|(_, ch)| ch),
    );
}

#[cfg(test)]
mod tests {
    use super::{
        strip_harakat, strip_harakat_into, strip_lastharaka, strip_lastharaka_into, strip_tashkeel,
        strip_tashkeel_into,
    };
    use crate::unicode::classify::{is_haraka, is_shadda, is_tanwin, is_tashkeel};
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

    /// `(input, expected)` pairs for `strip_harakat`, written in the same escape convention as
    /// `CASES` (see the note there) and deliberately laid out next to the `strip_tashkeel` rows so the
    /// shadda cases stand out: `strip_harakat` keeps `U+0651`, `strip_tashkeel` removes it.
    const HARAKAT_CASES: &[(&str, &str)] = &[
        // E1 — empty input.
        ("", ""),
        // E2 — whitespace only.
        ("   \t\n", "   \t\n"),
        // E3 — marks with no base letter: only the shadda survives.
        ("\u{064E}\u{0651}", "\u{0651}"),
        ("\u{064B}\u{064C}\u{064D}", ""),
        // E4 — text beginning with an orphan ḥaraka.
        ("\u{064E}\u{0628}", "\u{0628}"),
        // E5 — shadda before the ḥaraka …
        (
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{064E}\u{0645}\u{0633}",
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{0645}\u{0633}",
        ),
        // E5 — … and after it: identical result, shadda in place.
        (
            "\u{0627}\u{0644}\u{0634}\u{064E}\u{0651}\u{0645}\u{0633}",
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{0645}\u{0633}",
        ),
        // E6 — tatweel is a joiner, not a ḥaraka.
        (
            "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}",
            "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}",
        ),
        // E7 — mixed script.
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
        // E10 — astral characters.
        (
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{1F389}",
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627} \u{1F389}",
        ),
        // E11–E13 — joiners, directional marks and BOM.
        ("\u{0628}\u{200D}\u{062A}", "\u{0628}\u{200D}\u{062A}"),
        (
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        (
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        // E15/E16 — a ligature is not a ḥaraka, and a shadda attached to one stays attached.
        ("\u{FEFB}\u{0651}", "\u{FEFB}\u{0651}"),
        // E17 — line terminators preserved exactly.
        ("\u{0623}\r\n\u{0628}", "\u{0623}\r\n\u{0628}"),
        // E19 — stacked ḥarakāt all go, stacked shaddas all stay.
        ("\u{064E}\u{064E}", ""),
        ("\u{0651}\u{0651}", "\u{0651}\u{0651}"),
        // E19 — the realistic word: الْعَرَبِيَّةُ → العربيّة (upstream's own docstring example).
        (
            "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}\u{064F}",
            "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0651}\u{0629}",
        ),
        // E20 — no normalization: NFC and NFD spellings stay distinct.
        ("\u{0627}\u{0654}", "\u{0627}\u{0654}"),
        // Superscript alef, small waw/yeh and the combining marks survive (not ḥarakāt).
        ("\u{06E5}\u{0652}\u{06E6}", "\u{06E5}\u{06E6}"),
        (
            "\u{0653}\u{0654}\u{0655}\u{064F}",
            "\u{0653}\u{0654}\u{0655}",
        ),
    ];

    /// `(input, expected)` pairs for `strip_lastharaka`, in the same escape convention as `CASES`.
    ///
    /// Upstream's pattern is `[HARAKAT]$|[TANWIN]`, so every row answers two questions: does it hold
    /// tanwīn (removed anywhere), and does it end in a ḥaraka (removed, once)? Rows that show the
    /// differences from the sibling functions carry comments; those marked "probe" were reproduced
    /// with plain Python `re` and the same pattern.
    const LASTHARAKA_CASES: &[(&str, &str)] = &[
        // E1 — empty input; E2 — whitespace only.
        ("", ""),
        ("   \t\n", "   \t\n"),
        // E3 — the fatha is *not* trailing once the shadda follows it, so nothing is removed here ...
        ("\u{064E}\u{0651}", "\u{064E}\u{0651}"),
        // ... whereas a ḥaraka in the true final position does go, shadda or not (probe).
        ("\u{0651}\u{064E}", "\u{0651}"),
        // E3 — tanwīn only.
        ("\u{064B}\u{064C}\u{064D}", ""),
        // E4 — a *leading* ḥaraka is not trailing, so it survives (contrast with `strip_harakat`).
        ("\u{064E}\u{0628}", "\u{064E}\u{0628}"),
        ("\u{064E}\u{0628}\u{064E}", "\u{064E}\u{0628}"),
        // The single-pass trap (probe): the fatha is not at the end of the *input*, so a two-pass
        // "remove tanwīn, then look for a trailing ḥaraka" implementation would wrongly drop it.
        ("\u{0628}\u{064E}\u{064C}", "\u{0628}\u{064E}"),
        // ... in the reverse mark order both marks go (probe).
        ("\u{0628}\u{064C}\u{064E}", "\u{0628}"),
        // Mid-text tanwīn goes even when nothing is trailing (probe).
        ("\u{0628}\u{064B}\u{062A}", "\u{0628}\u{062A}"),
        ("\u{0628}\u{064B}\u{062A}\u{064E}", "\u{0628}\u{062A}"),
        // A trailing shadda is not a ḥaraka (probe).
        ("\u{0628}\u{0651}", "\u{0628}\u{0651}"),
        // E5 — internal ḥarakāt stay: only the trailing damma goes (upstream's own example word).
        (
            "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}\u{064F}",
            "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}",
        ),
        // The shadda/ḥaraka order does not change the outcome, and nothing is trailing here.
        (
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{064E}\u{0645}\u{0633}",
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{064E}\u{0645}\u{0633}",
        ),
        // E6 — tatweel is a joiner; E7 — mixed script; E8 — digits of three sets.
        (
            "\u{0628}\u{0640}\u{0640}\u{0645}",
            "\u{0628}\u{0640}\u{0640}\u{0645}",
        ),
        (
            "\u{0639}\u{064E}\u{0631}\u{0628} Book 12 \u{0661}\u{06F1}",
            "\u{0639}\u{064E}\u{0631}\u{0628} Book 12 \u{0661}\u{06F1}",
        ),
        // E9 — punctuation; E10 — astral characters.
        (
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}\u{060C} \u{1F389}",
            "\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}\u{060C} \u{1F389}",
        ),
        // E11–E13 — joiners, directional marks and BOM; the trailing sukūn still goes.
        (
            "\u{0628}\u{200D}\u{062A}\u{0652}",
            "\u{0628}\u{200D}\u{062A}",
        ),
        (
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{200F}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        (
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
            "\u{FEFF}\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
        ),
        // E15/E16 — a ligature is not a mark and a trailing shadda on it stays.
        ("\u{FEFB}\u{0651}", "\u{FEFB}\u{0651}"),
        // E17 — Python's `$` also matches just before a *single* trailing newline (probe), which we
        // mirror on purpose; `\r\n` and a second newline block the anchor (probe).
        ("\u{0628}\u{064E}\n", "\u{0628}\n"),
        ("\u{0628}\u{064E}\r\n", "\u{0628}\u{064E}\r\n"),
        ("\u{0628}\u{064E}\n\n", "\u{0628}\u{064E}\n\n"),
        // E19 — one ḥaraka per call: the reason this function is not idempotent.
        ("\u{0628}\u{064E}\u{064E}", "\u{0628}\u{064E}"),
        // E20 — no normalization; the non-ḥaraka combining marks survive.
        ("\u{0627}\u{0654}", "\u{0627}\u{0654}"),
        ("\u{06E5}\u{0652}\u{06E6}", "\u{06E5}\u{0652}\u{06E6}"),
        (
            "\u{0653}\u{0654}\u{0655}\u{064F}",
            "\u{0653}\u{0654}\u{0655}",
        ),
    ];

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
    fn strip_lastharaka_matches_the_case_table() {
        for &(input, expected) in LASTHARAKA_CASES {
            assert_eq!(strip_lastharaka(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn strip_lastharaka_removes_every_tanwin_and_only_a_trailing_haraka() {
        for &mark in tables::TANWIN {
            assert!(is_tanwin(mark), "TANWIN member {mark:?}");
            assert!(
                strip_lastharaka(&mark.to_string()).is_empty(),
                "tanwīn {mark:?} must go wherever it is"
            );

            let embedded = format!("\u{0628}{mark}\u{062A}");
            assert_eq!(
                strip_lastharaka(&embedded),
                "\u{0628}\u{062A}",
                "mid-text {mark:?} must go"
            );
        }

        for &mark in tables::SHORTHARAKAT {
            assert!(
                strip_lastharaka(&mark.to_string()).is_empty(),
                "trailing {mark:?} must go"
            );

            let embedded = format!("\u{0628}{mark}\u{062A}");
            assert_eq!(
                strip_lastharaka(&embedded),
                embedded,
                "mid-text {mark:?} must survive"
            );
        }

        for untouched in ["\u{0651}", "\u{0628}\u{0651}", "\u{0651}\u{0628}"] {
            assert_eq!(
                strip_lastharaka(untouched),
                untouched,
                "shadda is never touched: {untouched:?}"
            );
        }
    }

    #[test]
    fn strip_lastharaka_needs_one_call_per_trailing_haraka() {
        let double = "\u{0628}\u{064E}\u{064E}";
        let once = strip_lastharaka(double);
        let twice = strip_lastharaka(&once);
        let thrice = strip_lastharaka(&twice);

        assert_eq!(once, "\u{0628}\u{064E}");
        assert_eq!(twice, "\u{0628}");
        assert_eq!(thrice, "\u{0628}", "a third call changes nothing");
        assert_ne!(once, twice, "the documented non-idempotence");
    }

    #[test]
    fn strip_lastharaka_borrows_input_it_does_not_change() {
        for clean in [
            "",
            "\u{0643}\u{062A}\u{0627}\u{0628}",
            "\u{0628}\u{0651}",
            "\u{064E}\u{0628}",
            "\u{0628}\u{064E}\r\n",
            "\u{0628}\u{0652}\u{062A}",
        ] {
            assert!(
                matches!(strip_lastharaka(clean), Cow::Borrowed(_)),
                "nothing to remove in {clean:?}"
            );
        }
    }

    #[test]
    fn strip_lastharaka_into_clears_the_buffer_and_matches_the_case_table() {
        let mut buffer = String::from("stale content that must be cleared");
        for &(input, expected) in LASTHARAKA_CASES {
            strip_lastharaka_into(input, &mut buffer);
            assert_eq!(buffer, expected, "input: {input:?}");
        }

        strip_lastharaka_into("\u{0628}\u{064E}", &mut buffer);
        assert_eq!(buffer, "\u{0628}");
        strip_lastharaka_into("", &mut buffer);
        assert_eq!(buffer, "");

        strip_lastharaka_into("\u{0628}\u{064B}", &mut buffer).push('!');
        assert_eq!(buffer, "\u{0628}!");
    }

    #[test]
    fn strip_lastharaka_matches_the_words_spelled_out_in_the_documentation() {
        // The doc examples spell these words in Arabic script; pinning them with escapes means a
        // hand-typed mark can never silently change what the documentation promises.
        let arabiyya = "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}\u{064F}";
        let muhammad = "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}\u{064C}";

        assert_eq!(
            strip_lastharaka(arabiyya),
            "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}"
        );
        assert_eq!(
            strip_lastharaka(muhammad),
            "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}"
        );
    }

    #[test]
    fn strip_harakat_matches_the_case_table() {
        for &(input, expected) in HARAKAT_CASES {
            assert_eq!(strip_harakat(input), expected, "input: {input:?}");
        }
    }

    #[test]
    fn strip_harakat_keeps_shadda_and_borrows_such_input() {
        for shadda_only in [
            "\u{0651}",
            "\u{0651}\u{0651}",
            "\u{0627}\u{0644}\u{0634}\u{0651}\u{0645}\u{0633}",
        ] {
            assert_eq!(
                strip_harakat(shadda_only),
                shadda_only,
                "input: {shadda_only:?}"
            );
            assert!(
                matches!(strip_harakat(shadda_only), Cow::Borrowed(_)),
                "shadda is not a ḥaraka, so nothing is removed and nothing is allocated: \
                 {shadda_only:?}"
            );
        }
    }

    #[test]
    fn strip_harakat_removes_exactly_the_harakat_class_and_nothing_else() {
        for code in 0x0600u32..=0x06FF {
            let Some(c) = char::from_u32(code) else {
                continue;
            };
            let input = c.to_string();
            assert_eq!(
                strip_harakat(&input).is_empty(),
                is_haraka(c),
                "U+{code:04X} must be removed only when it is a ḥaraka"
            );
        }

        // Shadda is tashkeel but not a ḥaraka; nothing outside the block is touched either.
        for kept in [
            "\u{0651}",
            "\u{0640}",
            "\u{0670}",
            "\u{06E5}",
            "\u{06E6}",
            "\u{0653}",
            "\u{0654}",
            "\u{0655}",
            "\u{0300}",
            "a",
            "\u{1F389}",
        ] {
            assert_eq!(strip_harakat(kept), kept, "{kept:?} must be kept");
        }
    }

    #[test]
    fn strip_harakat_and_strip_tashkeel_differ_by_exactly_the_shadda() {
        assert_eq!(tables::HARAKAT.len(), 7, "the class this function mirrors");
        for &mark in tables::HARAKAT {
            let input = mark.to_string();
            assert!(strip_harakat(&input).is_empty(), "HARAKAT member {mark:?}");
        }
        for &mark in tables::TASHKEEL {
            let input = mark.to_string();
            let stripped = strip_harakat(&input);
            assert_eq!(
                stripped.is_empty(),
                !is_shadda(mark),
                "of {mark:?} only the shadda survives strip_harakat"
            );
        }
    }

    #[test]
    fn strip_harakat_then_strip_tashkeel_equals_strip_tashkeel_alone() {
        for &(input, _) in CASES.iter().chain(HARAKAT_CASES) {
            assert_eq!(
                strip_tashkeel(&strip_harakat(input)),
                strip_tashkeel(input),
                "input: {input:?}"
            );
            assert_eq!(
                strip_harakat(&strip_tashkeel(input)),
                strip_tashkeel(input),
                "input: {input:?}"
            );
        }
    }

    #[test]
    fn strip_harakat_is_idempotent() {
        for &(input, _) in HARAKAT_CASES {
            let once = strip_harakat(input);
            let twice = strip_harakat(&once);
            assert_eq!(once, twice, "input: {input:?}");
        }
    }

    #[test]
    fn strip_harakat_removes_only_the_harakat_own_bytes() {
        let input = "\u{0645}\u{064F}\u{0651}";
        let output = strip_harakat(input);

        assert_eq!(output, "\u{0645}\u{0651}");
        assert_eq!(input.len() - output.len(), 2, "only the damma is two bytes");
    }

    #[test]
    fn strip_harakat_into_clears_the_buffer_and_matches_the_case_table() {
        let mut buffer = String::from("stale content that must be cleared");
        for &(input, expected) in HARAKAT_CASES {
            strip_harakat_into(input, &mut buffer);
            assert_eq!(buffer, expected, "input: {input:?}");
        }

        strip_harakat_into("\u{0645}\u{064F}\u{0651}", &mut buffer);
        assert_eq!(
            buffer, "\u{0645}\u{0651}",
            "shadda survives, damma does not"
        );
        strip_harakat_into("", &mut buffer);
        assert_eq!(buffer, "");

        // The buffer is returned, so calls can be chained.
        strip_harakat_into("\u{0645}\u{064F}", &mut buffer).push('!');
        assert_eq!(buffer, "\u{0645}!");
    }

    #[test]
    fn strip_harakat_matches_the_words_spelled_out_in_the_documentation() {
        // The doc examples spell these words in Arabic script; pinning them with escapes means a
        // hand-typed mark can never silently change what the documentation promises.
        let muhammad = "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}\u{064C}";
        let kitab = "\u{0643}\u{0650}\u{062A}\u{064E}\u{0627}\u{0628}\u{064C}";

        assert_eq!(
            strip_harakat(muhammad),
            "\u{0645}\u{062D}\u{0645}\u{0651}\u{062F}"
        );
        assert_eq!(strip_harakat(kitab), "\u{0643}\u{062A}\u{0627}\u{0628}");
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

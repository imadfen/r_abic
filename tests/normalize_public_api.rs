// SPDX-License-Identifier: MIT OR Apache-2.0
//! Integration tests for the public normalization API.
//!
//! Only the public surface is used (`AGENTS.md` §4.2); expectations are built from escapes so the
//! intended code points are unambiguous.

use std::borrow::Cow;

use r_abic::normalize::{
    strip_harakat, strip_harakat_into, strip_lastharaka, strip_lastharaka_into, strip_tashkeel,
    strip_tashkeel_into,
};
use r_abic::unicode::classify;

/// الْعَرَبِيَّةُ — al-ʿarabiyya, spelled out as letters + marks.
///
/// Escapes instead of `constants` names are forced here: a `const &str` cannot be assembled from
/// `char` constants. The same code points are pinned against their numbers in
/// `unicode::constants`, and `normalize::tests::cross_check_constants_agree_with_the_escape_tables`
/// ties the two representations together.
const VOCALIZED_WORD: &str = "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}\u{064F}";
/// العربية — the same word without diacritics.
const PLAIN_WORD: &str = "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0629}";

#[test]
fn strips_a_vocalized_word() {
    assert_eq!(strip_tashkeel(VOCALIZED_WORD), PLAIN_WORD);
    assert_eq!(strip_tashkeel(PLAIN_WORD), PLAIN_WORD);
}

#[test]
fn strips_a_vocalized_sentence_without_touching_the_rest() {
    // A realistic document line: Arabic prose, a Latin word and digits of two sets.
    let text = format!("{VOCALIZED_WORD} Book 12 \u{0661}\u{0662}\u{06F1}\u{06F2}\u{061F}");
    let expected = format!("{PLAIN_WORD} Book 12 \u{0661}\u{0662}\u{06F1}\u{06F2}\u{061F}");

    assert_eq!(strip_tashkeel(&text), expected);
    assert!(
        classify::is_arabicrange(&expected) || expected.contains(' '),
        "the Latin part is preserved verbatim"
    );
}

#[test]
fn empty_and_mark_only_input() {
    assert!(matches!(strip_tashkeel(""), Cow::Borrowed("")));
    assert_eq!(strip_tashkeel("\u{064E}\u{0651}"), "");
}

#[test]
fn clean_input_is_borrowed_and_keeps_its_bytes() {
    let output = strip_tashkeel(PLAIN_WORD);

    assert!(matches!(output, Cow::Borrowed(_)), "zero-copy fast path");
    assert_eq!(output.as_bytes(), PLAIN_WORD.as_bytes());
}

#[test]
fn tatweel_and_ligatures_are_not_diacritics() {
    let stretched = "\u{0627}\u{0644}\u{0639}\u{0640}\u{0640}\u{0631}\u{0628}\u{064A}\u{0629}";
    assert_eq!(strip_tashkeel(stretched), stretched);

    // U+FEFB is the plain lam-alef ligature: de-shaping is `unshape`'s job, not this one's.
    let ligature = "\u{FEFB}";
    assert_eq!(strip_tashkeel(ligature), ligature);
    assert!(classify::is_ligature('\u{FEFB}'));
}

#[test]
fn line_terminators_survive_unchanged() {
    let lf = "\u{0623}\u{064E}\u{0628}\n\u{062A}\u{0652}\u{0645}";
    let crlf = "\u{0623}\u{064E}\u{0628}\r\n\u{062A}\u{0652}\u{0645}";
    let expected_lf = "\u{0623}\u{0628}\n\u{062A}\u{0645}";
    let expected_crlf = "\u{0623}\u{0628}\r\n\u{062A}\u{0645}";

    assert_eq!(strip_tashkeel(lf), expected_lf);
    assert_eq!(strip_tashkeel(crlf), expected_crlf);
    assert_eq!(
        strip_tashkeel(crlf).matches('\n').count(),
        crlf.matches('\n').count(),
        "no line terminator is added or lost"
    );
}

#[test]
fn no_normalization_form_is_applied() {
    // NFC spells this word with one code point (ALEF_HAMZA_ABOVE), NFD with alef + combining hamza.
    let nfc = "\u{0623}\u{064E}\u{0645}\u{0646}";
    let nfd = "\u{0627}\u{0654}\u{064E}\u{0645}\u{0646}";

    assert_eq!(strip_tashkeel(nfc), "\u{0623}\u{0645}\u{0646}");
    assert_eq!(strip_tashkeel(nfd), "\u{0627}\u{0654}\u{0645}\u{0646}");
    assert_ne!(
        strip_tashkeel(nfc),
        strip_tashkeel(nfd),
        "marks are never reordered or recomposed (`AGENTS.md` §3.5)"
    );
}

#[test]
fn strip_harakat_keeps_shadda_where_strip_tashkeel_removes_it() {
    // الشَّمْس — letters + shadda + ḥaraka, the two spellings of the same mark order.
    let shadda_first = "\u{0627}\u{0644}\u{0634}\u{0651}\u{064E}\u{0645}\u{0633}";
    let fatha_first = "\u{0627}\u{0644}\u{0634}\u{064E}\u{0651}\u{0645}\u{0633}";
    let with_shadda = "\u{0627}\u{0644}\u{0634}\u{0651}\u{0645}\u{0633}";
    let plain = "\u{0627}\u{0644}\u{0634}\u{0645}\u{0633}";

    assert_eq!(strip_harakat(shadda_first), with_shadda);
    assert_eq!(strip_harakat(fatha_first), with_shadda);
    assert_eq!(strip_tashkeel(shadda_first), plain);

    // Nothing to remove in a shadda-only word: the borrowing variant is taken.
    assert!(matches!(strip_harakat(with_shadda), Cow::Borrowed(_)));
}

#[test]
fn strip_harakat_leaves_the_word_partially_vocalized() {
    assert_eq!(
        strip_harakat(VOCALIZED_WORD),
        "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0651}\u{0629}"
    );
}

#[test]
fn strip_harakat_into_can_reuse_one_buffer_across_a_document() {
    let mut buffer = String::new();

    for (word, expected) in [
        (
            VOCALIZED_WORD,
            "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0651}\u{0629}",
        ),
        (PLAIN_WORD, PLAIN_WORD),
        ("\u{0645}\u{064F}\u{0651}", "\u{0645}\u{0651}"),
    ] {
        strip_harakat_into(word, &mut buffer);

        assert_eq!(buffer, expected, "word: {word:?}");
    }
}

#[test]
fn strip_lastharaka_takes_only_the_final_haraka_and_every_tanwin() {
    // الْعَرَبِيَّةُ → الْعَرَبِيَّة: the trailing damma goes, the internal marks and the shadda stay.
    let vocalized = "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}\u{064F}";
    let expected_partial = "\u{0627}\u{0644}\u{0652}\u{0639}\u{064E}\u{0631}\u{064E}\u{0628}\u{0650}\u{064A}\u{064E}\u{0651}\u{0629}";

    assert_eq!(strip_lastharaka(vocalized), expected_partial);
    assert_eq!(strip_lastharaka(expected_partial), expected_partial);

    // The three siblings differ by exactly what they remove.
    assert_eq!(strip_tashkeel(vocalized), PLAIN_WORD);
    assert_eq!(
        strip_harakat(vocalized),
        "\u{0627}\u{0644}\u{0639}\u{0631}\u{0628}\u{064A}\u{0651}\u{0629}"
    );
}

#[test]
fn strip_lastharaka_mirrors_the_trailing_newline_behaviour() {
    // Python's `$` also matches just before a single trailing newline, and the upstream pattern
    // relies on it, so we mirror that: the ḥaraka is removed even though it is not the last char.
    assert_eq!(
        strip_lastharaka("\u{0628}\u{064E}\n"),
        "\u{0628}\n",
        "mirrored upstream behaviour"
    );
    // `\r\n` and a second newline block the anchor, so nothing changes there.
    assert_eq!(
        strip_lastharaka("\u{0628}\u{064E}\r\n"),
        "\u{0628}\u{064E}\r\n"
    );
    assert_eq!(
        strip_lastharaka("\u{0628}\u{064E}\n\n"),
        "\u{0628}\u{064E}\n\n"
    );
}

#[test]
fn strip_lastharaka_into_can_reuse_one_buffer() {
    let mut buffer = String::new();

    for (word, expected) in [
        ("\u{0628}\u{064E}", "\u{0628}"),
        ("\u{0628}\u{064B}\u{062A}", "\u{0628}\u{062A}"),
        ("\u{0628}\u{0651}", "\u{0628}\u{0651}"),
    ] {
        strip_lastharaka_into(word, &mut buffer);

        assert_eq!(buffer, expected, "word: {word:?}");
    }
}

#[test]
fn into_variant_can_reuse_one_buffer_across_a_document() {
    let mut buffer = String::new();
    let mut stripped = Vec::new();

    for word in [
        VOCALIZED_WORD,
        PLAIN_WORD,
        "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}",
    ] {
        stripped.push(strip_tashkeel_into(word, &mut buffer).clone());
    }

    assert_eq!(
        stripped,
        vec![
            PLAIN_WORD.to_string(),
            PLAIN_WORD.to_string(),
            "\u{0645}\u{062D}\u{0645}\u{062F}".to_string(),
        ]
    );
}

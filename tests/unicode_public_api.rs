// SPDX-License-Identifier: MIT OR Apache-2.0
//! Integration tests for the Phase 0 public API.
//!
//! These tests deliberately use only the public surface of the crate (`AGENTS.md` §4.2): if a test
//! needs `pub(crate)` access, the item should probably be public instead.

use r_abic::Error;
use r_abic::unicode::{classify, constants, tables};

#[test]
fn code_points_are_reachable_and_stable() {
    assert_eq!(u32::from(constants::ALEF), 0x0627);
    assert_eq!(u32::from(constants::TATWEEL), 0x0640);
    assert_eq!(u32::from(constants::LAM_ALEF), 0xFEFB);
    assert_eq!(constants::MINI_ALEF, constants::SMALL_ALEF);
    assert_eq!(tables::NOT_DEF_HARAKA, constants::TATWEEL);
}

#[test]
fn class_tables_are_usable_from_outside_the_crate() {
    assert_eq!(tables::HARAKAT.len(), 7);
    assert_eq!(tables::TASHKEEL.len(), 8);
    assert_eq!(tables::LETTERS.len(), 36);
    assert_eq!(tables::MOON.len(), 18);
    assert_eq!(tables::SUN.len(), 14);
    assert_eq!(tables::NUMBERS_EAST[0], constants::ZERO);
    assert_eq!(tables::NUMBERS_WEST[9], constants::NINE_W);
    assert_eq!(tables::alphabetic_order(constants::YEH), Some(28));
}

#[test]
fn predicates_classify_real_arabic_text() {
    // Plain word (E7/E9): letters only.
    let word = "العربية";
    assert!(classify::is_arabicword(word));
    assert!(classify::is_arabicrange(word));
    assert!(!classify::has_tashkeel(word));

    // Tatweel inside a word is allowed but never a letter (E6).
    let stretched = "العــــربية";
    assert!(classify::is_arabicword(stretched));
    assert!(stretched.chars().any(classify::is_tatweel));
    let letter_count = stretched
        .chars()
        .filter(|&c| classify::is_arabic_letter(c))
        .count();
    assert_eq!(
        letter_count, 7,
        "the 7 letters survive; tatweel is not a letter"
    );

    // Diacritics: shadda is tashkeel, not a ḥaraka (E5).
    let vocalized = "\u{0645}\u{064F}\u{062D}\u{064E}\u{0645}\u{0651}\u{064E}\u{062F}"; // مُحَمَّد
    assert!(classify::has_tashkeel(vocalized));
    assert!(classify::has_harakat(vocalized));
    assert!(classify::has_shadda(vocalized));
    assert!(classify::is_vocalized(vocalized));

    // Mixed script and astral input must be rejected, not panicked on (E7/E10).
    assert!(!classify::is_arabicrange("العربية Book"));
    assert!(!classify::is_arabic_letter('🎉'));
    assert!(!classify::is_formatting('🎉'));
    assert!(!classify::is_arabicword(""));
}

#[test]
fn error_type_is_usable_from_outside_the_crate() {
    let err = Error::TashkeelLengthMismatch {
        letters: 3,
        marks: 2,
    };
    assert_eq!(
        err.to_string(),
        "tashkeel marks (2) do not match the number of letters (3)"
    );

    let boxed: Box<dyn std::error::Error> = Box::new(err);
    assert!(boxed.to_string().contains("tashkeel"));
    assert!(boxed.source().is_none());
}

#[test]
fn result_alias_accepts_the_crate_error() {
    fn fails() -> r_abic::Result<u8> {
        Err(Error::EmptyInput)
    }

    assert!(fails().is_err());
}

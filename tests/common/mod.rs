// SPDX-License-Identifier: MIT OR Apache-2.0
//! Shared helpers for the integration and property tests.
//!
//! The generator is a deterministic xorshift64: a failure can be pasted back into a unit test and it
//! reproduces everywhere, because there is no OS randomness, no clock and no dependency
//! (`AGENTS.md` §4.5).

#![allow(dead_code)] // each integration-test binary uses a different subset of these helpers

use r_abic::unicode::constants::{
    ALEF, ALEF_HAMZA_ABOVE, ALEF_MAKSURA, ALEF_WASLA, ARABIC_LETTER_MARK, BEH, BYTE_ORDER_MARK,
    DAMMA, DAMMATAN, FATHA, FATHATAN, FIVE, HAMZA, HAMZA_ABOVE, HAMZA_BELOW, KASRA, KASRATAN,
    LAM_ALEF, LAM_ALEF_HAMZA_ABOVE, LAM_ALEF_HAMZA_BELOW, LAM_ALEF_MADDA_ABOVE, LEFT_TO_RIGHT_MARK,
    MADDA_ABOVE, MEEM, NINE_P, RIGHT_TO_LEFT_MARK, SEEN, SHADDA, SMALL_ALEF, SMALL_WAW, SMALL_YEH,
    SUKUN, TATWEEL, TEH, TEH_MARBUTA, YEH, ZERO, ZERO_P, ZERO_WIDTH_JOINER, ZERO_WIDTH_NON_JOINER,
};

/// A deterministic pseudo-random generator (xorshift64), seeded by the test.
///
/// Only the tests use it; it is never part of the library, which must stay free of dependencies and
/// of any global state (`P1`, `P6`).
#[derive(Debug, Clone)]
pub struct Rng(u64);

impl Rng {
    /// Creates a generator. A zero seed is mapped to a non-zero state, which xorshift requires.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(if seed == 0 {
            0x9E37_79B9_7F4A_7C15
        } else {
            seed
        })
    }

    /// Returns the next value of the sequence.
    pub fn next_u64(&mut self) -> u64 {
        let mut state = self.0;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        self.0 = state;
        state
    }

    /// Returns a value in `0..upper` (`0` when `upper` is `0`).
    pub fn below(&mut self, upper: usize) -> usize {
        let bound = u64::try_from(upper).unwrap_or(u64::MAX).max(1);
        usize::try_from(self.next_u64() % bound).unwrap_or(0)
    }

    /// Picks one character of `alphabet` (a space when `alphabet` is empty).
    pub fn pick(&mut self, alphabet: &[char]) -> char {
        *alphabet.get(self.below(alphabet.len())).unwrap_or(&' ')
    }

    /// Builds a string of at most `max_len` characters drawn from `alphabet`.
    pub fn text(&mut self, alphabet: &[char], max_len: usize) -> String {
        let len = self.below(max_len + 1);
        let mut out = String::with_capacity(len * 3);
        for _ in 0..len {
            out.push(self.pick(alphabet));
        }
        out
    }
}

/// The curated alphabet of `AGENTS.md` §4.5: every class a transform must handle, including the
/// ones it must leave alone (tatweel, small letters, digits, invisible marks) and multi-byte
/// non-Arabic text (one CJK character, one astral character).
///
/// Named `constants` are used wherever one exists so the class coverage is auditable at a glance;
/// escapes remain only for code points that have no constant — the four *final* lam-alef forms, the
/// two alef presentation forms, ASCII/CJK/emoji.
#[rustfmt::skip] // hand-grouped data table: rustfmt would put one character per line and hide the grouping
pub const ALPHABET: &[char] = &[
    // Base letters, including the carriers and the hamza/wasla forms.
    HAMZA, ALEF_HAMZA_ABOVE, ALEF, BEH, TEH_MARBUTA, TEH, SEEN, MEEM, ALEF_MAKSURA, YEH, ALEF_WASLA,
    // Marks: every ḥaraka plus shadda (`tables::TASHKEEL`).
    FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SHADDA, SUKUN,
    // Joiners and small/combining forms that diacritic-stripping must keep.
    TATWEEL, SMALL_ALEF, SMALL_WAW, SMALL_YEH, MADDA_ABOVE, HAMZA_ABOVE, HAMZA_BELOW,
    // Presentation forms and all eight lam-alef ligature code points, isolated form then final form.
    '\u{FE8D}', '\u{FE8E}',
    LAM_ALEF_MADDA_ABOVE, '\u{FEF6}', LAM_ALEF_HAMZA_ABOVE, '\u{FEF8}',
    LAM_ALEF_HAMZA_BELOW, '\u{FEFA}', LAM_ALEF, '\u{FEFC}',
    // Digits of the three sets.
    '0', '7', ZERO, FIVE, ZERO_P, NINE_P,
    // ASCII letters, punctuation, whitespace and both line terminators.
    'a', 'Z', '.', ',', '"', ' ', '\t', '\r', '\n',
    // Invisible formatting code points (E11–E13).
    ARABIC_LETTER_MARK, ZERO_WIDTH_NON_JOINER, ZERO_WIDTH_JOINER, LEFT_TO_RIGHT_MARK,
    RIGHT_TO_LEFT_MARK, BYTE_ORDER_MARK,
    // Multi-byte non-Arabic: one 3-byte and one 4-byte code point (E10).
    '\u{6F22}', '\u{1F389}',
];

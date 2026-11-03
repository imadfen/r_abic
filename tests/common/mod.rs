// SPDX-License-Identifier: MIT OR Apache-2.0
//! Shared helpers for the integration and property tests.
//!
//! The generator is a deterministic xorshift64: a failure can be pasted back into a unit test and it
//! reproduces everywhere, because there is no OS randomness, no clock and no dependency
//! (`AGENTS.md` §4.5).

#![allow(dead_code)] // each integration-test binary uses a different subset of these helpers

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
#[rustfmt::skip] // hand-grouped data table: rustfmt would put one character per line and hide the grouping
pub const ALPHABET: &[char] = &[
    // Base letters, including the carriers and the hamza/wasla forms.
    '\u{0621}', '\u{0623}', '\u{0627}', '\u{0628}', '\u{0629}', '\u{062A}', '\u{0633}', '\u{0645}',
    '\u{0649}', '\u{064A}', '\u{0671}',
    // Marks: every ḥaraka plus shadda.
    '\u{064B}', '\u{064C}', '\u{064D}', '\u{064E}', '\u{064F}', '\u{0650}', '\u{0651}', '\u{0652}',
    // Joiners and small/combining forms that diacritic-stripping must keep.
    '\u{0640}', '\u{0670}', '\u{06E5}', '\u{06E6}', '\u{0653}', '\u{0654}', '\u{0655}',
    // Presentation forms and all eight lam-alef ligature code points.
    '\u{FE8D}', '\u{FE8E}', '\u{FEF5}', '\u{FEF6}', '\u{FEF7}', '\u{FEF8}', '\u{FEF9}', '\u{FEFA}',
    '\u{FEFB}', '\u{FEFC}',
    // Digits of the three sets.
    '0', '7', '\u{0660}', '\u{0665}', '\u{06F0}', '\u{06F9}',
    // ASCII letters, punctuation, whitespace and both line terminators.
    'a', 'Z', '.', ',', '"', ' ', '\t', '\r', '\n',
    // Invisible formatting code points (E11–E13).
    '\u{061C}', '\u{200C}', '\u{200D}', '\u{200E}', '\u{200F}', '\u{FEFF}',
    // Multi-byte non-Arabic: one 3-byte and one 4-byte code point (E10).
    '\u{6F22}', '\u{1F389}',
];

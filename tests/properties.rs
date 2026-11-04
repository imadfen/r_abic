// SPDX-License-Identifier: MIT OR Apache-2.0
//! Property tests for the text transforms (`AGENTS.md` §4.5).
//!
//! Inputs come from a fixed seed, so a failure prints a counterexample that reproduces on every
//! machine and can be pasted straight into a unit test.

mod common;

use std::borrow::Cow;

use common::{ALPHABET, Rng};
use r_abic::normalize::{strip_harakat, strip_harakat_into, strip_tashkeel, strip_tashkeel_into};
use r_abic::unicode::classify::{is_haraka, is_shadda, is_tashkeel};

/// One row of [`STRIPPERS`]: a name for the failure message, the transform, and the predicate that
/// says which characters it removes.
type Stripper = (
    &'static str,
    for<'a> fn(&'a str) -> Cow<'a, str>,
    fn(char) -> bool,
);

/// Every member of the `normalize` strip family. The invariants below are written once and apply to
/// all of them, so landing a new `strip_*` means adding one row here.
const STRIPPERS: &[Stripper] = &[
    ("strip_tashkeel", strip_tashkeel, is_tashkeel),
    ("strip_harakat", strip_harakat, is_haraka),
];

/// Samples per property: enough to hit every class combination of the curated alphabet while keeping
/// the whole file well under a second (no timing assertions anywhere — `AGENTS.md` §4.7).
const SAMPLES: usize = 4_000;

/// Seed of the generator; changing it invalidates every recorded counterexample.
const SEED: u64 = 0x5EED_0FAB_1C12_3456;

/// Maximum code points per generated sample.
const MAX_LEN: usize = 24;

fn samples() -> impl Iterator<Item = String> {
    let mut rng = Rng::new(SEED);
    (0..SAMPLES).map(move |_| rng.text(ALPHABET, MAX_LEN))
}

/// `true` when `needle` can be obtained from `haystack` by deletion only.
fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut rest = haystack.chars();
    needle.chars().all(|want| rest.any(|have| have == want))
}

#[test]
fn stripping_is_total_and_only_deletes_characters() {
    for &(name, strip, is_stripped) in STRIPPERS {
        for input in samples() {
            let output = strip(&input);

            assert!(
                output.len() <= input.len(),
                "{name}: counterexample {input:?} grew to {output:?}"
            );
            assert_eq!(
                output.chars().count(),
                input.chars().filter(|c| !is_stripped(*c)).count(),
                "{name}: counterexample {input:?} -> {output:?}"
            );
            assert!(
                is_subsequence(&output, &input),
                "{name}: counterexample {input:?} -> {output:?} is not a deletion"
            );
            assert!(
                output.chars().all(|c| !is_stripped(c)),
                "{name}: counterexample {input:?} -> {output:?} kept a removed character"
            );
        }
    }
}

#[test]
fn stripping_is_idempotent() {
    for &(name, strip, _) in STRIPPERS {
        for input in samples() {
            let once = strip(&input).into_owned();
            let twice = strip(&once);

            assert_eq!(once, twice, "{name}: counterexample {input:?}");
            assert!(
                matches!(twice, Cow::Borrowed(_)),
                "{name}: a stripped string must take the zero-copy path: {input:?}"
            );
        }
    }
}

#[test]
fn stripping_borrows_input_that_is_already_clean() {
    for &(name, strip, _) in STRIPPERS {
        for input in samples() {
            let clean = strip(&input).into_owned();

            assert!(
                matches!(strip(&clean), Cow::Borrowed(_)),
                "{name}: counterexample {clean:?}"
            );
        }
    }
}

#[test]
fn into_variants_agree_with_the_cow_variants() {
    let mut buffer = String::from("stale content that must be cleared");

    for input in samples() {
        strip_tashkeel_into(&input, &mut buffer);
        assert_eq!(
            buffer,
            strip_tashkeel(&input),
            "strip_tashkeel_into: {input:?}"
        );

        strip_harakat_into(&input, &mut buffer);
        assert_eq!(
            buffer,
            strip_harakat(&input),
            "strip_harakat_into: {input:?}"
        );
    }
}

#[test]
fn strip_harakat_keeps_the_shadda_that_strip_tashkeel_removes() {
    for input in samples() {
        let harakat = strip_harakat(&input).into_owned();
        let shaddas = input.chars().filter(|c| is_shadda(*c)).count();

        assert_eq!(
            harakat.chars().filter(|c| is_shadda(*c)).count(),
            shaddas,
            "strip_harakat must keep every shadda: {input:?}"
        );
        assert_eq!(
            strip_tashkeel(&input)
                .chars()
                .filter(|c| is_shadda(*c))
                .count(),
            0,
            "strip_tashkeel must remove every shadda: {input:?}"
        );
        // Removing the ḥarakāt first and everything afterwards equals removing everything at once.
        assert_eq!(
            strip_tashkeel(&harakat),
            strip_tashkeel(&input),
            "counterexample: {input:?}"
        );
    }
}

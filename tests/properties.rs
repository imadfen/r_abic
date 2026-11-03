// SPDX-License-Identifier: MIT OR Apache-2.0
//! Property tests for the text transforms (`AGENTS.md` §4.5).
//!
//! Inputs come from a fixed seed, so a failure prints a counterexample that reproduces on every
//! machine and can be pasted straight into a unit test.

mod common;

use std::borrow::Cow;

use common::{ALPHABET, Rng};
use r_abic::normalize::{strip_tashkeel, strip_tashkeel_into};
use r_abic::unicode::classify::is_tashkeel;

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
    for input in samples() {
        let output = strip_tashkeel(&input);

        assert!(
            output.len() <= input.len(),
            "counterexample: {input:?} grew to {output:?}"
        );
        assert_eq!(
            output.chars().count(),
            input.chars().filter(|c| !is_tashkeel(*c)).count(),
            "counterexample: {input:?} -> {output:?}"
        );
        assert!(
            is_subsequence(&output, &input),
            "counterexample: {input:?} -> {output:?} is not a deletion"
        );
        assert!(
            output.chars().all(|c| !is_tashkeel(c)),
            "counterexample: {input:?} -> {output:?} kept a mark"
        );
    }
}

#[test]
fn stripping_is_idempotent() {
    for input in samples() {
        let once = strip_tashkeel(&input).into_owned();
        let twice = strip_tashkeel(&once);

        assert_eq!(once, twice, "counterexample: {input:?}");
        assert!(
            matches!(twice, Cow::Borrowed(_)),
            "a stripped string must take the zero-copy path: {input:?}"
        );
    }
}

#[test]
fn stripping_borrows_input_that_is_already_clean() {
    for input in samples() {
        let clean = strip_tashkeel(&input).into_owned();

        assert!(
            matches!(strip_tashkeel(&clean), Cow::Borrowed(_)),
            "counterexample: {clean:?}"
        );
    }
}

#[test]
fn into_variant_agrees_with_the_cow_variant() {
    let mut buffer = String::from("stale content that must be cleared");

    for input in samples() {
        strip_tashkeel_into(&input, &mut buffer);

        assert_eq!(buffer, strip_tashkeel(&input), "counterexample: {input:?}");
    }
}

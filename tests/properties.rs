// SPDX-License-Identifier: MIT OR Apache-2.0
//! Property tests for the text transforms (`AGENTS.md` §4.5).
//!
//! Inputs come from a fixed seed, so a failure prints a counterexample that reproduces on every
//! machine and can be pasted straight into a unit test.

mod common;

use std::borrow::Cow;

use common::{ALPHABET, Rng};
use r_abic::normalize::{
    strip_harakat, strip_harakat_into, strip_lastharaka, strip_lastharaka_into, strip_tashkeel,
    strip_tashkeel_into,
};
use r_abic::unicode::classify::{is_haraka, is_shadda, is_tanwin, is_tashkeel};

/// One row of [`STRIPPERS`]: a name for the failure message, the transform, the predicate that says
/// which characters it removes (absent when the rule depends on position rather than on the character
/// alone), and whether the contract says that calling it twice equals calling it once.
type Stripper = (
    &'static str,
    for<'a> fn(&'a str) -> Cow<'a, str>,
    Option<fn(char) -> bool>,
    bool,
);

/// Every member of the `normalize` strip family. The invariants below are written once and apply to
/// all of them, so landing a new `strip_*` means adding one row here.
const STRIPPERS: &[Stripper] = &[
    ("strip_tashkeel", strip_tashkeel, Some(is_tashkeel), true),
    ("strip_harakat", strip_harakat, Some(is_haraka), true),
    // `strip_lastharaka` removes every tanwīn plus at most ONE trailing ḥaraka, so it has no
    // character-wise predicate and is deliberately not idempotent: two trailing ḥarakāt need two
    // calls (upstream's `[HARAKAT]$|[TANWIN]` behaves the same). Its own invariants are below.
    ("strip_lastharaka", strip_lastharaka, None, false),
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
    for &(name, strip, char_wise, _) in STRIPPERS {
        for input in samples() {
            let output = strip(&input);

            assert!(
                output.len() <= input.len(),
                "{name}: counterexample {input:?} grew to {output:?}"
            );
            assert!(
                is_subsequence(&output, &input),
                "{name}: counterexample {input:?} -> {output:?} is not a deletion"
            );

            if let Some(is_stripped) = char_wise {
                assert_eq!(
                    output.chars().count(),
                    input.chars().filter(|c| !is_stripped(*c)).count(),
                    "{name}: counterexample {input:?} -> {output:?}"
                );
                assert!(
                    output.chars().all(|c| !is_stripped(c)),
                    "{name}: counterexample {input:?} -> {output:?} kept a removed character"
                );
            }
        }
    }
}

#[test]
fn stripping_is_idempotent_where_the_contract_says_so() {
    for &(name, strip, _, idempotent) in STRIPPERS {
        if !idempotent {
            continue;
        }

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
    for &(name, strip, _, idempotent) in STRIPPERS {
        // For a non-idempotent member (only `strip_lastharaka` today) one call is not enough to reach
        // the fixed point, so its own output may still have something to remove; its borrowed path is
        // covered by the test below instead.
        if !idempotent {
            continue;
        }

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
fn strip_lastharaka_borrows_input_with_nothing_to_remove() {
    for input in samples() {
        // Drop every tanwīn and force a non-ḥaraka last character: nothing can match the pattern, so
        // the call must take the zero-copy path.
        let clean: String = input
            .chars()
            .filter(|c| !is_tanwin(*c))
            .chain(std::iter::once('\u{0643}'))
            .collect();

        assert!(
            matches!(strip_lastharaka(&clean), Cow::Borrowed(_)),
            "counterexample: {clean:?}"
        );
    }
}

#[test]
fn strip_lastharaka_removes_every_tanwin_and_at_most_one_haraka() {
    for input in samples() {
        let output = strip_lastharaka(&input).into_owned();

        assert!(
            !output.chars().any(is_tanwin),
            "every tanwīn must go: {input:?}"
        );
        assert_eq!(
            output.chars().filter(|c| is_shadda(*c)).count(),
            input.chars().filter(|c| is_shadda(*c)).count(),
            "shadda is never touched: {input:?}"
        );

        let removed_harakat = input.chars().filter(|c| is_haraka(*c)).count()
            - output.chars().filter(|c| is_haraka(*c)).count();
        let removed_tanwin = input.chars().filter(|c| is_tanwin(*c)).count();

        assert!(
            removed_harakat <= removed_tanwin + 1,
            "at most one non-tanwīn ḥaraka is removed per call: {input:?}"
        );
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

        strip_lastharaka_into(&input, &mut buffer);
        assert_eq!(
            buffer,
            strip_lastharaka(&input),
            "strip_lastharaka_into: {input:?}"
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

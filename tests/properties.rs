// SPDX-License-Identifier: MIT OR Apache-2.0
//! Property tests for the text transforms (`AGENTS.md` §4.5).
//!
//! Inputs come from a fixed seed, so a failure prints a counterexample that reproduces on every
//! machine and can be pasted straight into a unit test.

mod common;

use std::borrow::Cow;

use common::{ALPHABET, Rng};
use r_abic::normalize::{
    normalize_lamalef, normalize_lamalef_into, normalize_ligature, normalize_ligature_into,
    strip_diacritics, strip_diacritics_into, strip_harakat, strip_harakat_into, strip_lastharaka,
    strip_lastharaka_into, strip_shadda, strip_shadda_into, strip_small, strip_small_into,
    strip_tashkeel, strip_tashkeel_into, strip_tatweel, strip_tatweel_into,
};
use r_abic::unicode::classify::{
    is_haraka, is_ligature, is_shadda, is_small, is_tanwin, is_tashkeel, is_tatweel,
};

/// One row of [`STRIPPERS`]: a name for the failure message, the transform, the predicate that says
/// which characters it removes (absent when the rule depends on position rather than on the character
/// alone), and whether the contract says that calling it twice equals calling it once.
type Stripper = (
    &'static str,
    for<'a> fn(&'a str) -> Cow<'a, str>,
    Option<fn(char) -> bool>,
    bool,
);

/// Mirrors `normalize::strip_diacritics`'s private rule: nonspacing marks (`Mn`) in `U+0600–U+06FE`.
///
/// Duplicated here on purpose: the implementation keeps the range table private (upstream has no
/// `is_diacritic` predicate), so the property suite carries its own oracle and any drift between the
/// two shows up as a failing assertion.
fn is_diacritic(c: char) -> bool {
    matches!(
        c,
        '\u{0610}'..='\u{061A}'
            | '\u{064B}'..='\u{065F}'
            | '\u{0670}'
            | '\u{06D6}'..='\u{06DC}'
            | '\u{06DF}'..='\u{06E4}'
            | '\u{06E7}'..='\u{06E8}'
            | '\u{06EA}'..='\u{06ED}'
    )
}

/// Every member of the `normalize` strip family. The invariants below are written once and apply to
/// all of them, so landing a new `strip_*` means adding one row here.
const STRIPPERS: &[Stripper] = &[
    ("strip_tashkeel", strip_tashkeel, Some(is_tashkeel), true),
    ("strip_harakat", strip_harakat, Some(is_haraka), true),
    // `strip_lastharaka` removes every tanwīn plus at most ONE trailing ḥaraka, so it has no
    // character-wise predicate and is deliberately not idempotent: two trailing ḥarakāt need two
    // calls (upstream's `[HARAKAT]$|[TANWIN]` behaves the same). Its own invariants are below.
    ("strip_lastharaka", strip_lastharaka, None, false),
    ("strip_tatweel", strip_tatweel, Some(is_tatweel), true),
    ("strip_shadda", strip_shadda, Some(is_shadda), true),
    ("strip_small", strip_small, Some(is_small), true),
    (
        "strip_diacritics",
        strip_diacritics,
        Some(is_diacritic),
        true,
    ),
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

        strip_tatweel_into(&input, &mut buffer);
        assert_eq!(
            buffer,
            strip_tatweel(&input),
            "strip_tatweel_into: {input:?}"
        );

        strip_shadda_into(&input, &mut buffer);
        assert_eq!(buffer, strip_shadda(&input), "strip_shadda_into: {input:?}");

        strip_small_into(&input, &mut buffer);
        assert_eq!(buffer, strip_small(&input), "strip_small_into: {input:?}");

        strip_diacritics_into(&input, &mut buffer);
        assert_eq!(
            buffer,
            strip_diacritics(&input),
            "strip_diacritics_into: {input:?}"
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

#[test]
fn normalize_ligature_expands_every_form_and_is_idempotent() {
    let mut buffer = String::from("stale content that must be cleared");

    for input in samples() {
        let output = normalize_ligature(&input).into_owned();

        // Expansion only ever adds characters (one per ligature) and leaves none behind.
        assert!(
            output.chars().count() >= input.chars().count(),
            "counterexample: {input:?} -> {output:?}"
        );
        assert!(
            !output.chars().any(is_ligature),
            "counterexample: {input:?} -> {output:?}"
        );
        assert_eq!(
            normalize_ligature(&output).into_owned(),
            output,
            "idempotent: {input:?}"
        );

        // The alias and the `_into` variant agree with the primary name.
        assert_eq!(normalize_lamalef(&input).into_owned(), output);
        normalize_ligature_into(&input, &mut buffer);
        assert_eq!(buffer, output, "normalize_ligature_into: {input:?}");
        normalize_lamalef_into(&input, &mut buffer);
        assert_eq!(buffer, output, "normalize_lamalef_into: {input:?}");
    }
}

#[test]
fn normalize_ligature_borrows_input_that_has_no_ligature() {
    for input in samples() {
        let clean: String = input.chars().filter(|c| !is_ligature(*c)).collect();

        assert!(
            matches!(normalize_ligature(&clean), Cow::Borrowed(_)),
            "counterexample: {clean:?}"
        );
    }
}

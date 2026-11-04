// SPDX-License-Identifier: MIT OR Apache-2.0
//! Benchmarks for the `normalize` transforms (`AGENTS.md` §4.7).
//!
//! Harness: **Criterion** (`harness = false` in `Cargo.toml`). Run locally with `cargo bench`; CI only
//! *compiles* benches, because timings on shared runners are noise. Every input is built once, outside
//! the timed closure, and read through `std::hint::black_box`.

use std::borrow::Cow;
use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use r_abic::normalize::{
    strip_diacritics, strip_harakat, strip_lastharaka, strip_shadda, strip_small, strip_tashkeel,
    strip_tatweel,
};
use r_abic::unicode::constants::{SMALL_ALEF, TATWEEL};

/// A fully vocalized sentence: the realistic worst case, where nearly every letter carries a mark.
const VOCALIZED: &str = "الْعَرَبِيَّةُ لُغَةٌ جَمِيلَةٌ وَاسِعَةُ الِانْتِشَارِ";

/// The same text already stripped: this must take the zero-copy path.
const PLAIN: &str = "العربية لغة جميلة واسعة الانتشار";

/// Repeats `unit` until the result is at least `bytes` bytes long.
fn fill(unit: &str, bytes: usize) -> String {
    unit.repeat(bytes / unit.len() + 1)
}

/// The four sizes every transform is measured at, built from a `unit` (text the transform must
/// rewrite) and a `clean` text (already in the target form, to measure the zero-copy path).
fn sizes(unit: &str, clean: &str) -> [(&'static str, String); 4] {
    [
        ("tiny", unit.to_string()),
        ("paragraph/4KiB", fill(unit, 4 * 1024)),
        ("prose/1MiB", fill(unit, 1 << 20)),
        ("prose/1MiB-clean", fill(clean, 1 << 20)),
    ]
}

/// Interleaves `filler` after every character of `unit`: the worst case for a transform that removes
/// `filler`, when the shared [`VOCALIZED`] unit does not contain it and the bench would otherwise
/// only ever exercise the zero-copy path.
fn interleave(unit: &str, filler: char) -> String {
    let mut out = String::with_capacity(unit.len() * 2);
    for ch in unit.chars() {
        out.push(ch);
        out.push(filler);
    }
    out
}

/// Sweeps one transform over [`inputs()`]. Sharing the sweep keeps every `strip_*` measured the same
/// way, and adding the next one costs three lines.
///
/// The transform arrives as a generic parameter rather than a `fn` pointer on purpose: a pointer call
/// cannot be inlined into the timed closure, and that indirection alone shows up as a double-digit
/// "regression" — a benchmark must not measure its own scaffolding (`AGENTS.md` §4.7, P10).
fn sweep<F>(c: &mut Criterion, group_name: &'static str, strip: F)
where
    F: for<'a> Fn(&'a str) -> Cow<'a, str>,
{
    sweep_sizes(c, group_name, strip, &sizes(VOCALIZED, PLAIN));
}

/// Like [`sweep`], but over explicit inputs — for a transform whose interesting unit is not
/// `VOCALIZED` (e.g. `strip_tatweel`, which removes kashidas).
fn sweep_sizes<F>(
    c: &mut Criterion,
    group_name: &'static str,
    strip: F,
    inputs: &[(&'static str, String)],
) where
    F: for<'a> Fn(&'a str) -> Cow<'a, str>,
{
    let mut group = c.benchmark_group(group_name);

    for (name, input) in inputs {
        let bytes = u64::try_from(input.len()).unwrap_or(u64::MAX);
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            input.as_str(),
            |b, text| {
                b.iter(|| strip(black_box(text)));
            },
        );
    }

    group.finish();
}

fn bench_strip_tashkeel(c: &mut Criterion) {
    sweep(c, "strip_tashkeel", strip_tashkeel);
}

fn bench_strip_harakat(c: &mut Criterion) {
    sweep(c, "strip_harakat", strip_harakat);
}

fn bench_strip_lastharaka(c: &mut Criterion) {
    sweep(c, "strip_lastharaka", strip_lastharaka);
}

fn bench_strip_tatweel(c: &mut Criterion) {
    let stretched = interleave(VOCALIZED, TATWEEL);
    sweep_sizes(c, "strip_tatweel", strip_tatweel, &sizes(&stretched, PLAIN));
}

fn bench_strip_shadda(c: &mut Criterion) {
    sweep(c, "strip_shadda", strip_shadda);
}

fn bench_strip_small(c: &mut Criterion) {
    let dotted = interleave(VOCALIZED, SMALL_ALEF);
    sweep_sizes(c, "strip_small", strip_small, &sizes(&dotted, PLAIN));
}

fn bench_strip_diacritics(c: &mut Criterion) {
    sweep(c, "strip_diacritics", strip_diacritics);
}

criterion_group!(
    benches,
    bench_strip_tashkeel,
    bench_strip_harakat,
    bench_strip_lastharaka,
    bench_strip_tatweel,
    bench_strip_shadda,
    bench_strip_small,
    bench_strip_diacritics
);
criterion_main!(benches);

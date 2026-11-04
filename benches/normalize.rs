// SPDX-License-Identifier: MIT OR Apache-2.0
//! Benchmarks for the `normalize` transforms (`AGENTS.md` §4.7).
//!
//! Harness: **Criterion** (`harness = false` in `Cargo.toml`). Run locally with `cargo bench`; CI only
//! *compiles* benches, because timings on shared runners are noise. Every input is built once, outside
//! the timed closure, and read through `std::hint::black_box`.

use std::borrow::Cow;
use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use r_abic::normalize::{strip_harakat, strip_lastharaka, strip_tashkeel};

/// A fully vocalized sentence: the realistic worst case, where nearly every letter carries a mark.
const VOCALIZED: &str = "الْعَرَبِيَّةُ لُغَةٌ جَمِيلَةٌ وَاسِعَةُ الِانْتِشَارِ";

/// The same text already stripped: this must take the zero-copy path.
const PLAIN: &str = "العربية لغة جميلة واسعة الانتشار";

/// Repeats `unit` until the result is at least `bytes` bytes long.
fn fill(unit: &str, bytes: usize) -> String {
    unit.repeat(bytes / unit.len() + 1)
}

/// The inputs every transform is measured on: one vocalized sentence, a realistic paragraph, 1 MiB of
/// prose, and the same 1 MiB already clean (the zero-copy path).
fn inputs() -> [(&'static str, String); 4] {
    [
        ("tiny", VOCALIZED.to_string()),
        ("paragraph/4KiB", fill(VOCALIZED, 4 * 1024)),
        ("prose/1MiB", fill(VOCALIZED, 1 << 20)),
        ("prose/1MiB-clean", fill(PLAIN, 1 << 20)),
    ]
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
    let mut group = c.benchmark_group(group_name);

    for (name, input) in inputs() {
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

criterion_group!(
    benches,
    bench_strip_tashkeel,
    bench_strip_harakat,
    bench_strip_lastharaka
);
criterion_main!(benches);

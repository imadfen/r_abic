// SPDX-License-Identifier: MIT OR Apache-2.0
//! Benchmarks for the `normalize` transforms (`AGENTS.md` §4.7).
//!
//! Harness: **Criterion** (`harness = false` in `Cargo.toml`). Run locally with `cargo bench`; CI only
//! *compiles* benches, because timings on shared runners are noise. Every input is built once, outside
//! the timed closure, and read through `std::hint::black_box`.

use std::hint::black_box;

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};

use r_abic::normalize::strip_tashkeel;

/// A fully vocalized sentence: the realistic worst case, where nearly every letter carries a mark.
const VOCALIZED: &str = "الْعَرَبِيَّةُ لُغَةٌ جَمِيلَةٌ وَاسِعَةُ الِانْتِشَارِ";

/// The same text already stripped: this must take the zero-copy path.
const PLAIN: &str = "العربية لغة جميلة واسعة الانتشار";

/// Repeats `unit` until the result is at least `bytes` bytes long.
fn fill(unit: &str, bytes: usize) -> String {
    unit.repeat(bytes / unit.len() + 1)
}

fn bench_strip_tashkeel(c: &mut Criterion) {
    let tiny = VOCALIZED;
    let paragraph = fill(VOCALIZED, 4 * 1024);
    let prose = fill(VOCALIZED, 1 << 20);
    let prose_clean = fill(PLAIN, 1 << 20);

    let mut group = c.benchmark_group("strip_tashkeel");
    let cases = [
        ("tiny", tiny),
        ("paragraph/4KiB", paragraph.as_str()),
        ("prose/1MiB", prose.as_str()),
        ("prose/1MiB-clean", prose_clean.as_str()),
    ];

    for (name, input) in cases {
        let bytes = u64::try_from(input.len()).unwrap_or(u64::MAX);
        group.throughput(Throughput::Bytes(bytes));
        group.bench_with_input(BenchmarkId::from_parameter(name), input, |b, text| {
            b.iter(|| strip_tashkeel(black_box(text)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_strip_tashkeel);
criterion_main!(benches);

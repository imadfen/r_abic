// SPDX-License-Identifier: MIT OR Apache-2.0
//! # `r_abic` — Arabic text processing for Rust
//!
//! A high-performance, memory-efficient, dependency-free library for Arabic text processing:
//! character classification, normalization, diacritic (`tashkeel`) handling, tokenization,
//! unshaping, transliteration and Arabic number-to-words.
//!
//! `r_abic` is an independent Rust implementation whose documented behaviour is a superset of
//! [PyArabic](https://github.com/linuxscout/pyarabic)'s API. It is **not** a port of `PyArabic`'s
//! source: see `AGENTS.md` in the repository root for the parity, provenance and licensing policy.
//!
//! ## Status
//!
//! Phase 0 — the crate provides the error type and the [`unicode`] module (code points, character
//! classes, classification predicates). Text transforms, tokenization, unshaping, transliteration
//! and numbers arrive in later phases (see `AGENTS.md` §6).
//!
//! ## Examples
//!
//! ```
//! use r_abic::unicode::{classify, constants, tables};
//!
//! // Code points are exposed under their PyArabic names:
//! assert_eq!(constants::ALEF, '\u{0627}');
//! assert_eq!(constants::SHADDA, '\u{0651}');
//!
//! // Classes are `const` slices, predicates are `const fn`s:
//! assert!(tables::HARAKAT.contains(&constants::SUKUN));
//! assert!(classify::is_haraka(constants::FATHA));
//! assert!(!classify::is_haraka(constants::SHADDA));
//!
//! // Whole-word helpers work on `&str`:
//! assert!(classify::has_shadda("الشَّمس"));
//! assert_eq!(tables::alphabetic_order(constants::ALEF), Some(1));
//! ```
//!
//! ## Design constraints
//!
//! Guaranteed by the build and enforced by tests: no `unsafe`, no runtime dependencies, no panics on
//! user input, and no assumption that an Arabic character is one byte (see `AGENTS.md` §1.2).

#![forbid(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations, rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::dbg_macro,
    clippy::print_stdout
)]
// Tests assert freely; the deny list above targets library code only (`AGENTS.md` §3.7).
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
// Public names deliberately mirror the upstream API (`normalize::normalize_searchtext`, ...).
#![allow(clippy::module_name_repetitions)]

pub mod error;
pub mod unicode;

pub use crate::error::{Error, Result};

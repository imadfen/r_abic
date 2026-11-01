// SPDX-License-Identifier: MIT OR Apache-2.0
//! Arabic code points, character classes and classification predicates.
//!
//! * [`constants`] — every code point we handle, under its `PyArabic` name.
//! * [`tables`] — the upstream class groups (`HARAKAT`, `TASHKEEL`, `LIGUATURES`, `MOON`, …).
//! * [`classify`] — `const fn` predicates (`is_haraka`, `is_shadda`, `is_arabic_letter`, …).
//!
//! ```
//! use r_abic::unicode::{classify, constants, tables};
//!
//! assert!(classify::is_arabic_letter(constants::BEH));
//! assert!(!classify::is_arabic_letter(constants::TATWEEL));
//! assert_eq!(tables::MOON.len(), 18);
//! ```

pub mod classify;
pub mod constants;
pub mod tables;

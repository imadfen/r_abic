// SPDX-License-Identifier: MIT OR Apache-2.0
//! The crate's single error type.
//!
//! `r_abic` deliberately uses one small, hand-written error enum instead of a `thiserror` derive: the
//! error surface is tiny and the crate ships **zero dependencies** (`CLAUDE.md` §2.2, §3.4).
//!
//! The enum is `#[non_exhaustive]`, so new variants may be added in minor releases. Match it with a
//! `_ =>` arm:
//!
//! ```
//! use r_abic::Error;
//!
//! let err = Error::EmptyInput;
//! let message = match err {
//!     Error::EmptyInput => "empty",
//!     _ => "other",
//! };
//! assert_eq!(message, "empty");
//! ```

use core::fmt;

/// Errors returned by `r_abic` functions.
///
/// Every variant carries enough context to be actionable, and `Display` renders a sentence suitable
/// for logs or user-facing diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The function is undefined for empty input (for example `soundex("")`).
    ///
    /// Transforms such as `strip_*` never return this: empty input is a valid input there and is
    /// returned unchanged (`CLAUDE.md` §3.4).
    EmptyInput,
    /// Encoded tashkeel could not be decoded.
    ///
    /// Raised by `decode_tashkeel` (Phase 2) when the encoded marks are neither a decimal number nor
    /// a valid encoded-mark string.
    InvalidTashkeelEncoding,
    /// The number of tashkeel marks cannot correspond to the number of letters.
    ///
    /// Raised by `joint`/`decode_tashkeel` (Phase 2) when the two sequences are not aligned.
    TashkeelLengthMismatch {
        /// Number of letters in the `(letters, marks)` pair.
        letters: usize,
        /// Number of marks in the `(letters, marks)` pair.
        marks: usize,
    },
    /// The value handed to an Arabic number-to-words function is outside the supported domain.
    ///
    /// Raised by the `numbers` module (Phase 3) for negative or excessively large values.
    NumberOutOfRange {
        /// The rejected value.
        value: i64,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => {
                f.write_str("input is empty, but this operation is undefined for empty input")
            }
            Self::InvalidTashkeelEncoding => f.write_str(
                "encoded tashkeel is malformed: expected decimal digits or the ASCII mark alphabet",
            ),
            Self::TashkeelLengthMismatch { letters, marks } => write!(
                f,
                "tashkeel marks ({marks}) do not match the number of letters ({letters})"
            ),
            Self::NumberOutOfRange { value } => write!(
                f,
                "number {value} is outside the supported range for Arabic number-to-words"
            ),
        }
    }
}

impl std::error::Error for Error {}

/// Convenience alias for results produced by this crate.
///
/// ```
/// use r_abic::{Error, Result};
///
/// fn always_empty() -> Result<()> {
///     Err(Error::EmptyInput)
/// }
///
/// assert!(always_empty().is_err());
/// ```
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    fn display_messages_name_the_offending_values() {
        assert_eq!(
            Error::EmptyInput.to_string(),
            "input is empty, but this operation is undefined for empty input"
        );
        assert!(
            Error::InvalidTashkeelEncoding
                .to_string()
                .contains("decimal")
        );
        assert_eq!(
            Error::TashkeelLengthMismatch {
                letters: 3,
                marks: 2
            }
            .to_string(),
            "tashkeel marks (2) do not match the number of letters (3)"
        );
        assert_eq!(
            Error::NumberOutOfRange { value: -5 }.to_string(),
            "number -5 is outside the supported range for Arabic number-to-words"
        );
    }

    #[test]
    fn errors_are_clone_eq_and_debug() {
        let err = Error::TashkeelLengthMismatch {
            letters: 1,
            marks: 0,
        };
        let copy = err.clone();
        assert_eq!(err, copy);
        assert!(format!("{err:?}").contains("TashkeelLengthMismatch"));
    }

    #[test]
    fn source_is_none_because_no_variant_wraps_another_error() {
        let err = Error::EmptyInput;
        assert!(std::error::Error::source(&err).is_none());
    }
}

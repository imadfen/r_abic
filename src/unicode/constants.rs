// SPDX-License-Identifier: MIT OR Apache-2.0
//! Arabic code points as `const char` values.
//!
//! Names mirror `pyarabic.araby` / `pyarabic.araby_const` one-to-one so that parity checks stay
//! greppable (`CLAUDE.md` §3.2); the values are the Unicode code points listed in `CLAUDE.md`
//! Appendix A. Nothing in this module panics or allocates: it is pure data.
//!
//! ```
//! use r_abic::unicode::constants::{ALEF, SHADDA, TATWEEL};
//!
//! assert_eq!(u32::from(ALEF), 0x0627);
//! assert_eq!(u32::from(SHADDA), 0x0651);
//! assert_eq!(u32::from(TATWEEL), 0x0640);
//! ```

// ---------------------------------------------------------------- punctuation

/// `COMMA` — `U+060C` ARABIC COMMA (`،`).
pub const COMMA: char = '\u{060C}';
/// `SEMICOLON` — `U+061B` ARABIC SEMICOLON (`؛`).
pub const SEMICOLON: char = '\u{061B}';
/// `QUESTION` — `U+061F` ARABIC QUESTION MARK (`؟`).
pub const QUESTION: char = '\u{061F}';

// ------------------------------------------------------------------- letters

/// `HAMZA` — `U+0621` ARABIC LETTER HAMZA (`ء`).
pub const HAMZA: char = '\u{0621}';
/// `ALEF_MADDA` — `U+0622` ARABIC LETTER ALEF WITH MADDA ABOVE (`آ`).
pub const ALEF_MADDA: char = '\u{0622}';
/// `ALEF_HAMZA_ABOVE` — `U+0623` ARABIC LETTER ALEF WITH HAMZA ABOVE (`أ`).
pub const ALEF_HAMZA_ABOVE: char = '\u{0623}';
/// `WAW_HAMZA` — `U+0624` ARABIC LETTER WAW WITH HAMZA ABOVE (`ؤ`).
pub const WAW_HAMZA: char = '\u{0624}';
/// `ALEF_HAMZA_BELOW` — `U+0625` ARABIC LETTER ALEF WITH HAMZA BELOW (`إ`).
pub const ALEF_HAMZA_BELOW: char = '\u{0625}';
/// `YEH_HAMZA` — `U+0626` ARABIC LETTER YEH WITH HAMZA ABOVE (`ئ`).
pub const YEH_HAMZA: char = '\u{0626}';
/// `ALEF` — `U+0627` ARABIC LETTER ALEF (`ا`).
pub const ALEF: char = '\u{0627}';
/// `BEH` — `U+0628` ARABIC LETTER BEH (`ب`).
pub const BEH: char = '\u{0628}';
/// `TEH_MARBUTA` — `U+0629` ARABIC LETTER TEH MARBUTA (`ة`).
pub const TEH_MARBUTA: char = '\u{0629}';
/// `TEH` — `U+062A` ARABIC LETTER TEH (`ت`).
pub const TEH: char = '\u{062A}';
/// `THEH` — `U+062B` ARABIC LETTER THEH (`ث`).
pub const THEH: char = '\u{062B}';
/// `JEEM` — `U+062C` ARABIC LETTER JEEM (`ج`).
pub const JEEM: char = '\u{062C}';
/// `HAH` — `U+062D` ARABIC LETTER HAH (`ح`).
pub const HAH: char = '\u{062D}';
/// `KHAH` — `U+062E` ARABIC LETTER KHAH (`خ`).
pub const KHAH: char = '\u{062E}';
/// `DAL` — `U+062F` ARABIC LETTER DAL (`د`).
pub const DAL: char = '\u{062F}';
/// `THAL` — `U+0630` ARABIC LETTER THAL (`ذ`).
pub const THAL: char = '\u{0630}';
/// `REH` — `U+0631` ARABIC LETTER REH (`ر`).
pub const REH: char = '\u{0631}';
/// `ZAIN` — `U+0632` ARABIC LETTER ZAIN (`ز`).
pub const ZAIN: char = '\u{0632}';
/// `SEEN` — `U+0633` ARABIC LETTER SEEN (`س`).
pub const SEEN: char = '\u{0633}';
/// `SHEEN` — `U+0634` ARABIC LETTER SHEEN (`ش`).
pub const SHEEN: char = '\u{0634}';
/// `SAD` — `U+0635` ARABIC LETTER SAD (`ص`).
pub const SAD: char = '\u{0635}';
/// `DAD` — `U+0636` ARABIC LETTER DAD (`ض`).
pub const DAD: char = '\u{0636}';
/// `TAH` — `U+0637` ARABIC LETTER TAH (`ط`).
pub const TAH: char = '\u{0637}';
/// `ZAH` — `U+0638` ARABIC LETTER ZAH (`ظ`).
pub const ZAH: char = '\u{0638}';
/// `AIN` — `U+0639` ARABIC LETTER AIN (`ع`).
pub const AIN: char = '\u{0639}';
/// `GHAIN` — `U+063A` ARABIC LETTER GHAIN (`غ`).
pub const GHAIN: char = '\u{063A}';
/// `TATWEEL` — `U+0640` ARABIC TATWEEL (kashida, `ـ`); a joiner, never a letter and never a ḥaraka.
pub const TATWEEL: char = '\u{0640}';
/// `FEH` — `U+0641` ARABIC LETTER FEH (`ف`).
pub const FEH: char = '\u{0641}';
/// `QAF` — `U+0642` ARABIC LETTER QAF (`ق`).
pub const QAF: char = '\u{0642}';
/// `KAF` — `U+0643` ARABIC LETTER KAF (`ك`).
pub const KAF: char = '\u{0643}';
/// `LAM` — `U+0644` ARABIC LETTER LAM (`ل`).
pub const LAM: char = '\u{0644}';
/// `MEEM` — `U+0645` ARABIC LETTER MEEM (`م`).
pub const MEEM: char = '\u{0645}';
/// `NOON` — `U+0646` ARABIC LETTER NOON (`ن`).
pub const NOON: char = '\u{0646}';
/// `HEH` — `U+0647` ARABIC LETTER HEH (`ه`).
pub const HEH: char = '\u{0647}';
/// `WAW` — `U+0648` ARABIC LETTER WAW (`و`).
pub const WAW: char = '\u{0648}';
/// `ALEF_MAKSURA` — `U+0649` ARABIC LETTER ALEF MAKSURA (`ى`).
pub const ALEF_MAKSURA: char = '\u{0649}';
/// `YEH` — `U+064A` ARABIC LETTER YEH (`ي`).
pub const YEH: char = '\u{064A}';

// --------------------------------------------------------------------- marks

/// `FATHATAN` — `U+064B` ARABIC FATHATAN (tanwīn *an*).
pub const FATHATAN: char = '\u{064B}';
/// `DAMMATAN` — `U+064C` ARABIC DAMMATAN (tanwīn *un*).
pub const DAMMATAN: char = '\u{064C}';
/// `KASRATAN` — `U+064D` ARABIC KASRATAN (tanwīn *in*).
pub const KASRATAN: char = '\u{064D}';
/// `FATHA` — `U+064E` ARABIC FATHA.
pub const FATHA: char = '\u{064E}';
/// `DAMMA` — `U+064F` ARABIC DAMMA.
pub const DAMMA: char = '\u{064F}';
/// `KASRA` — `U+0650` ARABIC KASRA.
pub const KASRA: char = '\u{0650}';
/// `SHADDA` — `U+0651` ARABIC SHADDA (gemination); in `TASHKEEL` but **not** in `HARAKAT`.
pub const SHADDA: char = '\u{0651}';
/// `SUKUN` — `U+0652` ARABIC SUKUN (no vowel).
pub const SUKUN: char = '\u{0652}';
/// `MADDA_ABOVE` — `U+0653` ARABIC MADDAH ABOVE (combining).
pub const MADDA_ABOVE: char = '\u{0653}';
/// `HAMZA_ABOVE` — `U+0654` ARABIC HAMZA ABOVE (combining).
pub const HAMZA_ABOVE: char = '\u{0654}';
/// `HAMZA_BELOW` — `U+0655` ARABIC HAMZA BELOW (combining).
pub const HAMZA_BELOW: char = '\u{0655}';

// ------------------------------------------------------------- small letters

/// `SMALL_ALEF` — `U+0670` ARABIC LETTER SUPERSCRIPT ALEF.
pub const SMALL_ALEF: char = '\u{0670}';
/// `MINI_ALEF` — `U+0670`; upstream alias of [`SMALL_ALEF`].
pub const MINI_ALEF: char = SMALL_ALEF;
/// `SMALL_WAW` — `U+06E5` ARABIC SMALL WAW.
pub const SMALL_WAW: char = '\u{06E5}';
/// `SMALL_YEH` — `U+06E6` ARABIC SMALL YEH.
pub const SMALL_YEH: char = '\u{06E6}';

// -------------------------------------------------------------------- digits

/// `ZERO` — `U+0660` ARABIC-INDIC DIGIT ZERO (east/`٠`).
pub const ZERO: char = '\u{0660}';
/// `ONE` — `U+0661` ARABIC-INDIC DIGIT ONE (east/`١`).
pub const ONE: char = '\u{0661}';
/// `TWO` — `U+0662` ARABIC-INDIC DIGIT TWO (east/`٢`).
pub const TWO: char = '\u{0662}';
/// `THREE` — `U+0663` ARABIC-INDIC DIGIT THREE (east/`٣`).
pub const THREE: char = '\u{0663}';
/// `FOUR` — `U+0664` ARABIC-INDIC DIGIT FOUR (east/`٤`).
pub const FOUR: char = '\u{0664}';
/// `FIVE` — `U+0665` ARABIC-INDIC DIGIT FIVE (east/`٥`).
pub const FIVE: char = '\u{0665}';
/// `SIX` — `U+0666` ARABIC-INDIC DIGIT SIX (east/`٦`).
pub const SIX: char = '\u{0666}';
/// `SEVEN` — `U+0667` ARABIC-INDIC DIGIT SEVEN (east/`٧`).
pub const SEVEN: char = '\u{0667}';
/// `EIGHT` — `U+0668` ARABIC-INDIC DIGIT EIGHT (east/`٨`).
pub const EIGHT: char = '\u{0668}';
/// `NINE` — `U+0669` ARABIC-INDIC DIGIT NINE (east/`٩`).
pub const NINE: char = '\u{0669}';

/// `ZERO_W` — `U+0030` DIGIT ZERO (west/ASCII).
pub const ZERO_W: char = '\u{0030}';
/// `ONE_W` — `U+0031` DIGIT ONE (west/ASCII).
pub const ONE_W: char = '\u{0031}';
/// `TWO_W` — `U+0032` DIGIT TWO (west/ASCII).
pub const TWO_W: char = '\u{0032}';
/// `THREE_W` — `U+0033` DIGIT THREE (west/ASCII).
pub const THREE_W: char = '\u{0033}';
/// `FOUR_W` — `U+0034` DIGIT FOUR (west/ASCII).
pub const FOUR_W: char = '\u{0034}';
/// `FIVE_W` — `U+0035` DIGIT FIVE (west/ASCII).
pub const FIVE_W: char = '\u{0035}';
/// `SIX_W` — `U+0036` DIGIT SIX (west/ASCII).
pub const SIX_W: char = '\u{0036}';
/// `SEVEN_W` — `U+0037` DIGIT SEVEN (west/ASCII).
pub const SEVEN_W: char = '\u{0037}';
/// `EIGHT_W` — `U+0038` DIGIT EIGHT (west/ASCII).
pub const EIGHT_W: char = '\u{0038}';
/// `NINE_W` — `U+0039` DIGIT NINE (west/ASCII).
pub const NINE_W: char = '\u{0039}';

/// `ZERO_P` — `U+06F0` EXTENDED ARABIC-INDIC DIGIT ZERO (Persian/Urdu `۰`).
pub const ZERO_P: char = '\u{06F0}';
/// `ONE_P` — `U+06F1` EXTENDED ARABIC-INDIC DIGIT ONE (Persian/Urdu `۱`).
pub const ONE_P: char = '\u{06F1}';
/// `TWO_P` — `U+06F2` EXTENDED ARABIC-INDIC DIGIT TWO (Persian/Urdu `۲`).
pub const TWO_P: char = '\u{06F2}';
/// `THREE_P` — `U+06F3` EXTENDED ARABIC-INDIC DIGIT THREE (Persian/Urdu `۳`).
pub const THREE_P: char = '\u{06F3}';
/// `FOUR_P` — `U+06F4` EXTENDED ARABIC-INDIC DIGIT FOUR (Persian/Urdu `۴`).
pub const FOUR_P: char = '\u{06F4}';
/// `FIVE_P` — `U+06F5` EXTENDED ARABIC-INDIC DIGIT FIVE (Persian/Urdu `۵`).
pub const FIVE_P: char = '\u{06F5}';
/// `SIX_P` — `U+06F6` EXTENDED ARABIC-INDIC DIGIT SIX (Persian/Urdu `۶`).
pub const SIX_P: char = '\u{06F6}';
/// `SEVEN_P` — `U+06F7` EXTENDED ARABIC-INDIC DIGIT SEVEN (Persian/Urdu `۷`).
pub const SEVEN_P: char = '\u{06F7}';
/// `EIGHT_P` — `U+06F8` EXTENDED ARABIC-INDIC DIGIT EIGHT (Persian/Urdu `۸`).
pub const EIGHT_P: char = '\u{06F8}';
/// `NINE_P` — `U+06F9` EXTENDED ARABIC-INDIC DIGIT NINE (Persian/Urdu `۹`).
pub const NINE_P: char = '\u{06F9}';

// ---------------------------------------------------------------- typography

/// `PERCENT` — `U+066A` ARABIC PERCENT SIGN (`٪`).
pub const PERCENT: char = '\u{066A}';
/// `DECIMAL` — `U+066B` ARABIC DECIMAL SEPARATOR (`٫`).
pub const DECIMAL: char = '\u{066B}';
/// `THOUSANDS` — `U+066C` ARABIC THOUSANDS SEPARATOR (`٬`).
pub const THOUSANDS: char = '\u{066C}';
/// `STAR` — `U+066D` ARABIC FIVE POINTED STAR (`٭`).
pub const STAR: char = '\u{066D}';
/// `ALEF_WASLA` — `U+0671` ARABIC LETTER ALEF WASLA (`ٱ`).
pub const ALEF_WASLA: char = '\u{0671}';
/// `FULL_STOP` — `U+06D4` ARABIC FULL STOP (`۔`).
pub const FULL_STOP: char = '\u{06D4}';
/// `BYTE_ORDER_MARK` — `U+FEFF` ZERO WIDTH NO-BREAK SPACE / BOM.
///
/// Preserved by transforms and never dropped implicitly (`CLAUDE.md` §3.4, edge case E13).
pub const BYTE_ORDER_MARK: char = '\u{FEFF}';

// --------------------------------------------------- formatting (r_abic only)

// The five code points below are **r_abic additions**: upstream PyArabic defines no constant for
// them. The tokenizer of Phase 1 needs an explicit classification for each of them.

/// `ZERO_WIDTH_NON_JOINER` — `U+200C` (`r_abic` addition).
pub const ZERO_WIDTH_NON_JOINER: char = '\u{200C}';
/// `ZERO_WIDTH_JOINER` — `U+200D` (`r_abic` addition).
pub const ZERO_WIDTH_JOINER: char = '\u{200D}';
/// `LEFT_TO_RIGHT_MARK` — `U+200E` (`r_abic` addition).
pub const LEFT_TO_RIGHT_MARK: char = '\u{200E}';
/// `RIGHT_TO_LEFT_MARK` — `U+200F` (`r_abic` addition).
pub const RIGHT_TO_LEFT_MARK: char = '\u{200F}';
/// `ARABIC_LETTER_MARK` — `U+061C` (`r_abic` addition).
pub const ARABIC_LETTER_MARK: char = '\u{061C}';

// ------------------------------------------------------------------ ligatures

/// `LAM_ALEF` — `U+FEFB` ARABIC LIGATURE LAM WITH ALEF ISOLATED FORM (expands to `LAM` + `ALEF`).
pub const LAM_ALEF: char = '\u{FEFB}';
/// `LAM_ALEF_HAMZA_ABOVE` — `U+FEF7` LAM WITH ALEF WITH HAMZA ABOVE, isolated form.
pub const LAM_ALEF_HAMZA_ABOVE: char = '\u{FEF7}';
/// `LAM_ALEF_HAMZA_BELOW` — `U+FEF9` LAM WITH ALEF WITH HAMZA BELOW, isolated form.
pub const LAM_ALEF_HAMZA_BELOW: char = '\u{FEF9}';
/// `LAM_ALEF_MADDA_ABOVE` — `U+FEF5` LAM WITH ALEF WITH MADDA ABOVE, isolated form.
pub const LAM_ALEF_MADDA_ABOVE: char = '\u{FEF5}';

/// `SIMPLE_LAM_ALEF` — the two-letter sequence `لا` (`U+0644` + `U+0627`).
pub const SIMPLE_LAM_ALEF: &str = "\u{0644}\u{0627}";
/// `SIMPLE_LAM_ALEF_HAMZA_ABOVE` — the two-letter sequence `لأ`.
pub const SIMPLE_LAM_ALEF_HAMZA_ABOVE: &str = "\u{0644}\u{0623}";
/// `SIMPLE_LAM_ALEF_HAMZA_BELOW` — the two-letter sequence `لإ`.
pub const SIMPLE_LAM_ALEF_HAMZA_BELOW: &str = "\u{0644}\u{0625}";
/// `SIMPLE_LAM_ALEF_MADDA_ABOVE` — the two-letter sequence `لآ`.
pub const SIMPLE_LAM_ALEF_MADDA_ABOVE: &str = "\u{0644}\u{0622}";

#[cfg(test)]
#[allow(clippy::wildcard_imports)] // the tables below enumerate nearly every constant
mod tests {
    use super::*;

    #[test]
    fn punctuation_letters_and_marks_match_their_code_points() {
        const CASES: &[(char, u32, &str)] = &[
            (COMMA, 0x060C, "COMMA"),
            (SEMICOLON, 0x061B, "SEMICOLON"),
            (QUESTION, 0x061F, "QUESTION"),
            (HAMZA, 0x0621, "HAMZA"),
            (ALEF_MADDA, 0x0622, "ALEF_MADDA"),
            (ALEF_HAMZA_ABOVE, 0x0623, "ALEF_HAMZA_ABOVE"),
            (WAW_HAMZA, 0x0624, "WAW_HAMZA"),
            (ALEF_HAMZA_BELOW, 0x0625, "ALEF_HAMZA_BELOW"),
            (YEH_HAMZA, 0x0626, "YEH_HAMZA"),
            (ALEF, 0x0627, "ALEF"),
            (BEH, 0x0628, "BEH"),
            (TEH_MARBUTA, 0x0629, "TEH_MARBUTA"),
            (TEH, 0x062A, "TEH"),
            (THEH, 0x062B, "THEH"),
            (JEEM, 0x062C, "JEEM"),
            (HAH, 0x062D, "HAH"),
            (KHAH, 0x062E, "KHAH"),
            (DAL, 0x062F, "DAL"),
            (THAL, 0x0630, "THAL"),
            (REH, 0x0631, "REH"),
            (ZAIN, 0x0632, "ZAIN"),
            (SEEN, 0x0633, "SEEN"),
            (SHEEN, 0x0634, "SHEEN"),
            (SAD, 0x0635, "SAD"),
            (DAD, 0x0636, "DAD"),
            (TAH, 0x0637, "TAH"),
            (ZAH, 0x0638, "ZAH"),
            (AIN, 0x0639, "AIN"),
            (GHAIN, 0x063A, "GHAIN"),
            (TATWEEL, 0x0640, "TATWEEL"),
            (FEH, 0x0641, "FEH"),
            (QAF, 0x0642, "QAF"),
            (KAF, 0x0643, "KAF"),
            (LAM, 0x0644, "LAM"),
            (MEEM, 0x0645, "MEEM"),
            (NOON, 0x0646, "NOON"),
            (HEH, 0x0647, "HEH"),
            (WAW, 0x0648, "WAW"),
            (ALEF_MAKSURA, 0x0649, "ALEF_MAKSURA"),
            (YEH, 0x064A, "YEH"),
            (FATHATAN, 0x064B, "FATHATAN"),
            (DAMMATAN, 0x064C, "DAMMATAN"),
            (KASRATAN, 0x064D, "KASRATAN"),
            (FATHA, 0x064E, "FATHA"),
            (DAMMA, 0x064F, "DAMMA"),
            (KASRA, 0x0650, "KASRA"),
            (SHADDA, 0x0651, "SHADDA"),
            (SUKUN, 0x0652, "SUKUN"),
            (MADDA_ABOVE, 0x0653, "MADDA_ABOVE"),
            (HAMZA_ABOVE, 0x0654, "HAMZA_ABOVE"),
            (HAMZA_BELOW, 0x0655, "HAMZA_BELOW"),
        ];

        for &(ch, expected, name) in CASES {
            assert_eq!(u32::from(ch), expected, "constant {name}");
        }
    }

    #[test]
    fn letters_are_distinct_and_within_the_arabic_block() {
        const BASE_LETTERS: &[char] = &[
            HAMZA,
            ALEF_MADDA,
            ALEF_HAMZA_ABOVE,
            WAW_HAMZA,
            ALEF_HAMZA_BELOW,
            YEH_HAMZA,
            ALEF,
            BEH,
            TEH_MARBUTA,
            TEH,
            THEH,
            JEEM,
            HAH,
            KHAH,
            DAL,
            THAL,
            REH,
            ZAIN,
            SEEN,
            SHEEN,
            SAD,
            DAD,
            TAH,
            ZAH,
            AIN,
            GHAIN,
            FEH,
            QAF,
            KAF,
            LAM,
            MEEM,
            NOON,
            HEH,
            WAW,
            ALEF_MAKSURA,
            YEH,
        ];

        assert_eq!(BASE_LETTERS.len(), 36, "mirrors upstream `araby.LETTERS`");
        for (i, &a) in BASE_LETTERS.iter().enumerate() {
            assert!(
                matches!(a, '\u{0600}'..='\u{06FF}'),
                "letter {a:?} outside the Arabic block"
            );
            assert!(!TATWEEL.eq(&a), "TATWEEL is a joiner, never a letter");
            for &b in &BASE_LETTERS[i + 1..] {
                assert_ne!(a, b, "duplicate letter constant {a:?}");
            }
        }
    }

    #[test]
    fn aliases_are_consistent() {
        assert_eq!(
            MINI_ALEF, SMALL_ALEF,
            "MINI_ALEF is the upstream alias of SMALL_ALEF"
        );
        assert_eq!(SIMPLE_LAM_ALEF.chars().count(), 2);
        assert_eq!(SIMPLE_LAM_ALEF.chars().next(), Some(LAM));
        assert_eq!(SIMPLE_LAM_ALEF.chars().nth(1), Some(ALEF));
    }

    #[test]
    fn digits_typography_and_ligatures_match_their_code_points() {
        const CASES: &[(char, u32, &str)] = &[
            (ZERO, 0x0660, "ZERO"),
            (ONE, 0x0661, "ONE"),
            (TWO, 0x0662, "TWO"),
            (THREE, 0x0663, "THREE"),
            (FOUR, 0x0664, "FOUR"),
            (FIVE, 0x0665, "FIVE"),
            (SIX, 0x0666, "SIX"),
            (SEVEN, 0x0667, "SEVEN"),
            (EIGHT, 0x0668, "EIGHT"),
            (NINE, 0x0669, "NINE"),
            (ZERO_W, 0x0030, "ZERO_W"),
            (ONE_W, 0x0031, "ONE_W"),
            (TWO_W, 0x0032, "TWO_W"),
            (THREE_W, 0x0033, "THREE_W"),
            (FOUR_W, 0x0034, "FOUR_W"),
            (FIVE_W, 0x0035, "FIVE_W"),
            (SIX_W, 0x0036, "SIX_W"),
            (SEVEN_W, 0x0037, "SEVEN_W"),
            (EIGHT_W, 0x0038, "EIGHT_W"),
            (NINE_W, 0x0039, "NINE_W"),
            (ZERO_P, 0x06F0, "ZERO_P"),
            (ONE_P, 0x06F1, "ONE_P"),
            (TWO_P, 0x06F2, "TWO_P"),
            (THREE_P, 0x06F3, "THREE_P"),
            (FOUR_P, 0x06F4, "FOUR_P"),
            (FIVE_P, 0x06F5, "FIVE_P"),
            (SIX_P, 0x06F6, "SIX_P"),
            (SEVEN_P, 0x06F7, "SEVEN_P"),
            (EIGHT_P, 0x06F8, "EIGHT_P"),
            (NINE_P, 0x06F9, "NINE_P"),
            (SMALL_ALEF, 0x0670, "SMALL_ALEF"),
            (SMALL_WAW, 0x06E5, "SMALL_WAW"),
            (SMALL_YEH, 0x06E6, "SMALL_YEH"),
            (PERCENT, 0x066A, "PERCENT"),
            (DECIMAL, 0x066B, "DECIMAL"),
            (THOUSANDS, 0x066C, "THOUSANDS"),
            (STAR, 0x066D, "STAR"),
            (ALEF_WASLA, 0x0671, "ALEF_WASLA"),
            (FULL_STOP, 0x06D4, "FULL_STOP"),
            (BYTE_ORDER_MARK, 0xFEFF, "BYTE_ORDER_MARK"),
            (ARABIC_LETTER_MARK, 0x061C, "ARABIC_LETTER_MARK"),
            (ZERO_WIDTH_NON_JOINER, 0x200C, "ZERO_WIDTH_NON_JOINER"),
            (ZERO_WIDTH_JOINER, 0x200D, "ZERO_WIDTH_JOINER"),
            (LEFT_TO_RIGHT_MARK, 0x200E, "LEFT_TO_RIGHT_MARK"),
            (RIGHT_TO_LEFT_MARK, 0x200F, "RIGHT_TO_LEFT_MARK"),
            (LAM_ALEF, 0xFEFB, "LAM_ALEF"),
            (LAM_ALEF_HAMZA_ABOVE, 0xFEF7, "LAM_ALEF_HAMZA_ABOVE"),
            (LAM_ALEF_HAMZA_BELOW, 0xFEF9, "LAM_ALEF_HAMZA_BELOW"),
            (LAM_ALEF_MADDA_ABOVE, 0xFEF5, "LAM_ALEF_MADDA_ABOVE"),
        ];

        for &(ch, expected, name) in CASES {
            assert_eq!(u32::from(ch), expected, "constant {name}");
        }
    }

    #[test]
    fn digit_sets_are_contiguous_and_mutually_distinct() {
        // Digit-set conversion maps by offset, so the three sets must stay contiguous.
        assert_eq!(u32::from(NINE) - u32::from(ZERO), 9);
        assert_eq!(u32::from(NINE_W) - u32::from(ZERO_W), 9);
        assert_eq!(u32::from(NINE_P) - u32::from(ZERO_P), 9);
        assert_ne!(ZERO, ZERO_W);
        assert_ne!(ZERO, ZERO_P);
        assert_ne!(ZERO_W, ZERO_P);
    }

    #[test]
    fn ligature_constants_correspond_to_the_simple_sequences() {
        const PAIRS: &[(char, &str)] = &[
            (LAM_ALEF, SIMPLE_LAM_ALEF),
            (LAM_ALEF_HAMZA_ABOVE, SIMPLE_LAM_ALEF_HAMZA_ABOVE),
            (LAM_ALEF_HAMZA_BELOW, SIMPLE_LAM_ALEF_HAMZA_BELOW),
            (LAM_ALEF_MADDA_ABOVE, SIMPLE_LAM_ALEF_MADDA_ABOVE),
        ];

        for &(ligature, simple) in PAIRS {
            let code = u32::from(ligature);
            assert!(
                (0xFEF5..=0xFEFC).contains(&code),
                "ligature {ligature:?} not in Forms-B"
            );
            let mut chars = simple.chars();
            assert_eq!(
                chars.next(),
                Some(LAM),
                "simple sequence must start with LAM"
            );
            assert!(
                matches!(
                    chars.next(),
                    Some(ALEF | ALEF_HAMZA_ABOVE | ALEF_HAMZA_BELOW | ALEF_MADDA)
                ),
                "second letter of {simple:?} must be an alef form"
            );
            assert_eq!(chars.next(), None, "only two letters are expected");
        }
    }
}

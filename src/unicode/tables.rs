// SPDX-License-Identifier: MIT OR Apache-2.0
//! Character classes, mirroring the group constants of `pyarabic.araby_const`.
//!
//! Group order mirrors upstream exactly, so parity diffs stay greppable (`CLAUDE.md` §3.2). Every
//! group is a `const` slice (no allocation, no lazy initialisation) and every table is validated by
//! the tests at the bottom of this file (`CLAUDE.md` §3.5).
//!
//! ```
//! use r_abic::unicode::{constants::TATWEEL, tables};
//!
//! assert_eq!(tables::NOT_DEF_HARAKA, TATWEEL);
//! assert_eq!(tables::TASHKEEL.len(), 8);
//! assert_eq!(tables::MOON.len(), 18);
//! assert_eq!(tables::alphabetic_order(r_abic::unicode::constants::ALEF), Some(1));
//! ```

use super::constants::{
    AIN, ALEF, ALEF_HAMZA_ABOVE, ALEF_HAMZA_BELOW, ALEF_MADDA, ALEF_MAKSURA, ALEF_WASLA, BEH, DAD,
    DAL, DAMMA, DAMMATAN, EIGHT, EIGHT_P, EIGHT_W, FATHA, FATHATAN, FEH, FIVE, FIVE_P, FIVE_W,
    FOUR, FOUR_P, FOUR_W, GHAIN, HAH, HAMZA, HAMZA_ABOVE, HAMZA_BELOW, HEH, JEEM, KAF, KASRA,
    KASRATAN, KHAH, LAM, LAM_ALEF, LAM_ALEF_HAMZA_ABOVE, LAM_ALEF_HAMZA_BELOW,
    LAM_ALEF_MADDA_ABOVE, MEEM, NINE, NINE_P, NINE_W, NOON, ONE, ONE_P, ONE_W, QAF, REH, SAD, SEEN,
    SEVEN, SEVEN_P, SEVEN_W, SHADDA, SHEEN, SIX, SIX_P, SIX_W, SMALL_ALEF, SMALL_WAW, SMALL_YEH,
    SUKUN, TAH, TATWEEL, TEH, TEH_MARBUTA, THAL, THEH, THREE, THREE_P, THREE_W, TWO, TWO_P, TWO_W,
    WAW, WAW_HAMZA, YEH, YEH_HAMZA, ZAH, ZAIN, ZERO, ZERO_P, ZERO_W,
};

// ------------------------------------------------------------- tashkeel family

/// `NOT_DEF_HARAKA` — the "no ḥaraka defined" placeholder; upstream aliases it to [`TATWEEL`].
pub const NOT_DEF_HARAKA: char = TATWEEL;

/// `HARAKAT` — short vowels, tanwīn and sukūn; **excludes** `SHADDA` (7 members).
pub const HARAKAT: &[char] = &[FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SUKUN];

/// `TASHKEEL` — [`HARAKAT`] plus [`SHADDA`] (8 members).
pub const TASHKEEL: &[char] = &[
    FATHATAN, DAMMATAN, KASRATAN, FATHA, DAMMA, KASRA, SUKUN, SHADDA,
];

/// `SHORTHARAKAT` — the four short ḥarakāt (no tanwīn, no shadda).
pub const SHORTHARAKAT: &[char] = &[FATHA, DAMMA, KASRA, SUKUN];

/// `TANWIN` — the three nunation marks.
pub const TANWIN: &[char] = &[FATHATAN, DAMMATAN, KASRATAN];

// ------------------------------------------------------------ letter families

/// `LIGUATURES` — the four lam-alef presentation forms; each expands to two letters when de-shaped.
pub const LIGUATURES: &[char] = &[
    LAM_ALEF,
    LAM_ALEF_HAMZA_ABOVE,
    LAM_ALEF_HAMZA_BELOW,
    LAM_ALEF_MADDA_ABOVE,
];

/// `HAMZAT` — every code point that is, or carries, a hamza.
pub const HAMZAT: &[char] = &[
    HAMZA,
    WAW_HAMZA,
    YEH_HAMZA,
    HAMZA_ABOVE,
    HAMZA_BELOW,
    ALEF_HAMZA_BELOW,
    ALEF_HAMZA_ABOVE,
];

/// `ALEFAT` — every alef-like letter, including `ALEF_WASLA` and superscript alef.
pub const ALEFAT: &[char] = &[
    ALEF,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    ALEF_HAMZA_BELOW,
    ALEF_WASLA,
    ALEF_MAKSURA,
    SMALL_ALEF,
];

/// `LETTERS` — the 36 base Arabic letters, including the hamza carriers (upstream order).
pub const LETTERS: &[char] = &[
    ALEF,
    BEH,
    TEH,
    TEH_MARBUTA,
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
    HAMZA,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    WAW_HAMZA,
    ALEF_HAMZA_BELOW,
    YEH_HAMZA,
];

/// `WEAK` — the weak letters (`alef`, `waw`, `yeh`, `alef maksura`).
pub const WEAK: &[char] = &[ALEF, WAW, YEH, ALEF_MAKSURA];

/// `YEHLIKE` — every code point that is drawn or behaves like a yeh.
pub const YEHLIKE: &[char] = &[YEH, YEH_HAMZA, ALEF_MAKSURA, SMALL_YEH];

/// `WAWLIKE` — every code point that is drawn or behaves like a waw.
pub const WAWLIKE: &[char] = &[WAW, WAW_HAMZA, SMALL_WAW];

/// `TEHLIKE` — the two teh forms.
pub const TEHLIKE: &[char] = &[TEH, TEH_MARBUTA];

/// `SMALL` — the small (superscript) letters.
pub const SMALL: &[char] = &[SMALL_ALEF, SMALL_WAW, SMALL_YEH];

/// `MOON` — the 18 "moon" letters, which do not assimilate the lam of the definite article.
pub const MOON: &[char] = &[
    HAMZA,
    ALEF_MADDA,
    ALEF_HAMZA_ABOVE,
    ALEF_HAMZA_BELOW,
    ALEF,
    BEH,
    JEEM,
    HAH,
    KHAH,
    AIN,
    GHAIN,
    FEH,
    QAF,
    KAF,
    MEEM,
    HEH,
    WAW,
    YEH,
];

/// `SUN` — the 14 "sun" letters, which assimilate the lam of the definite article.
pub const SUN: &[char] = &[
    TEH, THEH, DAL, THAL, REH, ZAIN, SEEN, SHEEN, SAD, DAD, TAH, ZAH, LAM, NOON,
];

// ---------------------------------------------------------------- digit sets

/// `NUMBERS_EAST` — Arabic-Indic (east) digits, ordered `0..=9`.
pub const NUMBERS_EAST: [char; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

/// `NUMBERS_WEST` — ASCII (west) digits, ordered `0..=9`.
pub const NUMBERS_WEST: [char; 10] = [
    ZERO_W, ONE_W, TWO_W, THREE_W, FOUR_W, FIVE_W, SIX_W, SEVEN_W, EIGHT_W, NINE_W,
];

/// `NUMBERS_PERS` — Extended Arabic-Indic (Persian/Urdu) digits, ordered `0..=9`.
pub const NUMBERS_PERS: [char; 10] = [
    ZERO_P, ONE_P, TWO_P, THREE_P, FOUR_P, FIVE_P, SIX_P, SEVEN_P, EIGHT_P, NINE_P,
];

// --------------------------------------------------------- alphabetic order

/// `ALPHABETIC_ORDER` — upstream's letter → Arabic alphabetical rank table, **sorted by code point**
/// so that [`alphabetic_order`] can binary-search it.
///
/// Ranks follow the Arabic alphabet: `ALEF` = 1 … `YEH` = 28, and every hamza form (including the
/// carriers `ALEF_MADDA`, `ALEF_HAMZA_ABOVE`, `WAW_HAMZA`, `ALEF_HAMZA_BELOW`, `YEH_HAMZA`) = 29.
/// `TATWEEL` and `ALEF_MAKSURA` are not ranked, exactly as upstream.
pub const ALPHABETIC_ORDER: &[(char, u8)] = &[
    (HAMZA, 29),
    (ALEF_MADDA, 29),
    (ALEF_HAMZA_ABOVE, 29),
    (WAW_HAMZA, 29),
    (ALEF_HAMZA_BELOW, 29),
    (YEH_HAMZA, 29),
    (ALEF, 1),
    (BEH, 2),
    (TEH_MARBUTA, 3),
    (TEH, 3),
    (THEH, 4),
    (JEEM, 5),
    (HAH, 6),
    (KHAH, 7),
    (DAL, 8),
    (THAL, 9),
    (REH, 10),
    (ZAIN, 11),
    (SEEN, 12),
    (SHEEN, 13),
    (SAD, 14),
    (DAD, 15),
    (TAH, 16),
    (ZAH, 17),
    (AIN, 18),
    (GHAIN, 19),
    (FEH, 20),
    (QAF, 21),
    (KAF, 22),
    (LAM, 23),
    (MEEM, 24),
    (NOON, 25),
    (HEH, 26),
    (WAW, 27),
    (YEH, 28),
];

/// Returns the Arabic alphabetic rank of `c`, or `None` when `c` is not a ranked base letter.
///
/// Mirrors `PyArabic` `araby.ALPHABETIC_ORDER` (a dict lookup upstream).
///
/// # Examples
///
/// ```
/// use r_abic::unicode::constants::{ALEF, TATWEEL, YEH};
/// use r_abic::unicode::tables::alphabetic_order;
///
/// assert_eq!(alphabetic_order(ALEF), Some(1));
/// assert_eq!(alphabetic_order(YEH), Some(28));
/// assert_eq!(alphabetic_order(TATWEEL), None);
/// ```
///
/// # Performance
///
/// O(log n) over a 35-entry `const` table, no allocation.
#[must_use]
pub fn alphabetic_order(c: char) -> Option<u8> {
    ALPHABETIC_ORDER
        .binary_search_by_key(&c, |&(letter, _)| letter)
        .ok()
        .map(|index| ALPHABETIC_ORDER[index].1)
}

#[cfg(test)]
mod tests {
    use super::{
        ALEFAT, ALPHABETIC_ORDER, HAMZAT, HARAKAT, LETTERS, LIGUATURES, MOON, NOT_DEF_HARAKA,
        NUMBERS_EAST, NUMBERS_PERS, NUMBERS_WEST, SHORTHARAKAT, SMALL, SUN, TANWIN, TASHKEEL,
        TEHLIKE, WAWLIKE, WEAK, YEHLIKE, alphabetic_order,
    };
    use crate::unicode::constants::{
        ALEF, ALEF_MADDA, ALEF_MAKSURA, BEH, FEH, GHAIN, HAMZA, SHADDA, TATWEEL, TEH, TEH_MARBUTA,
        THEH, WAW, YEH, YEH_HAMZA, ZERO, ZERO_P, ZERO_W,
    };

    fn assert_no_duplicates(group: &[char], name: &str) {
        for (index, &letter) in group.iter().enumerate() {
            assert!(
                !group[index + 1..].contains(&letter),
                "{name} contains {letter:?} more than once"
            );
        }
    }

    #[test]
    fn groups_have_the_documented_sizes_and_no_duplicates() {
        let sizes: &[(&str, usize, &[char])] = &[
            ("HARAKAT", 7, HARAKAT),
            ("TASHKEEL", 8, TASHKEEL),
            ("SHORTHARAKAT", 4, SHORTHARAKAT),
            ("TANWIN", 3, TANWIN),
            ("LIGUATURES", 4, LIGUATURES),
            ("HAMZAT", 7, HAMZAT),
            ("ALEFAT", 7, ALEFAT),
            ("LETTERS", 36, LETTERS),
            ("WEAK", 4, WEAK),
            ("YEHLIKE", 4, YEHLIKE),
            ("WAWLIKE", 3, WAWLIKE),
            ("TEHLIKE", 2, TEHLIKE),
            ("SMALL", 3, SMALL),
            ("MOON", 18, MOON),
            ("SUN", 14, SUN),
        ];

        for &(name, expected_len, group) in sizes {
            assert_eq!(group.len(), expected_len, "{name} length");
            assert_no_duplicates(group, name);
        }
    }

    #[test]
    fn class_relations_hold() {
        for &letter in HARAKAT {
            assert!(
                TASHKEEL.contains(&letter),
                "{letter:?} missing from TASHKEEL"
            );
        }
        for &letter in SHORTHARAKAT {
            assert!(HARAKAT.contains(&letter), "{letter:?} missing from HARAKAT");
        }
        for &letter in TANWIN {
            assert!(HARAKAT.contains(&letter), "{letter:?} missing from HARAKAT");
        }

        assert!(TASHKEEL.contains(&SHADDA));
        assert!(
            !HARAKAT.contains(&SHADDA),
            "shadda is tashkeel but not a haraka"
        );
        assert_eq!(NOT_DEF_HARAKA, TATWEEL);
    }

    #[test]
    fn moon_and_sun_are_disjoint_subsets_of_the_letters() {
        for &letter in MOON {
            assert!(LETTERS.contains(&letter), "{letter:?} is not a base letter");
            assert!(
                !SUN.contains(&letter),
                "{letter:?} cannot be both moon and sun"
            );
        }
        for &letter in SUN {
            assert!(LETTERS.contains(&letter), "{letter:?} is not a base letter");
        }
        assert!(MOON.len() + SUN.len() <= LETTERS.len());
    }

    #[test]
    fn digit_sets_are_ordered_and_distinct() {
        assert_eq!(NUMBERS_EAST[0], ZERO);
        assert_eq!(NUMBERS_WEST[0], ZERO_W);
        assert_eq!(NUMBERS_PERS[0], ZERO_P);

        for (offset, ((east, west), pers)) in NUMBERS_EAST
            .iter()
            .zip(NUMBERS_WEST.iter())
            .zip(NUMBERS_PERS.iter())
            .enumerate()
        {
            let offset = u32::try_from(offset).expect("offset fits into u32");
            assert_eq!(
                u32::from(*east),
                u32::from(ZERO) + offset,
                "east digit {offset}"
            );
            assert_eq!(
                u32::from(*west),
                u32::from(ZERO_W) + offset,
                "west digit {offset}"
            );
            assert_eq!(
                u32::from(*pers),
                u32::from(ZERO_P) + offset,
                "persian digit {offset}"
            );
        }
    }

    #[test]
    fn alphabetic_order_table_is_sorted_by_code_point() {
        assert_eq!(
            ALPHABETIC_ORDER.len(),
            35,
            "mirrors the 35 ranked code points upstream"
        );
        for pair in ALPHABETIC_ORDER.windows(2) {
            assert!(
                pair[0].0 < pair[1].0,
                "table must stay sorted: {:?} before {:?}",
                pair[0].0,
                pair[1].0
            );
        }
        for &(letter, _) in ALPHABETIC_ORDER {
            assert!(LETTERS.contains(&letter), "{letter:?} is not a base letter");
        }
    }

    #[test]
    fn alphabetic_order_returns_the_documented_ranks() {
        let cases: &[(char, u8)] = &[
            (ALEF, 1),
            (BEH, 2),
            (TEH, 3),
            (TEH_MARBUTA, 3),
            (THEH, 4),
            (GHAIN, 19),
            (FEH, 20),
            (WAW, 27),
            (YEH, 28),
            (HAMZA, 29),
            (ALEF_MADDA, 29),
            (YEH_HAMZA, 29),
        ];

        for &(letter, rank) in cases {
            assert_eq!(alphabetic_order(letter), Some(rank), "rank of {letter:?}");
        }
    }

    #[test]
    fn alphabetic_order_finds_every_entry_and_rejects_everything_else() {
        for &(letter, rank) in ALPHABETIC_ORDER {
            assert_eq!(
                alphabetic_order(letter),
                Some(rank),
                "binary search must find {letter:?}"
            );
        }

        assert_eq!(alphabetic_order(TATWEEL), None, "tatweel is not ranked");
        assert_eq!(
            alphabetic_order(ALEF_MAKSURA),
            None,
            "alef maksura is not ranked"
        );
        assert_eq!(alphabetic_order('a'), None, "latin letters are not ranked");
        assert_eq!(alphabetic_order('\u{0000}'), None);
        assert_eq!(
            alphabetic_order('🎉'),
            None,
            "astral characters are not ranked"
        );
    }
}

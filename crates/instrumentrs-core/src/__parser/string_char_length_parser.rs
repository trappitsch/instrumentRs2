//! Parser for arguments where we know all character lengths.

/// Error type when working with RangeOfChar
#[derive(Debug)]
pub enum RangeOfCharError {
    InvalidPlaceholder,
}

/// Range of bytes for character length placeholders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RangeOfChar {
    /// Start of the range.
    pub start: usize,
    /// Inclusive end of the range.
    pub end: usize,
}

impl TryFrom<&str> for RangeOfChar {
    type Error = RangeOfCharError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value
            .split_once("..")
            .ok_or(RangeOfCharError::InvalidPlaceholder)?;

        let start: usize = parts
            .0
            .parse()
            .map_err(|_| RangeOfCharError::InvalidPlaceholder)?;

        let inclusive = parts.1.starts_with('=');
        let end = if inclusive {
            parts.1[1..]
                .parse::<usize>()
                .map_err(|_| RangeOfCharError::InvalidPlaceholder)?
        } else {
            parts
                .1
                .parse::<usize>()
                .map_err(|_| RangeOfCharError::InvalidPlaceholder)?
                .saturating_sub(1)
        };

        if end < start {
            Err(RangeOfCharError::InvalidPlaceholder)
        } else {
            Ok(Self { start, end })
        }
    }
}

#[derive(Debug)]
pub struct StringCharLengthParser {}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;
    use std::assert_matches;

    proptest! {
        #[test]
        fn range_from_str_valid_exclusive_upper(
            (low, upp) in (0..usize::MAX-1).prop_flat_map(|low| (Just(low), low+1..usize::MAX))
        ) {
            let s = format!("{low}..{upp}");
            let rng_exp = RangeOfChar { start: low, end: upp - 1 };
            let rng = RangeOfChar::try_from(s.as_str()).unwrap();
            assert_eq!(rng, rng_exp);
        }
    }

    proptest! {
        #[test]
        fn range_from_str_valid_inclusive_upper(
            (low, upp) in (0..usize::MAX).prop_flat_map(|low| (Just(low), low..usize::MAX))
        ) {
            let s = format!("{low}..={upp}");
            let rng_exp = RangeOfChar { start: low, end: upp  };
            let rng = RangeOfChar::try_from(s.as_str()).unwrap();
            assert_eq!(rng, rng_exp);
        }
    }

    proptest! {
        #[test]
        fn range_from_str_start_large_end_exclusive_upper(
            (low, upp) in (0..usize::MAX).prop_flat_map(|low| (Just(low), 0..=low))
        ) {
            let s = format!("{low}..{upp}");
            assert!(RangeOfChar::try_from(s.as_str())
                .is_err_and(|e| e == CommandParserError::InvalidPlaceholder));
        }
    }

    proptest! {
        #[test]
        fn range_from_str_start_large_end_inclusive_upper(
            (low, upp) in (1..usize::MAX).prop_flat_map(|low| (Just(low), 0..low))
        ) {
            let s = format!("{low}..={upp}");
            assert!(RangeOfChar::try_from(s.as_str()).is_err());
        }
    }

    #[test]
    fn range_from_str_invalid_stirngs() {
        assert!(RangeOfChar::try_from("23..45I").is_err());
        assert!(RangeOfChar::try_from("23..55..56").is_err());
        assert!(RangeOfChar::try_from("56").is_err());
        assert!(RangeOfChar::try_from("").is_err());
        assert!(RangeOfChar::try_from("23..55:1").is_err());
    }
}

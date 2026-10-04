use crate::derive::arg_parser::CommandParserError;

/// Range of bytes for character length placeholders.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct RangeOfChar {
    /// Start of the range.
    pub start: usize,
    /// Inclusive end of the range.
    pub end: usize,
}

impl TryFrom<&str> for RangeOfChar {
    type Error = CommandParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value
            .split_once("..")
            .ok_or(CommandParserError::InvalidPlaceholder)?;

        let start: usize = parts
            .0
            .parse()
            .map_err(|_| CommandParserError::InvalidPlaceholder)?;

        let inclusive = parts.1.starts_with('=');
        let end = if inclusive {
            parts.1[1..]
                .parse::<usize>()
                .map_err(|_| CommandParserError::InvalidPlaceholder)?
        } else {
            parts
                .1
                .parse::<usize>()
                .map_err(|_| CommandParserError::InvalidPlaceholder)?
                .saturating_sub(1)
        };

        if end < start {
            Err(CommandParserError::InvalidPlaceholder)
        } else {
            Ok(Self { start, end })
        }
    }
}

/// Placeholder type we are dealing with.
#[derive(Debug, PartialEq, Eq)]
pub enum PlaceholderType {
    /// In order of appearance and not length defined placeholder '{}'.
    Unordered,
    /// Sorting defined, but not length defined placeholder '{P}'.
    Pos { pos: usize },
    /// Length defined, in order of appearance placeholder '{:B}'.
    CharLength { range: RangeOfChar },
    /// Length defined and sorting defined placeholder '{P:B}'.
    CharLengthPos { pos: usize, range: RangeOfChar },
}

impl TryFrom<&str> for PlaceholderType {
    type Error = CommandParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // unrodered type: string must be empty.
        if value.is_empty() {
            return Ok(Self::Unordered);
        }

        // if there is no "..", it must be a delimited type.
        if !value.contains("..") {
            let pos: usize = value
                .parse()
                .map_err(|_| CommandParserError::InvalidPlaceholder)?;
            Ok(Self::Pos { pos })
        } else {
            let mut splt = value.split(':');

            let range_str = splt.next().expect("checked that it is not empty");
            let range = RangeOfChar::try_from(range_str)?;

            if let Some(pos_str) = splt.next() {
                let pos: usize = pos_str
                    .parse()
                    .map_err(|_| CommandParserError::InvalidPlaceholder)?;
                Ok(Self::CharLengthPos { pos, range })
            } else {
                Ok(Self::CharLength { range })
            }
        }
    }
}

impl PlaceholderType {
    /// Check if this placeholder type is the same as another one.
    ///
    /// Note that the position and length of characters (if either are present) is ignored in this
    /// comparison. Here, we are solely interested if, e.g., both are positional placeholders or not.
    pub fn is_same_type(&self, rhs: &PlaceholderType) -> bool {
        match &self {
            Self::Unordered => matches!(rhs, &PlaceholderType::Unordered),
            Self::Pos { pos: _ } => matches!(rhs, &PlaceholderType::Pos { .. }),
            Self::CharLength { range: _ } => matches!(rhs, &PlaceholderType::CharLength { .. }),
            Self::CharLengthPos { pos: _, range: _ } => {
                matches!(rhs, &PlaceholderType::CharLengthPos { .. })
            }
        }
    }
}

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
            assert!(RangeOfChar::try_from(s.as_str())
                .is_err_and(|e| e == CommandParserError::InvalidPlaceholder));
        }
    }

    #[test]
    fn range_from_str_invalid_stirngs() {
        assert!(
            RangeOfChar::try_from("23..45I")
                .is_err_and(|e| e == CommandParserError::InvalidPlaceholder)
        );

        assert!(
            RangeOfChar::try_from("23..55..56")
                .is_err_and(|e| e == CommandParserError::InvalidPlaceholder)
        );

        assert!(
            RangeOfChar::try_from("56").is_err_and(|e| e == CommandParserError::InvalidPlaceholder)
        );

        assert!(
            RangeOfChar::try_from("").is_err_and(|e| e == CommandParserError::InvalidPlaceholder)
        );

        assert!(
            RangeOfChar::try_from("23..55:1")
                .is_err_and(|e| e == CommandParserError::InvalidPlaceholder)
        );
    }

    #[test]
    fn unordered_placeholder() {
        assert_matches!(
            PlaceholderType::try_from("").unwrap(),
            PlaceholderType::Unordered
        );
    }

    #[test]
    fn positional_placeholder() {
        assert_matches!(
            PlaceholderType::try_from("0").unwrap(),
            PlaceholderType::Pos { pos: 0 }
        );

        assert_matches!(
            PlaceholderType::try_from("42").unwrap(),
            PlaceholderType::Pos { pos: 42 }
        );
    }

    #[test]
    fn char_length_placeholder() {
        assert_matches!(
            PlaceholderType::try_from("0..3").unwrap(),
            PlaceholderType::CharLength {
                range: RangeOfChar { start: 0, end: 2 }
            }
        );

        assert_matches!(
            PlaceholderType::try_from("0..=3").unwrap(),
            PlaceholderType::CharLength {
                range: RangeOfChar { start: 0, end: 3 }
            }
        );
    }

    #[test]
    fn char_length_positional_placeholder() {
        assert_matches!(
            PlaceholderType::try_from("0..3:0").unwrap(),
            PlaceholderType::CharLengthPos {
                pos: 0,
                range: RangeOfChar { start: 0, end: 2 }
            }
        );

        assert_matches!(
            PlaceholderType::try_from("42..=42:42").unwrap(),
            PlaceholderType::CharLengthPos {
                pos: 42,
                range: RangeOfChar { start: 42, end: 42 }
            }
        );
    }

    #[test]
    fn invalid_placeholders() {
        assert_matches!(
            PlaceholderType::try_from("a").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );

        assert_matches!(
            PlaceholderType::try_from(":asdf").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );

        assert_matches!(
            PlaceholderType::try_from("3:").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );

        assert_matches!(
            PlaceholderType::try_from("a:3").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );

        assert_matches!(
            PlaceholderType::try_from("3:3:3").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );
    }
}

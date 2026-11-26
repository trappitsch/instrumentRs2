use instrumentrs_core::__parser::string_char_length_parser::RangeOfChar;

use crate::derive::arg_parser::CommandParserError;

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
            let range = RangeOfChar::try_from(range_str)
                .map_err(|_| CommandParserError::InvalidPlaceholder)?;

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
    use std::assert_matches;

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

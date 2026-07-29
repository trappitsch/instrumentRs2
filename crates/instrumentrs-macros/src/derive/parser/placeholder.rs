use crate::derive::parser::CommandParserError;

/// Placeholder type we are dealing with.
#[derive(Debug, PartialEq)]
pub enum PlaceholderType {
    /// In order of appearance and not length defined placeholder '{}'.
    Unordered,
    /// Sorting defined, but not length defined placeholder '{P}'.
    Pos { pos: usize },
    /// Length defined, in order of appearance placeholder '{:B}'.
    CharLength { length: usize },
    /// Length defined and sorting defined placeholder '{P:B}'.
    CharLengthPos { pos: usize, length: usize },
}

impl TryFrom<&str> for PlaceholderType {
    type Error = CommandParserError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // unrodered type: string must be empty.
        if value.is_empty() {
            return Ok(Self::Unordered);
        }

        // if there is no ':', it must be a positional type.
        if !value.contains(':') {
            let pos: usize = value
                .parse()
                .map_err(|_| CommandParserError::InvalidPlaceholder)?;
            Ok(Self::Pos { pos })
        } else {
            let mut splt = value.split(':');

            let pos_str = splt
                .next()
                .expect("checked by contains ':' and split on this");

            let length: usize = splt
                .next()
                .expect("checked by contains ':' and split on this")
                .parse()
                .map_err(|_| CommandParserError::InvalidPlaceholder)?;

            // Split is not depleted or length is zero:
            if splt.next().is_some() || length == 0 {
                return Err(CommandParserError::InvalidPlaceholder);
            }

            if pos_str.is_empty() {
                Ok(Self::CharLength { length })
            } else {
                let pos: usize = pos_str
                    .parse()
                    .map_err(|_| CommandParserError::InvalidPlaceholder)?;
                Ok(Self::CharLengthPos { pos, length })
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
            Self::CharLength { length: _ } => matches!(rhs, &PlaceholderType::CharLength { .. }),
            Self::CharLengthPos { pos: _, length: _ } => {
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
            PlaceholderType::try_from(":3").unwrap(),
            PlaceholderType::CharLength { length: 3 }
        );

        assert_matches!(
            PlaceholderType::try_from(":1").unwrap(),
            PlaceholderType::CharLength { length: 1 }
        );
    }

    #[test]
    fn char_length_positional_placeholder() {
        assert_matches!(
            PlaceholderType::try_from("0:3").unwrap(),
            PlaceholderType::CharLengthPos { pos: 0, length: 3 }
        );

        assert_matches!(
            PlaceholderType::try_from("42:13").unwrap(),
            PlaceholderType::CharLengthPos {
                pos: 42,
                length: 13
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

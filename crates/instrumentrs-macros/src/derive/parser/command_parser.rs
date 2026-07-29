//! Parses any command.
//!
//! Only valid parsing commands will make it through the bracket parser.
//!
//! NOTE: The bracket parser should NEVER be called at runtime. It solely serves create the
//! structures to make actual runtime parsers in the `instrumentrs_macros` crate.

use std::collections::HashSet;

use crate::derive::parser::{CommandParserError, placeholder::PlaceholderType};

/// How do we separate between commands?
#[derive(Debug, Clone, PartialEq)]
pub enum CommandSeparation {
    /// A delimiter is used to separate between different types.
    ///
    /// Requires a DelimiterParser to parse the commands from the instrument.
    Delimiter,
    /// Character length is used to separate between different types.
    ///
    /// Requires a CharLengthParser to parse the commands from the instrument.
    CharLength,
}

/// Bracket parser.
///
/// This is the generic parser that checks a given command string for validity and gets the required
/// information on what type of command we have here. It will also check for validity of the
/// brackets. If the command actually contains a bracket, it can be escaped with \ beforehand.
#[derive(Debug)]
pub struct CommandParser {
    /// the original command.
    cmd: String,
    /// at which position is the left bracket (char position, not index!).
    left_pos: Vec<usize>,
    /// at which position is the left bracket (char position, not index!).
    right_pos: Vec<usize>,
    /// in between
    in_between: Vec<PlaceholderType>,
    /// store the required parser type
    parser_type: CommandSeparation,
}

impl TryFrom<&str> for CommandParser {
    type Error = CommandParserError;

    fn try_from(s: &str) -> Result<Self, CommandParserError> {
        let mut left_pos = vec![];
        let mut right_pos = vec![];
        let mut in_between = vec![];

        let mut is_cursor_between = false;
        let mut string_between = String::new();
        let mut last_char = ' ';

        for (it, ch) in s.chars().enumerate() {
            if ch == '{' && last_char != '\\' && !is_cursor_between {
                left_pos.push(it);
                is_cursor_between = true;
            } else if ch == '}' && last_char != '\\' {
                right_pos.push(it);
                in_between.push(PlaceholderType::try_from(string_between.as_str())?);

                string_between.clear();
                is_cursor_between = false;
            } else if is_cursor_between {
                string_between.push(ch);
            }
            last_char = ch;
        }

        // Are all brackets complete?
        if left_pos.len() != right_pos.len() {
            return Err(CommandParserError::InvalidPlaceholder);
        }

        verify_placeholders(&in_between)?;

        let parser_type = match in_between
            .first()
            .expect("already checked that we have items")
        {
            PlaceholderType::Unordered | PlaceholderType::Pos { .. } => {
                CommandSeparation::Delimiter
            }
            PlaceholderType::CharLength { .. } | PlaceholderType::CharLengthPos { .. } => {
                CommandSeparation::CharLength
            }
        };

        if parser_type == CommandSeparation::Delimiter {
            // error if no delimiter was found between placeholders.
            for (lpn, rp) in left_pos.iter().skip(1).zip(right_pos.iter()) {
                if *lpn <= *rp + 1 {
                    return Err(CommandParserError::DelimiterNotFound);
                }
            }
        }

        Ok(CommandParser {
            cmd: s.to_string(),
            left_pos,
            right_pos,
            in_between,
            parser_type,
        })
    }
}

impl CommandParser {
    /// Get delimiter for a delimiter parsing type.
    ///
    /// Panics:
    /// - If the Argument type is not `CommandSeparation::Delimiter`.
    pub fn get_delimiter(&self) -> (String, Vec<String>) {
        if self.parser_type == CommandSeparation::CharLength {
            panic!(
                "instrumentRs bracket_parser get delimiter was called on a CharLength command - this should not happen, report as bug"
            )
        }

        let before: String = self
            .cmd
            .chars()
            .take(
                *self
                    .left_pos
                    .first()
                    .expect("checked during construction that this exists"),
            )
            .collect();

        // add all in between delimiters to the after vector
        let mut after: Vec<String> = vec![];

        // loop through left pos (skipping first) and zip this with right pos (from 0th, but last
        // will be left out)
        for (lpn, rp) in self.left_pos.iter().skip(1).zip(self.right_pos.iter()) {
            // in between commands are the chars between `rp` and `lpn`
            after.push(self.cmd.chars().skip(rp + 1).take(lpn - rp - 1).collect());
        }

        // and now push the last.
        let last_rbracket = self
            .right_pos
            .last()
            .expect("checked when initializing the bracket parser");
        after.push(self.cmd.chars().skip(*last_rbracket + 1).collect());

        (before, after)
    }

    /// Get the sort order for the parameters.
    ///
    /// This returns an index sorting key!
    pub fn get_sort_index_keys(&self) -> Option<Vec<usize>> {
        let mut sort_keys: Vec<usize> = Vec::with_capacity(self.in_between.len());
        for p in &self.in_between {
            match p {
                PlaceholderType::Unordered | PlaceholderType::CharLength { .. } => return None,
                PlaceholderType::Pos { pos }
                | PlaceholderType::CharLengthPos { pos, length: _ } => sort_keys.push(*pos),
            }
        }

        let sort_ind_keys: Vec<usize> = (0..sort_keys.len()).collect();
        let mut pairs: Vec<_> = sort_keys.into_iter().zip(sort_ind_keys).collect();
        pairs.sort_by_key(|(sk, _)| *sk);

        let sort_ind_keys: Vec<usize> = pairs.into_iter().map(|(_, ik)| ik).collect();
        Some(sort_ind_keys)
    }
}

/// Verify the placeholders.
///
/// Error:
/// - No placeholders in slice.
/// - Mixed placeholders were found.
/// - Positional placeholders (if present) are not unique.
fn verify_placeholders(ph: &[PlaceholderType]) -> Result<(), CommandParserError> {
    // Error if we don't have any placeholders.
    if ph.is_empty() {
        return Err(CommandParserError::NoPlaceholder);
    }

    // Error if we have mixed placeholder types.
    if !ph.windows(2).all(|w| w[0].is_same_type(&w[1])) {
        return Err(CommandParserError::MixedPlaceholder);
    }

    let mut uniqueness_set = HashSet::new();
    for p in ph {
        match p {
            PlaceholderType::Pos { pos } | PlaceholderType::CharLengthPos { pos, length: _ } => {
                if !uniqueness_set.insert(pos) {
                    return Err(CommandParserError::PositionNotUnique);
                };
            }
            _ => return Ok(()),
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::assert_matches;

    #[test]
    fn unordered_simple() {
        let bp = CommandParser::try_from("{}").unwrap();

        assert_eq!(bp.cmd, "{}");
        assert_eq!(bp.left_pos, vec![0]);
        assert_eq!(bp.right_pos, vec![1]);
        assert_eq!(bp.in_between, vec![PlaceholderType::Unordered]);
        assert_eq!(bp.parser_type, CommandSeparation::Delimiter);
    }

    #[test]
    fn unordered_multiple() {
        let bp = CommandParser::try_from("CMD {},{},{}").unwrap();

        assert_eq!(bp.cmd, "CMD {},{},{}");
        assert_eq!(bp.left_pos, vec![4, 7, 10]);
        assert_eq!(bp.right_pos, vec![5, 8, 11]);
        assert_eq!(
            bp.in_between,
            vec![
                PlaceholderType::Unordered,
                PlaceholderType::Unordered,
                PlaceholderType::Unordered
            ]
        );
        assert_eq!(bp.parser_type, CommandSeparation::Delimiter);
    }

    #[test]
    fn positional_simple() {
        let bp = CommandParser::try_from("{0}").unwrap();

        assert_eq!(bp.cmd, "{0}");
        assert_eq!(bp.left_pos, vec![0]);
        assert_eq!(bp.right_pos, vec![2]);
        assert_eq!(bp.in_between, vec![PlaceholderType::Pos { pos: 0 }]);
        assert_eq!(bp.parser_type, CommandSeparation::Delimiter);
    }

    #[test]
    fn positional_multiple() {
        let bp = CommandParser::try_from("CMD {1},{0},{2}").unwrap();

        assert_eq!(bp.cmd, "CMD {1},{0},{2}");
        assert_eq!(bp.left_pos, vec![4, 8, 12]);
        assert_eq!(bp.right_pos, vec![6, 10, 14]);
        assert_eq!(
            bp.in_between,
            vec![
                PlaceholderType::Pos { pos: 1 },
                PlaceholderType::Pos { pos: 0 },
                PlaceholderType::Pos { pos: 2 },
            ]
        );
        assert_eq!(bp.parser_type, CommandSeparation::Delimiter);
    }

    #[test]
    fn charlength_simple() {
        let bp = CommandParser::try_from("{:3}").unwrap();

        assert_eq!(bp.cmd, "{:3}");
        assert_eq!(bp.left_pos, vec![0]);
        assert_eq!(bp.right_pos, vec![3]);
        assert_eq!(
            bp.in_between,
            vec![PlaceholderType::CharLength { length: 3 }]
        );
        assert_eq!(bp.parser_type, CommandSeparation::CharLength);
    }

    #[test]
    fn charlength_multiple() {
        let bp = CommandParser::try_from("CMD {:3},{:1},{:42}").unwrap();

        assert_eq!(bp.cmd, "CMD {:3},{:1},{:42}");
        assert_eq!(bp.left_pos, vec![4, 9, 14]);
        assert_eq!(bp.right_pos, vec![7, 12, 18]);
        assert_eq!(
            bp.in_between,
            vec![
                PlaceholderType::CharLength { length: 3 },
                PlaceholderType::CharLength { length: 1 },
                PlaceholderType::CharLength { length: 42 }
            ]
        );
        assert_eq!(bp.parser_type, CommandSeparation::CharLength);
    }

    #[test]
    fn charlengthpos_simple() {
        let bp = CommandParser::try_from("{0:3}").unwrap();

        assert_eq!(bp.cmd, "{0:3}");
        assert_eq!(bp.left_pos, vec![0]);
        assert_eq!(bp.right_pos, vec![4]);
        assert_eq!(
            bp.in_between,
            vec![PlaceholderType::CharLengthPos { pos: 0, length: 3 }]
        );
        assert_eq!(bp.parser_type, CommandSeparation::CharLength);
    }

    #[test]
    fn charlengthpos_multiple() {
        let bp = CommandParser::try_from("CMD {2:3},{0:1},{1:42}").unwrap();

        assert_eq!(bp.cmd, "CMD {2:3},{0:1},{1:42}");
        assert_eq!(bp.left_pos, vec![4, 10, 16]);
        assert_eq!(bp.right_pos, vec![8, 14, 21]);
        assert_eq!(
            bp.in_between,
            vec![
                PlaceholderType::CharLengthPos { pos: 2, length: 3 },
                PlaceholderType::CharLengthPos { pos: 0, length: 1 },
                PlaceholderType::CharLengthPos { pos: 1, length: 42 },
            ]
        );
        assert_eq!(bp.parser_type, CommandSeparation::CharLength);
    }

    #[test]
    fn get_delimiter_order_simple_unordered() {
        let bp = CommandParser::try_from("{}").unwrap();
        let (before, after) = bp.get_delimiter();
        let sort_keys = bp.get_sort_index_keys();

        assert_eq!(before, "");
        assert_eq!(after, [""]);
        assert!(sort_keys.is_none());
    }

    #[test]
    fn get_delimiter_order_multi_unordered() {
        let bp = CommandParser::try_from("CMD {}, {},{}").unwrap();
        let (before, after) = bp.get_delimiter();
        let sort_keys = bp.get_sort_index_keys();

        assert_eq!(before, "CMD ");
        assert_eq!(after, [", ", ",", ""]);
        assert!(sort_keys.is_none());
    }

    #[test]
    fn get_delimiter_order_multi_unordered_with_after() {
        let bp = CommandParser::try_from("CMD {}, {},{}after").unwrap();
        let (before, after) = bp.get_delimiter();
        let sort_keys = bp.get_sort_index_keys();

        assert_eq!(before, "CMD ");
        assert_eq!(after, [", ", ",", "after"]);
        assert!(sort_keys.is_none());
    }

    #[test]
    fn get_sort_keys_positional_multi() {
        let bp = CommandParser::try_from("CMD {1},{3},{2},{0}").unwrap();
        assert_eq!(bp.get_sort_index_keys().unwrap(), [3, 0, 2, 1]);
    }

    /// Test if gaps are allowed in the sort keys. These are sort keys, not sort indices!
    #[test]
    fn get_sort_keys_positional_without_gaps() {
        let bp = CommandParser::try_from("CMD {10},{5},{42},{6}").unwrap();
        assert_eq!(bp.get_sort_index_keys().unwrap(), [1, 3, 0, 2]);
    }

    #[test]
    fn err_unmatched_bracket() {
        assert_matches!(
            CommandParser::try_from("{} {").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );
    }

    #[test]
    fn err_no_placeholder() {
        assert_matches!(
            CommandParser::try_from("").unwrap_err(),
            CommandParserError::NoPlaceholder
        );
    }

    #[test]
    fn err_mixed_placeholders() {
        assert_matches!(
            CommandParser::try_from("{} {1}").unwrap_err(),
            CommandParserError::MixedPlaceholder
        );

        assert_matches!(
            CommandParser::try_from("{} {:333}").unwrap_err(),
            CommandParserError::MixedPlaceholder
        );

        assert_matches!(
            CommandParser::try_from("{0} {3:333}").unwrap_err(),
            CommandParserError::MixedPlaceholder
        );
    }

    #[test]
    fn err_position_not_unique() {
        assert_matches!(
            CommandParser::try_from("{0} {1} {1} {2}").unwrap_err(),
            CommandParserError::PositionNotUnique
        );

        assert_matches!(
            CommandParser::try_from("{0:3} {1:3} {1:9} {2:3}").unwrap_err(),
            CommandParserError::PositionNotUnique
        );
    }

    #[test]
    fn err_placeholder_in_placeholder() {
        assert_matches!(
            CommandParser::try_from("{{}}").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );

        assert_matches!(
            CommandParser::try_from("{} {    {} }, {}").unwrap_err(),
            CommandParserError::InvalidPlaceholder
        );
    }

    #[test]
    fn err_delimited_placeholders_with_zero_length_delimiters() {
        assert_matches!(
            CommandParser::try_from("{} {}{}").unwrap_err(),
            CommandParserError::DelimiterNotFound
        );
    }
}

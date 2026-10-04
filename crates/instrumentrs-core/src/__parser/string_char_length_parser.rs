//! Parser for arguments where we know all character lengths.

use crate::errors::InstrumentError;

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

impl RangeOfChar {
    /// A non-public new function: You must make sure end >= start.
    pub fn new_unchecked(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Return the total length of a given range.
    fn len(&self) -> usize {
        self.end - self.start + 1
    }
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

/// String character length parser.
///
/// This contains a position vector and the sort keys, if required.
#[derive(Debug)]
pub struct StringCharLengthParser {
    /// Ranges that should be parsed.
    ranges: Vec<RangeOfChar>,
}

impl StringCharLengthParser {
    /// Create a new parser from components.
    ///
    /// Note: The ranges must already be ordered correctly! This means that the parser does not have
    /// to order anything anymore.
    pub fn new(ranges: Vec<RangeOfChar>) -> Self {
        Self { ranges }
    }

    /// Parse an instrument return value with a given command structure.
    ///
    /// Return:
    /// Result of a `Vec<String>` with the ordered parameters of this command.
    ///
    /// Error:
    /// - Returns a `InstrumentError::BadInstrumentResponseString` if the parsing fails.
    pub fn parse(&self, value: &str) -> Result<Vec<String>, InstrumentError> {
        let mut params = vec![];

        for rng in &self.ranges {
            let this: String = value.chars().skip(rng.start).take(rng.len()).collect();
            if this.chars().count() != rng.len() {
                return Err(InstrumentError::BadInstrumentResponseString { msg: this });
            };
            params.push(this);
        }

        Ok(params)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use proptest::prelude::*;

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
            assert!(RangeOfChar::try_from(s.as_str()).is_err());
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

    /// Create a random string of given length.
    fn string_of_length(n: usize) -> impl Strategy<Value = String> {
        proptest::collection::vec(any::<char>(), n).prop_map(|chars| chars.into_iter().collect())
    }

    proptest! {
        #[test]
        fn parse_single_param(s in string_of_length(5)) {
            let ranges = vec![RangeOfChar::new_unchecked(0, 4)];
            let parser = StringCharLengthParser::new(ranges);

            let param = parser.parse(&s).unwrap();

            assert_eq!(param[0], s);
        }
    }

    proptest! {
        #[test]
        fn parse_single_param_with_surrounding(s in string_of_length(5)) {
            let from_inst = format!("012{}89012", s);
            let ranges = vec![RangeOfChar::new_unchecked(3, 7)];
            let parser = StringCharLengthParser::new(ranges);

            let param = parser.parse(&from_inst).unwrap();

            assert_eq!(param[0], s);
        }
    }

    proptest! {
        #[test]
        fn parse_multiple_params(s in string_of_length(5), t in string_of_length(3), u in string_of_length(7)) {
            let from_inst = format!("{s}{t}{u}");
            let ranges = vec![RangeOfChar::new_unchecked(0, 4), RangeOfChar::new_unchecked(5, 7), RangeOfChar::new_unchecked(8, 14)];
            let parser = StringCharLengthParser::new(ranges);

            let param = parser.parse(&from_inst).unwrap();

            assert_eq!(param[0], s);
            assert_eq!(param[1], t);
            assert_eq!(param[2], u);
        }
    }

    proptest! {
        #[test]
        fn parse_multiple_params_with_surrounding(s in string_of_length(5), t in string_of_length(3), u in string_of_length(7)) {
            let from_inst = format!("RESP{s},{t},,,{u}");
            let ranges = vec![RangeOfChar::new_unchecked(4, 8), RangeOfChar::new_unchecked(10, 12), RangeOfChar::new_unchecked(16, 22)];

            let parser = StringCharLengthParser::new(ranges);

            let param = parser.parse(&from_inst).unwrap();

            assert_eq!(param[0], s);
            assert_eq!(param[1], t);
            assert_eq!(param[2], u);
        }
    }

    proptest! {
        #[test]
        fn parse_multiple_params_with_order(s in string_of_length(5), t in string_of_length(3), u in string_of_length(7)) {
            let from_inst = format!("{t}{u}{s}");

            let ranges = vec![RangeOfChar::new_unchecked(10, 14), RangeOfChar::new_unchecked(0, 2), RangeOfChar::new_unchecked(3, 9)];

            let parser = StringCharLengthParser::new(ranges);
            let param = parser.parse(&from_inst).unwrap();

            assert_eq!(param[0], s);
            assert_eq!(param[1], t);
            assert_eq!(param[2], u);
        }
    }

    proptest! {
        #[test]
        fn parse_multiple_params_with_order_and_surroundings(
            s in string_of_length(5), t in string_of_length(3), u in string_of_length(7)
        ) {
            let from_inst = format!("12{t}34{u}567{s}");

            let ranges = vec![RangeOfChar::new_unchecked(17, 21), RangeOfChar::new_unchecked(2, 4), RangeOfChar::new_unchecked(7, 13)];

            let parser = StringCharLengthParser::new(ranges);
            let param = parser.parse(&from_inst).unwrap();

            assert_eq!(param[0], s);
            assert_eq!(param[1], t);
            assert_eq!(param[2], u);
        }
    }
}

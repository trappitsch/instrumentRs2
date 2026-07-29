/// Delimiter struct that holds the parts surrounding the arguments in a command string.
use crate::errors::InstrumentError;

#[derive(Debug, Default)]
pub struct Delimiter {
    /// Part that comes before the first argument.
    pub before: String,
    /// All the parts that come after each arguments.
    pub after: Vec<String>,
}

/// This is a delimited parser.
///
/// It contains the delimiters and, optionally, sort keys.
pub struct DelimitedParser {
    delimiter: Delimiter,
    sort_index: Option<Vec<usize>>,
}

impl DelimitedParser {
    /// Create a new parser from components.
    pub fn new(before: String, after: Vec<String>, sort_index: Option<Vec<usize>>) -> Self {
        Self {
            delimiter: Delimiter { before, after },
            sort_index,
        }
    }

    /// Parse an instrument return value with a given command structure.
    ///
    /// Return:
    /// Result of a `Vec<String>` with the ordered parameters of this command.
    ///
    /// Error:
    /// - Returns a `InstrumentError::BadInstrumentResponseString` if the parsing fails.
    pub fn parse(&self, value: &str) -> Result<Vec<String>, InstrumentError> {
        // take 'before' off and keep rest
        let mut rest = value
            .trim()
            .split_once(&self.delimiter.before)
            .ok_or(InstrumentError::BadInstrumentResponseString {
                msg: value.to_string(),
            })?
            .1;

        let params: Result<Vec<String>, InstrumentError> = self
            .delimiter
            .after
            .iter()
            .map(|d| {
                if d.is_empty() {
                    // we reached the end and there's no more delimiter!
                    Ok(rest.to_string())
                } else {
                    // we have not reached the end or there's a delim after end.
                    let res =
                        rest.split_once(d)
                            .ok_or(InstrumentError::BadInstrumentResponseString {
                                msg: value.to_string(),
                            });
                    match res {
                        Ok((p, r)) => {
                            rest = r;
                            Ok(p.to_string())
                        }
                        Err(e) => Err(e),
                    }
                }
            })
            .collect();

        if let Some(ind) = &self.sort_index
            && let Ok(vec) = params
        {
            // NOTE:
            // Indexing into `vec[*i]` here should be safe as the bracket parser ensures that:
            // 1. The sort indexes have the same length as the number of placeholders.
            // 2. The sort indexes are actually indexes.
            // TODO: Can we avoid this clone below?
            let params_sorted: Vec<String> = ind.iter().map(|i| vec[*i].clone()).collect();
            return Ok(params_sorted);
        };

        params
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_command_parser() {
        // Command: "{}"
        let dp = DelimitedParser::new("".to_string(), vec!["".to_string()], None);

        assert_eq!(dp.parse("ASDF").unwrap(), ["ASDF"]);
    }

    #[test]
    fn single_command_parser_with_start_and_end() {
        // Command: "START{}END"
        let dp = DelimitedParser::new("START".to_string(), vec!["END".to_string()], None);

        assert_eq!(dp.parse("STARTASDFEND").unwrap(), ["ASDF"]);
    }

    #[test]
    fn multiple_command_parser() {
        // Command: "CMD {},{},{},{}"
        let dp = DelimitedParser::new(
            "CMD ".to_string(),
            vec![
                ",".to_string(),
                ",".to_string(),
                ",".to_string(),
                "".to_string(),
            ],
            None,
        );

        assert_eq!(dp.parse("CMD 1,2,3,4").unwrap(), ["1", "2", "3", "4"]);
    }

    #[test]
    fn ensure_whitespace_is_trimmed() {
        // Command: "CMD {},{},{},{}"
        let dp = DelimitedParser::new(
            "CMD ".to_string(),
            vec![
                ",".to_string(),
                ",".to_string(),
                ",".to_string(),
                "".to_string(),
            ],
            None,
        );

        assert_eq!(dp.parse("CMD 1,2,3,4\n").unwrap(), ["1", "2", "3", "4"]);
    }

    #[test]
    fn multiple_command_parse_with_order() {
        // Command: "CMD {2},{1},{0},{3}"
        let dp = DelimitedParser::new(
            "CMD ".to_string(),
            vec![
                ",".to_string(),
                ",".to_string(),
                ",".to_string(),
                "".to_string(),
            ],
            Some(vec![2, 1, 0, 3]),
        );

        assert_eq!(dp.parse("CMD 3,2,1,4\n").unwrap(), ["1", "2", "3", "4"]);
    }
}

use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

fn main() {
    // ordered placeholders not allowed in enum
    #[derive(Parameter)]
    #[cmd("{0}")]
    enum Example1 {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }

    // invalid placeholder
    #[derive(Parameter)]
    #[cmd("{X}")]
    enum Example2 {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }

    // too many placeholders
    #[derive(Parameter)]
    #[cmd("{} {}")]
    enum Example3 {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }

    // no placeholder
    #[derive(Parameter)]
    #[cmd("none")]
    enum Example4 {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }
}

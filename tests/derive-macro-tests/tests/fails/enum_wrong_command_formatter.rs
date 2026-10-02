use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

fn main() {
    #[derive(Parameter)]
    #[cmd("no placeholder")]
    enum Example {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }

    #[derive(Parameter)]
    #[cmd("two {} placeholders {}")]
    enum Example2 {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
        #[param("3")]
        Item3,
    }
}

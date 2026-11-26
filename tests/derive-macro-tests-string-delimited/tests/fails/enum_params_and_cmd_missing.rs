use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

fn main() {
    #[derive(Parameter)]
    enum Example {
        Item1,
        Item2,
        Item3,
    }
}

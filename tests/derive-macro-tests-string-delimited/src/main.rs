use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

#[derive(Debug, Parameter)]
#[cmd("S({})")]
enum MyEnum {
    #[param("0")]
    Item1,
    #[param("1")]
    Item2,
}

#[derive(Debug, Parameter)]
#[cmd("Something {}")]
pub struct MyStruct {
    one: MyEnum,
}

fn main() {}

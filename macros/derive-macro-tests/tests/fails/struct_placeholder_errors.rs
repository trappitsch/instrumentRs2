use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

fn main() {
    #[derive(Parameter)]
    #[cmd("{}")]
    enum MyEnum {
        #[param("1")]
        Item1,
        #[param("2")]
        Item2,
    }

    // no placeholder
    #[derive(Parameter)]
    #[cmd("")]
    struct MyStruct1 {
        struct_item_1: MyEnum,
    }

    // too many placeholders
    #[derive(Parameter)]
    #[cmd("{} {}")]
    struct MyStruct2 {
        struct_item_1: MyEnum,
    }

    // too few placeholders
    #[derive(Parameter)]
    #[cmd("{} {}")]
    struct MyStruct3 {
        struct_item_1: MyEnum,
        struct_item_2: MyEnum,
        struct_item_3: MyEnum,
    }

    // used ordered and unordered placeholders
    #[derive(Parameter)]
    #[cmd("{} {0} {}")]
    struct MyStruct4 {
        struct_item_1: MyEnum,
        struct_item_2: MyEnum,
        struct_item_3: MyEnum,
    }
}

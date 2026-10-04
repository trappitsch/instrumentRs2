use instrumentrs_macros::{__instrument_parameter, Parameter};

__instrument_parameter!();

#[derive(Debug, Parameter)]
#[cmd("command {}")]
enum MyEnum {
    #[param("a")]
    Item1,
    #[param("b")]
    Item2,
    #[param("c")]
    Item3,
}

#[test]
fn to_writable() {
    assert_eq!(MyEnum::Item1.to_writable(), "command a");
    assert_eq!(MyEnum::Item2.to_writable(), "command b");
    assert_eq!(MyEnum::Item3.to_writable(), "command c");
}

#[test]
fn try_from_writable() {
    std::assert_matches!(
        MyEnum::try_from_writable("command a".to_string()).unwrap(),
        MyEnum::Item1
    );
    std::assert_matches!(
        MyEnum::try_from_writable("command b".to_string()).unwrap(),
        MyEnum::Item2
    );
    std::assert_matches!(
        MyEnum::try_from_writable("command c".to_string()).unwrap(),
        MyEnum::Item3
    );
}

#[test]
fn try_from_writable_with_eol() {
    std::assert_matches!(
        MyEnum::try_from_writable("command a\n".to_string()).unwrap(),
        MyEnum::Item1
    );
    std::assert_matches!(
        MyEnum::try_from_writable("command b\n".to_string()).unwrap(),
        MyEnum::Item2
    );
    std::assert_matches!(
        MyEnum::try_from_writable("command c\n".to_string()).unwrap(),
        MyEnum::Item3
    );
}

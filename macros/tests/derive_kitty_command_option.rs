use kitty_remote_bindings::ToArg;
use kitty_remote_bindings_macros::KittyCommandOption;
use pretty_assertions::assert_eq;

#[test]
fn test_enum() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        Value1,
    }

    assert_eq!(Option::Value1.to_arg(), "value1".to_string());
}

#[test]
fn test_enum_prefix() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        Value1(u8),
    }

    assert_eq!(Option::Value1(8).to_arg(), "value1:8".to_string());
}

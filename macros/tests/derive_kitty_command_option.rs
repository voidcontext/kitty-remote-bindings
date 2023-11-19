use kitty_remote_bindings_core::ToArg;
use kitty_remote_bindings_macros::KittyCommandOption;
use pretty_assertions::assert_eq;

#[test]
fn test_enum() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        Value,
    }

    assert_eq!(Option::Value.to_arg(), vec!["value".to_string()]);
}

#[test]
fn test_enum_case_variant_converted_to_param_case() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        ThisIsSomeValue,
    }

    assert_eq!(
        Option::ThisIsSomeValue.to_arg(),
        vec!["this-is-some-value".to_string()]
    );
}

#[test]
fn test_enum_unwraps_value() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        Value(String),
    }

    assert_eq!(
        Option::Value("some-value".to_string()).to_arg(),
        vec!["some-value".to_string()]
    );
}

#[test]
fn test_enum_prefix() {
    #[derive(KittyCommandOption, PartialEq)]
    enum Option {
        #[prefix]
        Value(u8),
    }

    assert_eq!(Option::Value(8).to_arg(), vec!["value:8".to_string()]);
}

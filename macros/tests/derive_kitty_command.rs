use kitty_remote_bindings_macros::{KittyCommand, KittyCommandOption};
use pretty_assertions::assert_eq;

#[derive(Debug, PartialEq, KittyCommandOption)]
struct StringOption(String);

#[test]
fn test_derive_kitty_command_creates_constructor_no_options() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand;

    let cmd = TestCommand::new();

    assert_eq!(cmd, TestCommand);
}

#[test]
fn test_derive_kitty_command_creates_constructor_optional_options() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand {
        string_option: Option<StringOption>,
    }

    let cmd = TestCommand::new();

    assert_eq!(
        cmd,
        TestCommand {
            string_option: None
        }
    );
}

#[derive(Debug, KittyCommand, PartialEq)]
#[kitty_command = "test-command"]
struct TestCommand {
    text: String,
    option_str: Option<StringOption>,
}

#[test]
fn test_derive_kitty_command_creates_constructor_required_options() {
    let cmd = TestCommand::new("some-text".to_string());
    assert_eq!(
        cmd,
        TestCommand {
            text: "some-text".to_string(),
            option_str: None
        }
    );
}

#[test]
fn test_derive_kitty_command_creates_setter_for_required_field() {
    let cmd = TestCommand::new("some-text".to_string()).text("other-text".to_string());
    assert_eq!(
        cmd,
        TestCommand {
            text: "other-text".to_string(),
            option_str: None
        }
    );
}

#[test]
fn test_derive_kitty_command_creates_setter_for_optional_field() {
    let cmd = TestCommand::new("some-text".to_string())
        .option_str(StringOption("lorem-ipsum".to_string()));
    assert_eq!(
        cmd,
        TestCommand {
            text: "some-text".to_string(),
            option_str: Some(StringOption("lorem-ipsum".to_string()))
        }
    );
}

#[test]
fn test_derive_kitty_command_implements_from_kitty_command_for_std_command() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand;

    let cmd = TestCommand::new();
    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec!["@", "test-command"]
    );
}

#[test]
fn test_derive_kitty_command_from_kitty_command_for_std_command_positional_args() {
    let cmd = TestCommand::new("some text".to_string());

    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec!["@", "test-command", "some text"]
    );
}

#[test]
fn test_derive_kitty_command_from_kitty_command_for_std_command_options() {
    let cmd =
        TestCommand::new("some text".to_string()).option_str(StringOption("value1".to_string()));

    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec!["@", "test-command", "--option-str", "value1", "some text"]
    );
}

#[test]
fn test_derive_kitty_command_from_kitty_command_for_std_command_option_can_be_renamed() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand {
        text: String,
        #[option = "foobar"]
        option_str: Option<StringOption>,
    }
    let cmd =
        TestCommand::new("some text".to_string()).option_str(StringOption("value1".to_string()));

    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec!["@", "test-command", "--foobar", "value1", "some text"]
    );
}

#[test]
fn test_derive_kitty_command_from_kitty_command_for_std_command_option_underscore_replaced() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand {
        option_str: Option<StringOption>,
    }
    let cmd = TestCommand::new().option_str(StringOption("value1".to_string()));

    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec!["@", "test-command", "--option-str", "value1"]
    );
}

#[test]
fn test_derive_kitty_command_from_kitty_command_for_std_argument_order_correct() {
    #[derive(Debug, KittyCommand, PartialEq)]
    #[kitty_command = "test-command"]
    struct TestCommand {
        text: String,
        option_str: Option<StringOption>,
        #[option = "foobar"]
        option_str2: Option<StringOption>,
        #[top_level]
        other_option: Option<String>,
    }
    let cmd = TestCommand::new("some text".to_string())
        .option_str(StringOption("value1".to_string()))
        .option_str2(StringOption("value2".to_string()))
        .other_option("foo".to_string());

    let std_cmd = std::process::Command::from(&cmd);

    assert_eq!(std_cmd.get_program(), "kitty");
    assert_eq!(
        std_cmd.get_args().collect::<Vec<_>>(),
        vec![
            "@",
            "--other-option",
            "foo",
            "test-command",
            "--option-str",
            "value1",
            "--foobar",
            "value2",
            "some text"
        ]
    );
}

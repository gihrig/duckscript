use super::*;
use crate::sdk::std::collections::array;
use crate::test;
use crate::test::{CommandValidation, SetCommand};

#[test]
fn common_functions() {
    test::test_common_command_functions(create(""));
}

#[test]
fn run_no_args() {
    test::run_script_and_error(vec![create("")], "out = is_array", "out");
}

#[test]
fn run_not_found() {
    test::run_script_and_validate(
        vec![create("")],
        "out = is_array bad_handle",
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_not_array() {
    test::run_script_and_validate(
        vec![create(""), Box::new(SetCommand {})],
        r#"
        handle = test_set true
        out = is_array ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "false".to_string()),
    );
}

#[test]
fn run_found() {
    test::run_script_and_validate(
        vec![create(""), array::create("")],
        r#"
        handle = array a b c "d e"
        out = is_array ${handle}
        "#,
        CommandValidation::Match("out".to_string(), "true".to_string()),
    );
}

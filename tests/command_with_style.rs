mod common; use common::*;
use xlsx_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--style-symbols"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_GROUPS_STYLE_SYMBOLS_LAYOUT_RECORDS));
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--style-controls"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_GROUPS_STYLE_CONTROLS_LAYOUT_RECORDS));
}

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--style-braces"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_GROUPS_STYLE_BRACES_LAYOUT_RECORDS));
}

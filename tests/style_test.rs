#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

#[test]
fn command_with_style_braces() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-braces"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_BRACES_GROUPS));
}

#[test]
fn command_with_style_controls() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-controls"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_CONTROLS_GROUPS));
}

#[test]
fn command_with_style_symbols() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-symbols"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_SYMBOLS_GROUPS));
}

#[test]
fn command_with_style_liners() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-liners"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_LINERS_GROUPS));
}

#[test]
fn command_with_style_sheets() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(command.arg("--style-sheets"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_SHEETS_GROUPS));
}

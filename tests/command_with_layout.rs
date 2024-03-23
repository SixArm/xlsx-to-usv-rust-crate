mod common; use common::*;
use xlsx_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_layout_0() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-0"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_0));
}

#[test]
fn command_with_layout_1() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-1"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_1));
}

#[test]
fn command_with_layout_2() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-2"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_2));
}

#[test]
fn command_with_layout_units() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-units"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_UNITS));
}

#[test]
fn command_with_layout_records() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-records"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_RECORDS));
}

#[test]
fn command_with_layout_groups() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command.arg("--layout-groups"), &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_GROUPS));
}

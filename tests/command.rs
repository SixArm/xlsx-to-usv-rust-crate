mod common; use common::*;
use xlsx_to_usv::examples::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(&mut command, &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_RECORDS));
}

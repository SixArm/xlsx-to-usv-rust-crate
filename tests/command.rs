mod common; use common::*;
use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(&mut command, &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_SYMBOLS_GROUPS));
}

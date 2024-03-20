#[path = "testing.rs"]
mod testing;
use testing::*;

#[path = "command_io.rs"]
mod command_io;
use command_io::*;

use std::process::Command;

#[test]
fn command() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io(&mut command, &EXAMPLE_XLSX_GROUPS);
    assert_eq!(actual, format!("{}\n", usv::examples::EXAMPLE_STYLE_SYMBOLS_GROUPS));
}

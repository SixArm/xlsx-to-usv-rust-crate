mod common; use common::*;
use xlsx_to_usv::examples::*;
use std::process::Command;

#[test]
fn command_with_custom_separators_with_short_options() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command
        .arg("-u").arg("{US}")
        .arg("-r").arg("{RS}")
        .arg("-g").arg("{GS}")
        .arg("-f").arg("{FS}")
        .arg("-e").arg("{ESC}")
        .arg("-z").arg("{EOT}")
        , &EXAMPLE_XLSX_GROUPS
    );
    assert_eq!(actual, format!("{}\n", EXAMPLE_GROUPS_STYLE_BRACES));
}

#[test]
fn command_with_custom_separators_with_long_options() {
    let mut command = Command::new(&*COMMAND_OS);
    let actual = command_io_bytes_to_string(command
        .arg("--us").arg("{US}")
        .arg("--rs").arg("{RS}")
        .arg("--gs").arg("{GS}")
        .arg("--fs").arg("{FS}")
        .arg("--esc").arg("{ESC}")
        .arg("--eot").arg("{EOT}")
        , &EXAMPLE_XLSX_GROUPS
    );
    assert_eq!(actual, format!("{}\n", EXAMPLE_GROUPS_STYLE_BRACES));
}

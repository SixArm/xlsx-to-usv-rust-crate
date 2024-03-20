//! Testing code for shared purposes

use ::std::ffi::OsString;
use std::path::{Path, PathBuf};
use once_cell::sync::Lazy;

#[allow(dead_code)]
pub static CARGO_MANIFEST_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR")].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TESTS_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tests"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static TMP_DIR: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "tmp"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_FILE: Lazy<PathBuf> = Lazy::new(||
    [env!("CARGO_MANIFEST_DIR"), "target", "debug", "xlsx-to-usv"].iter().collect::<PathBuf>()
);

#[allow(dead_code)]
pub static COMMAND_OS: Lazy<OsString> = Lazy::new(||
    OsString::from([env!("CARGO_MANIFEST_DIR"), "target", "debug", "xlsx-to-usv"].iter().collect::<PathBuf>())
);

#[allow(dead_code)]
pub static EXAMPLE_XLSX_GROUPS: Lazy<Vec<u8>> = Lazy::new(||
    std::fs::read(&TESTS_DIR.join("example1.xlsx")).expect("EXAMPLE_XLSX_GROUPS")
);

#[allow(dead_code)]
fn assert_str_contains(outer: &str, inner: &str) {
    assert!(
        outer.contains(inner),
        "outer: {:?}\n inner: {}\n", &outer, &inner
    );
}

#[allow(dead_code)]
pub fn remove_file_if_exists<P: AsRef<Path>>(path: P) -> std::io::Result<()>
{
    if path.as_ref().exists() {
        ::std::fs::remove_file(path)
    } else {
        Ok(())
    }
}

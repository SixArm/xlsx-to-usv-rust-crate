//! # xlsx-to-usv
//! 
//! Convert Microsoft Excel (XLSX) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).
//! 
//! Syntax:
//! 
//! ```sh
//! stdin | xlsx-to-usv [options] | stdout
//! ```
//! 
//! Example:
//! 
//! ```sh
//! cat example.xlsx | xlsx-to-usv
//! ```
//! 
//! More examples below.
//! 
//! ## Options
//! 
//! Options for USV separators and modifiers:
//! 
//! * -u, --unit-separator : Set the unit separator string.
//! 
//! * -r, --record-separator : Set the record separator string.
//! 
//! * -g, --group-separator : Set the group separator string.
//! 
//! * -f, --file-separator : Set the file separator string.
//! 
//! * --escape : Set the escape string.
//! 
//! * --end-of-transmission : Set the end-of-transmission string.
//! 
//! Options for USV style sets:
//! 
//! * --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.
//! 
//! * --style-controls : Set the style to use controls, such as "\u{001F}" for Unit Separator.
//! 
//! * --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.
//! 
//! * --style-liners : Set the style to use liners wrapping every symbol, such as "\n␟\n" for Unit Separator.
//! 
//! * --style-sheets : Set the style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator.
//! 
//! Options for command line tools:
//! 
//! * -h, --help : Print help
//! 
//! * -V, --version : Print version
//! 
//! * -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …
//! 
//! * --test : Print test output for debugging, verifying, tracing, and the like. Example: --test
//! 
//! ## Install
//! 
//! Install:
//! 
//! ```sh
//! cargo install xlsx-to-usv
//! ```
//! 
//! Link: [https://crates.io/crates/xlsx-to-usv](https://crates.io/crates/xlsx-to-usv)
//! 
//! ## Example
//! 
//! Suppose file example.xlsx contains this kind of data:
//! 
//! ```xlsx
//! Sheet 1
//! a,b
//! c,d
//! 
//! Sheet 2
//! d,e
//! f,g
//! ```
//! 
//! Run:
//! 
//! ```sh
//! cat example.xlsx | xlsx-to-usv --style-sheets
//! ```
//! 
//! Output:
//! 
//! ```usv
//! Sheet1␟␞
//! a␟b␟␞
//! c␟d␟␞
//! ␞
//! Sheet2␟␞
//! e␟f␟␞
//! g␟h␟␞
//! ␞
//! ```
//! 
//! ## FAQ
//! 
//! ### When to use this command?
//! 
//! Use this command when you want to convert from XLSX to USV.
//! 
//! A typical use case is when you have XLSX data, such as a spreadsheet file,
//! and you want to convert it to USV, such as to make the data easier to view
//! in a terminal, or edit in a text editor, or maintain in a text format.
//! 
//! Our real-world use case is converting a bunch of XLSX spreadsheet exports
//! from a variety of programs, including Excel, to USV so we're better-able to
//! handle quoting, and multi-line data units, and Unicode characters in a wide
//! variety of human languages.
//! 
//! ### Is there a similar command to convert from USV to XLSX?
//! 
//! Yes: [usv-to-xlsx](https://crates.io/crates/usv-to-xlsx).
//! 
//! ### Why use USV instead of XLSX?
//! 
//! See the documentation for [USV](https://github.com/sixarm/usv).
//! 
//! ### Is USV aiming to become a standard?
//! 
//! Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
//! [link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).
//! 
//! ## Help wanted
//! 
//! Constructive feedback welcome. Pull requests and feature requests welcome.
//! 
//! ## Tracking
//! 
//! * Package: xlsx-to-usv-rust-crate
//! * Version: 0.3.0
//! * Created: 2024-03-09T13:33:20Z
//! * Updated: 2024-03-21T14:05:26Z
//! * License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
//! * Contact: Joel Parker Henderson (joel@sixarm.com)

//// log
//#[macro_use]
extern crate log; use log::*;
extern crate env_logger;

use std::io::{Read, stdin};

pub mod app {
    pub mod args;
    pub mod clap;
    pub mod log;
}

fn main() -> std::io::Result<()> {
    trace!("main");
    let args: crate::app::args::Args = crate::app::clap::clap();
    if args.test { println!("{:?}", args); }
    let mut stdin = stdin().lock();
    let mut buf: Vec<u8> = Vec::new();
    stdin.read_to_end(&mut buf)?;
    let mut cursor = std::io::Cursor::new(buf);
    match xlsx_to_usv::xlsx_reader_to_usv(cursor, &args.style) {
        Ok(s) => {
            println!("{}", &s);
        },
        Err(e) => {
            error!("{e}");
            std::process::exit(1);
        }
    }
    Ok(())
}

//! clap setup.
//!
//! clap is a crate for command line argument parsing.
//! See https://docs.rs/clap/
//!
//! We prefer clap using the `command!` macro, which runs at compile time.
//! We prefer clap using the builder pattern, which offers more capabilities.
//!
//! We favor our convention of doing clap setup in a file named `clap.rs`,
//! rather than in `main.rs`, because we favor the separation of concerns.

use clap::Arg;
use usv::style::Style;

pub fn clap() -> crate::app::args::Args {
    let matches = clap::command!()
    .name("xlsx-to-usv")
    .version("1.2.0")
    .author("Joel Parker Henderson <joel@joelparkerhenderson.com>")
    .about("Convert Microsoft Excel (XLSX) to Unicode Separated Values (USV)")
    .arg(Arg::new("test")
        .long("test")
        .help("Print test output for debugging, verifying, tracing, and the like.\nExample: --test")
        .action(clap::ArgAction::SetTrue))
    .arg(Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace.\nExample: --verbose …")
        .action(clap::ArgAction::Count))
    .arg(Arg::new("unit-separator")
        .help("Set the unit separator string")
        .short('u')
        .long("unit-separator")
        .default_value(usv::UNIT_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("record-separator")
        .help("Set the record separator string")
        .short('r')
        .long("record-separator")
        .default_value(usv::RECORD_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("group-separator")
        .help("Set the group separator string")
        .short('g')
        .long("group-separator")
        .default_value(usv::GROUP_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("file-separator")
        .help("Set the file separator string")
        .short('f')
        .long("file-separator")
        .default_value(usv::FILE_SEPARATOR_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("escape")
        .help("Set the escape string")
        .long("escape")
        .default_value(usv::ESCAPE_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .arg(Arg::new("end-of-transmission")
        .help("Set the escape string")
        .long("end-of-transmission")
        .default_value(usv::END_OF_TRANSMISSION_SYMBOL_STR)
        .action(clap::ArgAction::Set))
    .get_matches();

    crate::app::args::Args {
        test: matches.get_flag("test"),
        log_level: crate::app::log::u8_to_log_level(matches.get_count("verbose")),
        style: Style {
            unit_separator: matches.get_one::<String>("unit-separator").expect("unit-separator").to_string(),
            record_separator: matches.get_one::<String>("record-separator").expect("record-separator").to_string(),
            group_separator: matches.get_one::<String>("group-separator").expect("group-separator").to_string(),
            file_separator: matches.get_one::<String>("file-separator").expect("file-separator").to_string(),
            escape: matches.get_one::<String>("escape").expect("escape").to_string(),
            end_of_transmission: matches.get_one::<String>("end-of-transmission").expect("end-of-transmission").to_string(),
        }
    }

}

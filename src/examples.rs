//! Examples of styles with USV units. These can be useful for demos and tests.

//// Style symbols

/// Example Style::symbols() with USV unit. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_UNIT: &str = "a␟";

/// Example Style::symbols() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_UNITS: &str = "a␟b␟";

/// Example Style::symbols() with USV record. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_RECORD: &str = "a␟b␟␞";

/// Example Style::symbols() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_RECORDS: &str = "a␟b␟␞c␟d␟␞";

/// Example Style::symbols() with USV group. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_GROUP: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝";

/// Example Style::symbols() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_GROUPS: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝Sheet2␟␞e␟f␟␞g␟h␟␞␝";

/// Example Style::symbols() with USV file. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_FILE: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝Sheet2␟␞e␟f␟␞g␟h␟␞␝␜";

/// Example Style::symbols() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_SYMBOLS_FILES: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝Sheet2␟␞e␟f␟␞g␟h␟␞␝␜Sheet1␟␞i␟j␟␞k␟l␟␞␝Sheet2␟␞m␟n␟␞o␟p␟␞␝␜";

//// Style controls

/// Example Style::controls() with USV unit. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_UNIT: &str = "a\u{001F}";

/// Example Style::controls() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_UNITS: &str = "a\u{001F}b\u{001F}";

/// Example Style::controls() with USV record. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_RECORD: &str = "a\u{001F}b\u{001F}\u{001E}";

/// Example Style::controls() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_RECORDS: &str = "a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}";

/// Example Style::controls() with USV group. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_GROUP: &str = "Sheet1\u{001F}\u{001E}a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}";

/// Example Style::controls() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_GROUPS: &str = "Sheet1\u{001F}\u{001E}a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}Sheet2\u{001F}\u{001E}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}";

/// Example Style::controls() with USV file. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_FILE: &str = "Sheet1\u{001F}\u{001E}a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}Sheet2\u{001F}\u{001E}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}";

/// Example Style::controls() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_CONTROLS_FILES: &str = "Sheet1\u{001F}\u{001E}a\u{001F}b\u{001F}\u{001E}c\u{001F}d\u{001F}\u{001E}\u{001D}Sheet2\u{001F}\u{001E}e\u{001F}f\u{001F}\u{001E}g\u{001F}h\u{001F}\u{001E}\u{001D}\u{001C}Sheet1\u{001F}\u{001E}i\u{001F}j\u{001F}\u{001E}k\u{001F}l\u{001F}\u{001E}Sheet2\u{001F}\u{001E}\u{001D}m\u{001F}n\u{001F}\u{001E}o\u{001F}p\u{001F}\u{001E}\u{001D}\u{001C}";

//// Style braces

/// Example Style::braces() with USV unit. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_UNIT: &str = "a{US}";

/// Example Style::braces() with USV units. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_UNITS: &str = "a{US}b{US}";

/// Example Style::braces() with USV record. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_RECORD: &str = "a{US}b{US}{RS}";

/// Example Style::braces() with USV records. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_RECORDS: &str = "a{US}b{US}{RS}c{US}d{US}{RS}";

/// Example Style::braces() with USV group. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_GROUP: &str = "Sheet1{US}{RS}a{US}b{US}{RS}c{US}d{US}{RS}{GS}";

/// Example Style::braces() with USV groups. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_GROUPS: &str = "Sheet1{US}{RS}a{US}b{US}{RS}c{US}d{US}{RS}{GS}Sheet2{US}{RS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}";

/// Example Style::braces() with USV file. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_FILE: &str = "Sheet1{US}{RS}a{US}b{US}{RS}c{US}d{US}{RS}{GS}Sheet2{US}{RS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}";

/// Example Style::braces() with USV files. This can be useful for demos and tests.
pub const EXAMPLE_STYLE_BRACES_FILES: &str = "Sheet1{US}{RS}a{US}b{US}{RS}c{US}d{US}{RS}{GS}Sheet2{US}{RS}e{US}f{US}{RS}g{US}h{US}{RS}{GS}{FS}Sheet1{US}{RS}i{US}j{US}{RS}k{US}l{US}{RS}{GS}Sheet2{US}{RS}m{US}n{US}{RS}o{US}p{US}{RS}{GS}{FS}";

//// Style tests

pub const EXAMPLE_STYLE_CONTROLS_GROUPS_AND_LAYOUT_RECORDS: &str = "Sheet1\u{001F}\u{001E}\na\u{001F}b\u{001F}\u{001E}\nc\u{001F}d\u{001F}\u{001E}\n\u{001D}\nSheet2\u{001F}\u{001E}\ne\u{001F}f\u{001F}\u{001E}\ng\u{001F}h\u{001F}\u{001E}\n\u{001D}\n";

pub const EXAMPLE_STYLE_BRACES_GROUPS_AND_LAYOUT_RECORDS: &str = "Sheet1{US}{RS}\na{US}b{US}{RS}\nc{US}d{US}{RS}\n{GS}\nSheet2{US}{RS}\ne{US}f{US}{RS}\ng{US}h{US}{RS}\n{GS}\n";

//// Layout tests

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_0: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝Sheet2␟␞e␟f␟␞g␟h␟␞␝";

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_1: &str = "Sheet1\n␟\n\n␞\na\n␟\nb\n␟\n\n␞\nc\n␟\nd\n␟\n\n␞\n\n␝\nSheet2\n␟\n\n␞\ne\n␟\nf\n␟\n\n␞\ng\n␟\nh\n␟\n\n␞\n\n␝\n";

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_2: &str = "Sheet1\n\n␟\n\n\n\n␞\n\na\n\n␟\n\nb\n\n␟\n\n\n\n␞\n\nc\n\n␟\n\nd\n\n␟\n\n\n\n␞\n\n\n\n␝\n\nSheet2\n\n␟\n\n\n\n␞\n\ne\n\n␟\n\nf\n\n␟\n\n\n\n␞\n\ng\n\n␟\n\nh\n\n␟\n\n\n\n␞\n\n\n\n␝\n\n";

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_UNITS: &str = "Sheet1␟\n␞\na␟\nb␟\n␞\nc␟\nd␟\n␞\n␝\nSheet2␟\n␞\ne␟\nf␟\n␞\ng␟\nh␟\n␞\n␝\n";

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_RECORDS: &str = "Sheet1␟␞\na␟b␟␞\nc␟d␟␞\n␝\nSheet2␟␞\ne␟f␟␞\ng␟h␟␞\n␝\n";

pub const EXAMPLE_STYLE_SYMBOLS_GROUPS_AND_LAYOUT_GROUPS: &str = "Sheet1␟␞a␟b␟␞c␟d␟␞␝\nSheet2␟␞e␟f␟␞g␟h␟␞␝\n";

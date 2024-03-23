# xlsx-to-usv

Convert Microsoft Excel (XLSX) to [Unicode Separated Values (USV)](https://github.com/sixarm/usv).

Syntax:

```sh
stdin | xlsx-to-usv [options] | stdout
```

Example:

```sh
cat example.xlsx | xlsx-to-usv
```

More examples below.

## Options

Options for USV separators and modifiers:

* -u, --us : Set the unit separator (US) string.

* -r, --rs : Set the record separator (RS) string.

* -g, --gs : Set the group separator (GS) string.

* -f, --fs : Set the file separator (FS) string.

* -e, --esc : Set the escape (ESC) string.

* -z, --eot : Set the end of transmission (EOT) string.

Options for USV style:

* --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.

* --style-controls : Set the style to use controls, such as "\u{001F}" for Unit Separator.

* --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.

Options for USV layout:

* --layout-0: Show each item with no line around it. This is no layout, in other words one long line.

* --layout-1: Show each item with one line around it. This is like single-space lines for long form text.

* --layout-2: Show each item with two lines around it. This is like double-space lines for long form text.

* --layout-units: Show each unit on one line. This can be helpful for line-oriented tools.

* --layout-records: Show each record on one line. This is like a typical spreadsheet sheet export.

* --layout-groups: Show each group on one line. This can be helpful for folio-oriented tools.

* --layout-files: Show one file on one line. This can be helpful for archive-oriented tools.

Options for command line tools:

* -h, --help : Print help

* -V, --version : Print version

* -v, --verbose... : Set the verbosity level: 0=none, 1=error, 2=warn, 3=info, 4=debug, 5=trace. Example: --verbose …

* --test : Print test output for debugging, verifying, tracing, and the like. Example: --test

## Install

Install:

```sh
cargo install xlsx-to-usv
```

Link: [https://crates.io/crates/xlsx-to-usv](https://crates.io/crates/xlsx-to-usv)

## Example

Excel and USV have similar data concepts:

| Excel     | USV    |
|-----------|--------|
| Workbook  | File   |
| Worksheet | Group  |
| Row       | Record |
| Cell      | Unit   |

Suppose file example.xlsx contains this kind of data:

```xlsx
Worksheet 1
a,b
c,d

Worksheet 2
d,e
f,g
```

Run:

```sh
cat example.xlsx | xlsx-to-usv
```

Output:

```usv
Worksheet 1␟␞
a␟b␟␞
c␟d␟␞
␝
Worksheet 2␟␞
e␟f␟␞
g␟h␟␞
␝
```

If you prefer ASCII Separated Values (ASV) with zero-width character controls:

Run:

```sh
cat example.xlsx | xlsx-to-usv --style-controls
```

Output:

```usv
Worksheet 1\u{001F}\u{001E}
a\u{001F}b\u{001F}\u{001E}
c\u{001F}d\u{001F}\u{001E}
\u{001D}
Worksheet 2\u{001F}\u{001E}
e\u{001F}f\u{001F}\u{001E}
g\u{001F}h\u{001F}\u{001E}
\u{001D}
```

If you prefer to render markers with braces, to see the markers more easily:

```sh
cat example.xlsx | xlsx-to-usv --style-braces
```

Output:

```usv
Worksheet 1{US}{RS}
a{US}b{US}{RS}
c{US}d{US}{RS}
{GS}
Worksheet 2{US}{RS}
e{US}f{US}{RS}
g{US}h{US}{RS}
{GS}
```

For more, see the official repository:<br> 
[Unicode Separated Values (USV)](https://github.com/sixarm/usv)

## FAQ

### When to use this command?

Use this command when you want to convert from XLSX to USV.

A typical use case is when you have XLSX data, such as a spreadsheet file,
and you want to convert it to USV, such as to make the data easier to view
in a terminal, or edit in a text editor, or maintain in a text format.

Our real-world use case is converting a bunch of XLSX spreadsheet exports
from a variety of programs, including Excel, to USV so we're better-able to
handle quoting, and multi-line data units, and Unicode characters in a wide
variety of human languages.

### Is there a similar command to convert from USV to XLSX?

Yes: [usv-to-xlsx](https://crates.io/crates/usv-to-xlsx).

### Is USV aiming to become a standard?

Yes, USV is submitted to IETF.org as an Internet-Draft work in progress:
[link](https://datatracker.ietf.org/doc/draft-unicode-separated-values/).

## Help wanted

Constructive feedback welcome. Pull requests and feature requests welcome.

## Tracking

* Package: xlsx-to-usv-rust-crate
* Version: 1.2.0
* Created: 2024-03-09T13:33:20Z
* Updated: 2024-03-21T14:05:26Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)

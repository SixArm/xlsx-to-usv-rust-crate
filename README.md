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

* -u, --unit-separator : Set the unit separator string.

* -r, --record-separator : Set the record separator string.

* -g, --group-separator : Set the group separator string.

* -f, --file-separator : Set the file separator string.

* --escape : Set the escape string.

* --end-of-transmission : Set the end-of-transmission string.

Options for USV style sets:

* --style-braces : Set the style to use braces, such as "{US}" for Unit Separator.

* --style-controls : Set the style to use controls, such as "\u{001F}" for Unit Separator.

* --style-symbols : Set the style to use symbols, such as "␟" for Unit Separator.

* --style-liners : Set the style to use liners wrapping every symbol, such as "\n␟\n" for Unit Separator.

* --style-sheets : Set the style similar to spreadsheet sheets, such as "␟" for Unit Separator and "␟\n" for Record Separator.

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
Worksheet 1\u001F\u001E
a\u001Fb\u001F\u001E
c\u001Fd\u001F\u001E
\u001D
Worksheet 2\u001F\u001E
e\u001Ff\u001F\u001E
g\u001Fh\u001F\u001E
\u001D
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
* Version: 0.3.0
* Created: 2024-03-09T13:33:20Z
* Updated: 2024-03-21T14:05:26Z
* License: MIT or Apache-2.0 or GPL-2.0 or GPL-3.0 or contact us for more
* Contact: Joel Parker Henderson (joel@sixarm.com)

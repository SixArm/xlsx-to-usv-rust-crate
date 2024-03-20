use std::path::Path;
use usv::style::Style;
use calamine::{
    open_workbook,
    open_workbook_from_rs,
    DataType,
    Error,
    Xlsx,
    Reader,
    RangeDeserializerBuilder
};

pub fn xlsx_file_to_usv<
    P: AsRef<Path> + Sized,
>(
    path: P,
    style: &Style,
) -> Result<String, Error> {
    let workbook: Xlsx<_> = open_workbook(path)?;
    xlsx_workbook_to_usv(workbook, style)
}

pub fn xlsx_reader_to_usv<
    RS: std::io::Read + std::io::Seek,
>(
    rs: RS,
    style: &Style,
) -> Result<String, Error> {
    let workbook: Xlsx<_> = open_workbook_from_rs(rs)?;
    xlsx_workbook_to_usv(workbook, style)
}

pub fn xlsx_workbook_to_usv<
    RS: std::io::Read + std::io::Seek,
>(
    mut workbook: Xlsx<RS>,
    style: &Style,
) -> Result<String, Error> {
    let worksheets = workbook.worksheets();
    let mut s = String::new();
    for worksheet in worksheets {
        s += &xlsx_worksheet_to_usv(worksheet, style)?;
        s += &style.group_separator;
    }
    Ok(s)
}

pub fn xlsx_worksheet_to_usv(
    worksheet: (String, calamine::Range<calamine::Data>),
    style: &Style,
) -> Result<String, Error> {
    let (name, range) = worksheet;
    let mut s = String::new();
    for row in range.rows() {
        for data in row {
            let unit = data.as_string().unwrap_or(String::from(""));
            s += &format!("{}{}", unit, style.unit_separator);
        }
        s += &style.record_separator;
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    // See ./tests for integration tests
}

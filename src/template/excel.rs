use std::collections::HashMap;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

pub fn parse_excel(path: &str, headers: &[&str]) -> Result<HashMap<usize, Vec<String>>, Error> {
    let mut result: HashMap<usize, Vec<String>> = HashMap::new();
    let mut work_book: Xlsx<_> = open_workbook(path)?;
    if let Some(Ok(ref range)) = work_book.worksheet_range("Sheet1") {
        let mut iter = RangeDeserializerBuilder::with_headers(headers).from_range(range)?;
        for sub in 1..range.height() {
            if let Some(row) = iter.next() {
                let cells: Vec<String> = row?;
                // 将解析的结果放入vec
                result.insert(sub, cells);
            } else {
                result.insert(sub, Vec::new());
            }
        }
    }
    Ok(result)
}

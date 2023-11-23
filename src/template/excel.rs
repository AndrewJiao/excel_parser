use std::collections::HashMap;
use std::hash::Hash;

use calamine::{Error, open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

pub fn parse_excel(path: &str, headers: &[&str]) -> Result<Vec<HashMap<String, String>>, Error> {
    let mut result: Vec<HashMap<String, String>> = Vec::new();
    let mut work_book: Xlsx<_> = open_workbook(path)?;
    if let Some(Ok(ref range)) = work_book.worksheet_range("Sheet1") {
        let mut iter = RangeDeserializerBuilder::with_headers(headers).from_range(range)?;
        for sub in 1..range.height() {
            if let Some(row) = iter.next() {
                let cells: Vec<String> = row?;
                // 将解析的结果放入vec
                result.push(contribute_head_value_map(cells, headers));
            } else {
                result.push(HashMap::new());
            }
        }
    }
    Ok(result)
}

fn contribute_head_value_map(mut cells: Vec<String>, headers: &[&str]) -> HashMap<String, String> {
    let mut sub = 0;
    cells.into_iter().map(|value| {
        let header = headers[sub].to_string();
        sub += 1;
        (header, value)
    }).collect()
}

use std::collections::HashMap;

use calamine::{DataType, open_workbook, RangeDeserializerBuilder, Reader, Xlsx};

use crate::template::model::Parser;

use super::template_trait::ParserError;

pub struct ExcelParser;


impl Parser for ExcelParser {
    fn do_parse(&self, path: &str, headers: &[&str]) -> Result<Vec<HashMap<String, String>>, ParserError> {
        let mut result: Vec<HashMap<String, String>> = Vec::new();
        let mut work_book: Xlsx<_> = open_workbook(path)?;
        if let Some(Ok(ref range)) = work_book.worksheet_range("Sheet1") {
            //过滤一下headers里面没有的字段
            let (_, len) = range.end().unwrap();
            let first_range = range.clone().range((0, 0), (0, len));

            let real_headers = first_range.rows().next().unwrap()
                .iter().filter_map(|e| {
                if let DataType::String(e) = e {
                    Some(e.as_str())
                } else {
                    None
                }
            }).filter(|e| headers.contains(e)).collect::<Vec<&str>>();

            let mut iter = RangeDeserializerBuilder::with_headers(&real_headers).from_range(range)?;
            for _sub in 1..range.height() {
                if let Some(row) = iter.next() {
                    let cells: Vec<String> = row?;
                    // 将解析的结果放入vec
                    println!("cells = {:?}", cells);
                    result.push(contribute_head_value_map(cells, &real_headers));
                } else {
                    result.push(HashMap::new());
                }
            }
        }
        Ok(result)
    }
}

fn contribute_head_value_map(cells: Vec<String>, headers: &[&str]) -> HashMap<String, String> {
    let mut sub = 0;
    cells.into_iter().map(|value| {
        let header = headers[sub].to_string();
        sub += 1;
        (header, value)
    }).collect()
}

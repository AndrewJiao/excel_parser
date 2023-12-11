extern crate core;

use std::collections::HashMap;
use std::env;
use std::error::Error;

use template::json_template;

use crate::template::excel::ExcelParser;
use crate::template::template_trait::Parser;

#[path = "../template/mod.rs"]
mod template;

pub fn main() {
    dotenv::dotenv().ok();
    let excel_path = env::var("EXCEL_SOURCE").expect("no source");
    let template_path = env::var("JSON_TEMPLATE_PATH").expect("no source");

    let mut pattern_model = json_template::parse(&template_path).unwrap();

    let patterns = pattern_model.get_all_template_value_key();
    let ref_patterns: Vec<&str> = patterns.iter().map(|e| e.as_str()).collect();

    let my_parser = ExcelParser;
    
    let parse_result: Vec<HashMap<String, String>> = my_parser.do_parse(&excel_path, ref_patterns.as_slice()).unwrap();

    pattern_model.replace_template_value(patterns, &parse_result);
    println!("value={:?}", pattern_model);
    println!("===============================");
    println!("to_json {:?}", pattern_model.to_json())
}
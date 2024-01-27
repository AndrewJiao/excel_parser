use std::collections::HashMap;
use std::env;

use excel_parser::output::OutPutJson;
use excel_parser::output::regex::FileType;
use excel_parser::template;
use excel_parser::template::excel::ExcelParser;
use excel_parser::template::model::{BaseModel, Parser};
use excel_parser::template::model::root_model::RootModel;

pub fn main() {
    dotenv::dotenv().ok();
    let excel_path = env::var("EXCEL_SOURCE").expect("no source");
    let template_path = env::var("JSON_TEMPLATE_PATH").expect("no source");

    let mut pattern_model: Box<RootModel> = template::json_template::parse(&template_path).unwrap();

    let patterns = pattern_model.get_all_template_value_key();

    let ref_patterns: Vec<&str> = patterns.iter().map(|e| e.as_str()).collect();

    let my_parser = ExcelParser;

    let parse_result: Vec<HashMap<String, String>> = my_parser.do_parse(&excel_path, ref_patterns.as_slice()).unwrap();

    pattern_model.replace_template_value(&patterns, &parse_result);
    let value = pattern_model.get_final_json_result();
    let mut out_put = OutPutJson::new("result", FileType::JSON);
    out_put.do_out_put(&value).expect("out put error");
    println!("end")
}
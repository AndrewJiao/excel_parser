use std::collections::HashMap;
use std::env::VarError;

use crate::output::regex::FileType;
use crate::output::OutPutJson;
use crate::template::excel::ExcelParser;
use crate::template::{Model, Parser};
use thiserror::Error;

pub mod output;
pub mod template;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("xsl error : {0}")]
    XslError(#[from] calamine::XlsxError),

    #[error("de error : {0}")]
    DeError(#[from] calamine::DeError),

    #[error("var error : {0}")]
    VarError(#[from] VarError),

    #[error("serde json error : {0}")]
    JsonError(#[from] serde_json::Error),
}

pub fn excel_to_json(
    excel_path: &str,
    template_path: &str,
    output_path: Option<&str>
) -> Result<String, ParserError> {
    //解析
    let mut pattern_model  = template::json_template::parse(&template_path).expect("parse template not success");

    //获取所有的keys
    let patterns = pattern_model.get_all_template_value_key();
    let ref_patterns: Vec<&str> = patterns.iter().map(|e| e.as_str()).collect();
    //解析excel
    let my_parser = ExcelParser;
    let parse_result: Vec<HashMap<String, String>> = my_parser.do_parse(&excel_path, ref_patterns.as_slice()).expect("parse excel not success");
    println!("parse result: {:?}", parse_result);
    pattern_model.replace_template_value(&patterns, &parse_result);

    println!("write result: {:?}", pattern_model.get_final_json_result());
    let value = pattern_model.get_final_json_result();
    let output_path = output_path.unwrap_or("result");
    let mut out_put = OutPutJson::new(output_path, FileType::JSON);
    out_put.do_out_put(&value).expect("out put error");
}

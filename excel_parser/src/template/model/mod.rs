use std::collections::HashMap;

use serde_json::Value;

pub use crate::template::model::array_model::ArrayModel;
pub use crate::template::model::object_model::ObjectModel;

pub mod root_model;
mod object_model;
mod array_model;
pub mod json_template;


pub trait Model {
    ///
    /// 获取所有的key
    ///
    fn get_all_template_value_key(&self) -> Vec<String>;

    ///
    /// 执行替换
    /// patterns,所有的替换key
    /// data, 需要替换的值map
    ///
    fn replace_template_value(&mut self, patterns: &[String], data: &[HashMap<String, String>]);
    ///
    /// 获取json解析后的结果
    ///
    fn get_final_json_result(&self) -> Value;
}

///
/// 用于存储表达式${?}
///
#[derive(Debug, Clone)]
pub struct ParseDescription {
    pub json_index: Vec<String>,
    pub pattern_type: String,
    pub pattern_key: String,
}

impl ParseDescription {
    pub fn new(json_index: String, pattern_type: String, pattern_value: String) -> Self {
        let json_index = vec![json_index];
        Self { json_index, pattern_type, pattern_key: pattern_value }
    }

    pub fn pattern_key(&self) -> &str {
        &self.pattern_key
    }


    pub fn put_description(&mut self, mut desc: ParseDescription) {
        self.json_index.push(desc.json_index.remove(0));
    }

    pub fn generate_value(&self, data: &str) -> Value {
        match self.pattern_type.as_str() {
            "num" => Value::from(data.parse::<i32>().unwrap()),
            "Number" => Value::from(data.parse::<i32>().unwrap()),
            "Num" => Value::from(data.parse::<i32>().unwrap()),
            "Boolean" => Value::from(data.parse::<bool>().unwrap()),
            "bool" => Value::from(data.parse::<bool>().unwrap()),
            "boolean" => Value::from(data.parse::<bool>().unwrap()),
            "double" => Value::from(data.parse::<f64>().unwrap()),
            "float" => Value::from(data.parse::<f32>().unwrap()),
            _ => Value::from(data.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModelType {
    Array(ArrayModel),
    Object(ObjectModel),
    None,
}

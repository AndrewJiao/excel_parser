use std::any::Any;
use std::collections::HashMap;

use serde_json::Value;
use crate::template::model::array_model::ArrayModel;
use crate::template::model::object_model::ObjectModel;

use crate::template::template_trait::ParserError;

pub mod root_model;
pub mod object_model;
pub mod array_model;


///
/// 有多种实现
///
pub trait Parser {
    ///
    ///
    /// path 路径
    /// header
    ///
    /// return 以key-value形式的键值对
    ///
    fn do_parse(&self, path: &str, header: &[&str]) -> Result<Vec<HashMap<String, String>>, ParserError>;
}

pub trait ToAny {
    fn to_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: BaseModel> ToAny for T {
    fn to_any(self: Box<Self <>>) -> Box<dyn Any> {
        self
    }
}


pub trait BaseModel: ToAny + Any {
    ///
    /// 批量获取
    ///
    fn get_all_template_value_key(&self) -> Vec<String>;

    ///
    /// 执行替换
    /// patterns,所有的替换key
    /// data, 需要替换的值map
    ///
    fn replace_template_value(&mut self, patterns: &Vec<String>, data: &Vec<HashMap<String, String>>);
    ///
    /// 获取替换后的json
    ///
    fn get_final_json_result(&self) -> Value;
}

///
/// 用于存储表达式${?}
///
#[derive(Debug)]
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

#[derive(Debug)]
pub enum ObjectType {
    Array(ArrayModel),
    Object(ObjectModel),
    None,
}

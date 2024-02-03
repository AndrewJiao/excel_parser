use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde_json::Value;

use crate::ParserError;
use crate::template::model::{json_template, Model, ModelType};

///
/// 数据都存在root,其他用引用
///
#[derive(Debug)]
pub struct RootModel {
    //内部用，用于复制用
    // json_template: OriginMapRef,
    //to_json_template的模板替换完值后往这里复制
    // json_result: Map<String, Value>,
    //
    sub_model: ModelType,
}

impl Model for RootModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        match &self.sub_model {
            ModelType::Array(value) => {
                value.get_all_template_value_key()
            }
            ModelType::Object(value) => {
                value.get_all_template_value_key()
            }
            _ => { vec![] }
        }
    }

    fn replace_template_value(&mut self, patterns: &[String], data: &[HashMap<String, String>]) {
        match &mut self.sub_model {
            ModelType::Array(value) => {
                value.replace_template_value(patterns, data)
            }
            ModelType::Object(value) => {
                value.replace_template_value(patterns, data)
            }
            _ => {}
        }
    }

    fn get_final_json_result(&self) -> Value {
        match &self.sub_model {
            ModelType::Array(value) => {
                value.get_final_json_result()
            }
            ModelType::Object(value) => {
                value.get_final_json_result()
            }
            _ => { Value::Null }
        }
    }
}

impl RootModel {
    ///开始解析
    // 1.正则解析${}
    // 2.初始化to_json
    pub fn parse(path: &str) -> Result<RootModel, ParserError> {
        let mut file = File::open(path).expect("unKnow file json template");
        let context = &mut String::new();
        file.read_to_string(context).expect("read file error");
        let model: RootModel = serde_json::from_slice::<Value>(context.as_bytes())?
            .into();
        Ok(model)
    }
}

impl From<Value> for RootModel {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(arr) => {
                let sub_model = arr.into_iter().filter_map(|e| {
                    if let Value::Object(map) = e {
                        json_template::try_extract_object_model("", map)
                    } else { None }
                }).collect::<Vec<_>>();
                RootModel { sub_model: ModelType::Array(sub_model.into()) }
            }
            Value::Object(obj) => {
                json_template::try_extract_object_model("", obj)
                    .map(|sub_model| RootModel { sub_model })
                    .unwrap_or_else(|| RootModel { sub_model: ModelType::None })
            }
            _ => unreachable!()
        }
    }
}


use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;

use serde_json::{Map, Value};

use crate::template::json_template;
use crate::template::model::{BaseModel, ObjectType};

pub type OriginMapRef = Rc<Map<String, Value>>;

///
/// 数据都存在root,其他用引用
///
pub struct RootModel {
    //内部用，用于复制用
    // json_template: OriginMapRef,
    //to_json_template的模板替换完值后往这里复制
    // json_result: Map<String, Value>,
    //
    sub_model: ObjectType,
}

impl BaseModel for RootModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        match &self.sub_model {
            ObjectType::Array(value) => {
                value.iter().flat_map(|e| e.get_all_template_value_key()).collect()
            }
            ObjectType::Object(value) => {
                value.get_all_template_value_key()
            }
            _ => {vec![]}
        }
    }

    fn replace_template_value(&mut self, patterns: &Vec<String>, data: &Vec<HashMap<String, String>>) {
        match &mut self.sub_model {
            ObjectType::Array(value) => {
                for sub in value {
                    sub.replace_template_value(patterns, data)
                }
            }
            ObjectType::Object(value) => {
                value.replace_template_value(patterns, data)
            }
            _ => {}
        }
    }


    fn get_final_json_result(&self) -> Value {
        match &self.sub_model {
            ObjectType::Array(value) => {
                let json = value.iter().map(|e| e.get_final_json_result()).collect::<Vec<Value>>();
                Value::Array(json)
            }
            ObjectType::Object(value) => {
                value.get_final_json_result()
            }
            _ => {Value::Null}
        }
    }
}

impl RootModel {
    ///开始解析
    // 1.正则解析${}
    // 2.初始化to_json
    pub fn parse(path: &str) -> Option<Box<RootModel>> {
        let mut file = File::open(path).expect("unKnow file json template");
        let context = &mut String::new();
        file.read_to_string(context).expect("read file error");
        match serde_json::from_slice(context.as_bytes()).unwrap() {
            Value::Object(obj) => {
                let origin_json = Rc::new(obj);
                if let Some(sub_model) = json_template::try_extract_object_model("", Rc::clone(&origin_json)) {
                    Some(Box::from(RootModel { sub_model: ObjectType::Object(sub_model) }))
                } else { None }
            }
            Value::Array(arr) => {
                let sub_model = arr.into_iter().filter_map(|e| {
                    if let Value::Object(map) = e {
                        let origin_json = Rc::new(map);
                        if let Some(sub_model) = json_template::try_extract_object_model("", Rc::clone(&origin_json)) {
                            Some(sub_model)
                        } else { None }
                    } else { None }
                }).collect::<Vec<_>>();
                Some(Box::from(RootModel { sub_model: ObjectType::Array(sub_model) }))
            }
            _ => None
        }
    }
}


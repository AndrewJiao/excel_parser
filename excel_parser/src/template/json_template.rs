use std::collections::HashMap;
use std::rc::Rc;

use regex::Regex;
use serde_json::{Map, Value};

use crate::template::model::{ObjectType, ParseDescription};
use crate::template::model::array_model::ArrayModel;
use crate::template::model::object_model::ObjectModel;
use crate::template::model::root_model::{OriginMapRef, RootModel};

pub fn parse(path: &str) -> Option<Box<RootModel>> {
    RootModel::parse(path)
}

///
/// 写一个递归方法负责递归json所有节点，提取所有${}
///
pub(crate) fn try_extract_object_model(parent_key: &str, origin_json: OriginMapRef) -> Option<ObjectType> {
    if origin_json.is_empty() { return None; }
    let mut res: HashMap<String, ParseDescription> = HashMap::new();
    let mut sub_base_vec: HashMap<String, ObjectType> = HashMap::new();

    for (current_key, value) in origin_json.iter() {
        let current_path: String;
        if parent_key.is_empty() {
            current_path = current_key.clone();
        } else {
            current_path = format!("{}.{}", parent_key, current_key);
        }
        match value {
            Value::Array(sub_json_array) => {
                let array_model: ArrayModel = sub_json_array.into_iter()
                    .filter_map(|e| {
                        if let Value::Object(sub_e) = e {
                            Some(sub_e)
                        } else {
                            None
                        }
                    })
                    .filter_map(|sub_e| {
                        let rc = safe_from_raw(sub_e);
                        try_extract_object_model("", rc)
                    })
                    .collect::<Vec<ObjectType>>().into();
                // sub_base_vec.insert(current_key.to_string(), ObjectType::Array(Box::from(array_model)))
                sub_base_vec.insert(current_key.to_string(), ObjectType::Array(array_model));
            }
            Value::Object(sub_json) => {
                let rc: Rc<Map<String, Value>>;
                unsafe {
                    rc = Rc::from_raw(sub_json);
                }
                if let Some(sub) = try_extract_object_model(&current_path, rc) {
                    // sub_base_vec.insert(current_path.to_string(), ObjectType::Object(Box::from(sub)));
                    sub_base_vec.insert(current_path.to_string(), sub);
                }
            }
            Value::String(ref maybe_pattern) => {
                if let Some(pattern) = extract(maybe_pattern).take() {
                    //找到值往集合加入
                    let description: ParseDescription = parse_util(pattern, current_path);

                    let pattern_key = description.pattern_key();
                    if let Some(descript) = res.get_mut(pattern_key) {
                        descript.put_description(description);
                    } else {
                        res.insert(pattern_key.to_string(), description);
                    }
                }
            }
            _ => {}
        }
    }
    Some(ObjectType::Object(ObjectModel {
        parser_index: res,
        json_template: origin_json,
        sub_model: sub_base_vec,
        json_result: None,
    }))
}

fn safe_from_raw(sub_e: &Map<String, Value>) -> Rc<Map<String, Value>> {
    let rc: Rc<Map<String, Value>>;
    unsafe {
        rc = Rc::from_raw(sub_e);
    }
    rc
}

///
/// 将模板字符串转为解析体ParseDescription
///
fn parse_util(pattern: String, json_index: String) -> ParseDescription {
    let description: Vec<&str> = pattern.split(":").collect();
    if description.len() < 1 {
        panic!("json pattern is not success for value {}", pattern)
    }
    if description.len() == 1 {
        ParseDescription::new(json_index, "String".to_string(), pattern)
    } else {
        let pattern_value = description.get(0).unwrap().to_string();
        let pattern_type = description.get(1).unwrap().to_string();
        ParseDescription::new(json_index, pattern_type, pattern_value)
    }
}

///
/// 提取模板自负床
///
fn extract(json: &str) -> Option<String> {
    let regex = Regex::new("(\\$\\{[\\S]+\\})").unwrap();

    if let Some(caps) = regex.captures(json) {
        // let caps = regex.captures(json).unwrap();
        let pattern = caps.get(0).unwrap().as_str().to_string();
        Some(pattern.replace("${", "").replace("}", ""))
    } else {
        None
    }
}





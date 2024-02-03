use std::collections::HashMap;

use regex::Regex;
use serde_json::{Map, Value};

use crate::ParserError;
use crate::template::model::{ArrayModel, ModelType, ObjectModel, ParseDescription};
use crate::template::model::root_model::RootModel;

pub fn parse(path: &str) -> Result<RootModel, ParserError> {
    RootModel::parse(path)
}

///
/// 写一个递归方法负责递归json所有节点，提取所有${}
///
pub(crate) fn try_extract_object_model(parent_key: &str, origin_json: Map<String, Value>) -> Option<ModelType> {
    let mut res: HashMap<String, ParseDescription> = HashMap::new();
    let mut sub_base_vec: HashMap<String, ModelType> = HashMap::new();

    let local_map: Map<String, Value> = origin_json.into_iter().filter_map(|(current_key, value)| {
        let current_path: String = if parent_key.is_empty() {
            current_key.clone()
        } else {
            format!("{}.{}", parent_key, current_key)
        };
        match value {
            Value::Array(sub_json_array) => {
                let array_model: ArrayModel = sub_json_array.into_iter()
                    .filter_map(|e| {
                        //目前只考虑array下面的obj
                        if let Value::Object(sub_e) = e {
                            Some(sub_e)
                        } else { None }
                    })
                    .filter_map(|sub_e| try_extract_object_model("", sub_e))
                    .collect::<Vec<ModelType>>().into();
                sub_base_vec.insert(current_key.to_string(), ModelType::Array(array_model));
                None
            }
            Value::Object(sub_json) => {
                if let Some(sub) = try_extract_object_model(&current_path, sub_json) {
                    sub_base_vec.insert(current_path.to_string(), sub);
                }
                None
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
                Some((current_key, value))
            }
            _ => { None }
        }
    }).collect();
    Some(ModelType::Object(ObjectModel {
        parser_index: res,
        json_template: local_map,
        sub_model: sub_base_vec,
        result: None,
    }))
}

///
/// 将模板字符串转为解析体ParseDescription
///
fn parse_util(pattern: String, json_index: String) -> ParseDescription {
    let description: Vec<&str> = pattern.split(':').collect();
    if description.is_empty() {
        panic!("json pattern is not success for value {}", pattern)
    }
    if description.len() == 1 {
        ParseDescription::new(json_index, "String".to_string(), pattern)
    } else {
        let pattern_value = description.first().unwrap().to_string();
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
        Some(pattern.replace("${", "").replace('}', ""))
    } else {
        None
    }
}






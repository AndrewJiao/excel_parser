use std::collections::HashMap;
use std::rc::Rc;

use serde_json::{Map, Value};

use crate::template::model::{BaseModel, ObjectType, ParseDescription};
use crate::template::model::root_model::OriginMapRef;

//解析json模板 用${}解析
#[derive(Debug,Clone)]
pub struct ObjectModel {
    //替换parse用的,存入${any}
    //key = pattern:${any:number} value = json_index
    //
    pub parser_index: HashMap<String, ParseDescription>,

    //内部用，用于复制用
    pub json_template: OriginMapRef,

    // 一个object有多个属性
    pub sub_model: HashMap<String, ObjectType>,

    pub result: Option<Map<String, Value>>,
}

impl BaseModel for ObjectModel {
    ///
    /// 获取所有要替换的template_key
    ///
    fn get_all_template_value_key(&self) -> Vec<String> {
        let mut current: Vec<String> = self.parser_index.iter().map(|(pattern, _)| pattern.to_string()).collect();
        let _ = &self.sub_model.iter().for_each(|(_, sub)| {
            match sub {
                ObjectType::Array(array) => {
                    let mut sub = array.get_all_template_value_key();
                    current.append(&mut sub);
                }
                ObjectType::Object(obj) => current.append(&mut obj.get_all_template_value_key()),
                ObjectType::None => {}
            }
        });
        current
    }

    ///
    ///按照顺序将解析的结果替换到模板上
    /// pattern:可以通过get_all_template_value_key方法获取
    ///
    fn replace_template_value(&mut self, patterns: &Vec<String>, data: &Vec<HashMap<String, String>>) {
        for head_value_map in data {
            let mut json_2_be_result = self.copy_json_template();
            self.do_replace(&mut json_2_be_result, &patterns, head_value_map);
            self.push_json_result(json_2_be_result)
        }
        //考虑sub的情况
        self.sub_model.iter_mut().for_each(|(_, value)| {
            match value {
                ObjectType::Array(arr) => {
                    arr.replace_template_value(patterns, data);
                }
                ObjectType::Object(obj) => { obj.replace_template_value(patterns, data); }
                ObjectType::None => {}
            }
        })
    }

    fn get_final_json_result(&self) -> Value {
        let clone_map: Option<Map<String, Value>> = self.result.clone();
        if let Some(mut map) = clone_map {
            //考虑sub的情况
            self.sub_model.iter().for_each(|(key, value)| {
                match value {
                    ObjectType::Array(arr) => {
                        let json_values = arr.get_final_json_result();
                        map.insert(key.to_string(), json_values);
                    }
                    ObjectType::Object(obj) => {
                        let json_value = obj.get_final_json_result();
                        map.insert(key.to_string(), json_value);
                    }
                    ObjectType::None => {}
                }
            });
            return Value::Object(map);
        } else {
            Value::Null
        }
    }
}

impl ObjectModel {
    ///执行替换
    fn do_replace(&mut self, json_line: &mut Map<String, Value>, patterns: &Vec<String>, data: &HashMap<String, String>) {
        for pattern in patterns {
            // println!("{},index = {:?}", pattern.to_string(),self.parser_index);
            if let Some(real_value) = data.get(pattern) {
                if let Some(parser_desc) = self.parser_index.get(pattern) {
                    parser_desc.json_index.iter().for_each(|json_index| {
                        let json_index_key: Vec<&str> = json_index.split(".").collect();
                        let mut json_value: Option<&mut Value> = None;
                        for key in json_index_key {
                            json_value = json_line.get_mut(key);
                        }
                        ObjectModel::do_set_value(json_value, parser_desc, real_value);
                    })
                }
            }
        }
    }
    ///
    /// 设置json的值
    /// 识别new_value的值的类型判断
    ///
    ///
    fn do_set_value(old_value: Option<&mut Value>, new_value: &ParseDescription, real_value: &str) {
        if let Some(json_value) = old_value {
            *json_value = new_value.generate_value(real_value);
        }
    }

    ///
    /// 复制一个json
    ///
    pub fn copy_json_template(&mut self) -> Map<String, Value> {
        let rc = &mut Rc::clone(&self.json_template);
        Rc::make_mut(rc).clone()
    }

    ///
    /// 没有就初始化一个vec
    ///
    ///
    pub fn push_json_result(&mut self, mut real_json: Map<String, Value>) {
        if let Some(ref mut map) = self.result {
            map.append(&mut real_json);
        } else {
            self.result = Some(real_json)
        }
    }

    pub fn get_sub_model_template_value_key(&self) -> Vec<String> {
        let mut current: Vec<String> = self.parser_index.iter().map(|(pattern, _)| pattern.to_string()).collect();
        // self.sub_model
        let mut sub: Vec<String> = self.sub_model.iter().flat_map(|(_, sub)| {
            match sub {
                ObjectType::Array(_) => {vec![]}
                ObjectType::Object(obj) => {obj.get_all_template_value_key()}
                ObjectType::None => {vec![]}
            }
        }).collect();
        current.append(&mut sub);
        current
    }
}

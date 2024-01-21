use std::collections::HashMap;
use std::rc::Rc;

use serde_json::{Map, Value};

use crate::template::model::{BaseModel, ParseDescription};
use crate::template::model::root_model::OriginMapRef;

//解析json模板 用${}解析
pub struct ObjectModel {
    //替换parse用的,存入${any}
    //key = pattern:${any:number} value = json_index
    //
    pub parser_index: HashMap<String, ParseDescription>,

    //内部用，用于复制用
    pub json_template: OriginMapRef,

    pub sub_model: Vec<Box<dyn BaseModel>>,

    pub json_result: Option<Map<String, Value>>,
}

impl BaseModel for ObjectModel {
    //
    // 获取所有要替换的template_key
    //
    fn get_all_template_value_key(&self) -> Vec<String> {
        let mut current: Vec<String> = self.parser_index.iter().map(|(pattern, _)| pattern.to_string()).collect();
        let mut sub: Vec<String> = self.sub_model.iter().flat_map(|sub| sub.get_all_template_value_key()).collect();
        current.append(&mut sub);
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
    }

    fn get_final_json_result(&self) -> Value {
        let clone_map = self.json_result.clone();
        Value::Object(clone_map.unwrap())
    }
}

impl ObjectModel {
    ///执行替换
    fn do_replace(&mut self, json_line: &mut Map<String, Value>, patterns: &Vec<String>, data: &HashMap<String, String>) {
        for pattern in patterns {
            if let Some(real_value) = data.get(pattern) {
                let parser_desc: &ParseDescription = self.parser_index.get(pattern).unwrap();
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
        if let Some(ref mut map) = self.json_result {
            map.append(&mut real_json);
        } else {
            self.json_result = Some(real_json)
        }
    }
}

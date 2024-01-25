use std::collections::HashMap;
use std::vec::IntoIter;

use serde_json::{Map, Value};

use crate::template::model::{BaseModel, ObjectType};

impl From<Vec<ObjectType>> for ArrayModel {
    fn from(value: Vec<ObjectType>) -> Self {
        if value.is_empty() {
            ArrayModel { sub_model_array: vec![] }
        } else {
            //其实这里需要考虑字符串数组的场景
            ArrayModel { sub_model_array: value }
        }
    }
}


#[derive(Debug)]
pub struct ArrayModel {
    // model_array: Vec<Box<ObjectModel>>,
    sub_model_array: Vec<ObjectType>,
}


impl BaseModel for ArrayModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        self.sub_model_array
            .iter()
            .flat_map(|sub| {
                match sub {
                    ObjectType::Array(arr) => {
                        arr.get_all_template_value_key()
                    }
                    ObjectType::Object(obj) => {
                        obj.get_all_template_value_key()
                    }
                    ObjectType::None => { vec![] }
                }
            }).collect()
    }

    ///
    /// 当数据是array时，array需要考虑又多少行数据，因为模板始终只有一行
    /// eq
    /// 模板
    /// ```json
    ///[
    ///  {
    ///    "id": "${id:num}",
    ///    "name": "${姓名}",
    ///    "age":"${年龄:num}",
    ///    "hello": "hello",
    ///    "age2": "${年龄}",
    ///        "sub":{
    ///            "sub_name":"${sub_name}",
    ///            "sub_age":"${sub.年龄}"
    ///        }
    ///    }
    ///]
    /// ```
    /// 上面的模板只有一行数据，但是如果转出的结果有5条的话
    /// 则array需要考虑clone五个模板给ObjectModel处理数据
    ///
    fn replace_template_value(&mut self, patterns: &Vec<String>, data: &Vec<HashMap<String, String>>) {
        for obj_type in self.sub_model_array.iter_mut() {
            match obj_type {
                ObjectType::Array(array) => {
                    array.replace_template_value(patterns, data);
                }
                ObjectType::Object(obj) => obj.replace_template_value(patterns, data),
                ObjectType::None => {}
            }
        }
    }

    fn get_final_json_result(&self) -> Value {
        let vec = self.sub_model_array.iter().map(|e| {
            match e {
                ObjectType::Array(array) => {
                    array.get_final_json_result()
                    // let json = array.iter().map(|e| e.get_final_json_result()).collect::<Vec<Value>>();
                    // Value::Array(json)
                }
                ObjectType::Object(obj) => obj.get_final_json_result(),
                ObjectType::None => Value::Null,
            }
        }).collect::<Vec<_>>();
        Value::Array(vec)
    }

}


impl ArrayModel {
    pub fn iter(&self) -> std::slice::Iter<'_, ObjectType> {
        self.sub_model_array.iter()
    }

    pub fn into_iter(self) -> IntoIter<ObjectType> {
        self.sub_model_array.into_iter()
    }

    fn get_sub_template(&self) -> Option<Map<String, Value>> {
        if let Some(sub) = self.sub_model_array.get(0) {
            match sub {
                ObjectType::Array(e) => { e.get_sub_template() }
                ObjectType::Object(e) => { Some(*(e.json_template.clone())) }
                ObjectType::None => { None }
            }
        } else {
            None
        }
    }
}



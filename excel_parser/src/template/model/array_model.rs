use std::collections::HashMap;
use serde_json::Value;

use crate::template::model::{Model, ModelType};

impl From<Vec<ModelType>> for ArrayModel {
    fn from(value: Vec<ModelType>) -> Self {
        if value.is_empty() {
            ArrayModel { sub_model_array: vec![] }
        } else {
            //其实这里需要考虑字符串数组的场景
            ArrayModel { sub_model_array: value }
        }
    }
}


#[derive(Debug, Clone)]
pub struct ArrayModel {
    sub_model_array: Vec<ModelType>,
}


impl Model for ArrayModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        self.sub_model_array
            .iter()
            .flat_map(|sub| {
                match sub {
                    ModelType::Array(arr) => {
                        arr.get_all_template_value_key()
                    }
                    ModelType::Object(obj) => {
                        obj.get_all_template_value_key()
                    }
                    ModelType::None => { vec![] }
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
    fn replace_template_value(&mut self, patterns: &[String], data: &[HashMap<String, String>]) {
        let mut new_data: Vec<ModelType> = vec![];
        for obj_type in self.sub_model_array.iter_mut() {
            match obj_type {
                ModelType::Array(array) => {
                    array.replace_template_value(patterns, data);
                }
                //这是excel中的数据行
                ModelType::Object(obj) => {
                    let group_key = obj.get_sub_model_template_value_key();
                    let group_values: Vec<Vec<HashMap<String, String>>> = group_by(&group_key, data);
                    //分组后的数据,note:分组后顺序会被打乱
                    for group_data in group_values {
                        //先clone，再插入
                        let mut new_obj = obj.clone();
                        new_obj.replace_template_value(patterns, &group_data);
                        new_data.push(ModelType::Object(new_obj));
                    }
                }
                ModelType::None => {}
            }
        }
        self.sub_model_array = new_data;
    }

    fn get_final_json_result(&self) -> Value {
        let vec = self.sub_model_array.iter().map(|e| {
            match e {
                ModelType::Array(array) => {
                    array.get_final_json_result()
                }
                ModelType::Object(obj) => obj.get_final_json_result(),
                ModelType::None => Value::Null,
            }
        }).collect::<Vec<_>>();
        Value::Array(vec)
    }
}

///
/// 根据给定的key，比较map里面的值，相同的分成一组
/// 需要保证顺序
///
fn group_by(group_keys: &Vec<String>, data: &[HashMap<String, String>]) -> Vec<Vec<HashMap<String, String>>> {
    //key=字段和，value=分组的序号
    let mut group_index: HashMap<String, usize> = HashMap::new();

    //定义一个group用来封装值
    let mut groups: Vec<Vec<HashMap<String, String>>> = Vec::new();
    let mut index: usize = 0;
    for entry in data {
        let mut key_builder = String::new();

        for key in group_keys {
            if let Some(value) = entry.get(key) {
                key_builder.push_str(value);
            } else {
                // Handle cases where the key is missing in the entry
                key_builder.push_str("missing");
            }
        }
        if let Some(&key_index) = group_index.get(&key_builder) {
            groups[key_index].push(entry.clone());
        } else if let std::collections::hash_map::Entry::Vacant(e) = group_index.entry(key_builder) {
            e.insert(index);
            groups.push(vec![entry.clone()]);
            index += 1;
        }
    }
    groups
}





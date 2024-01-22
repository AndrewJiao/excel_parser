use std::collections::HashMap;

use serde_json::Value;

use crate::template::model::{BaseModel, ObjectType};
use crate::template::model::object_model::ObjectModel;

impl From<Vec<Box<dyn BaseModel>>> for ArrayModel {
    fn from(value: Vec<Box<dyn BaseModel>>) -> Self {
        if value.is_empty() {
            ArrayModel { sub_model_array: ObjectType::None }
        } else {
            //其实这里需要考虑字符串数组的场景
            // let array = value.into_iter()
            //     .map(|e| {
            //         let x = e.to_any();
            //         x.downcast::<objectmodel>().unwrap()
            //     })
            //     .collect();
            ArrayModel {
                sub_model_array: ObjectType::Array(value)
            }
        }
    }
}


pub struct ArrayModel {
    // model_array: Vec<Box<ObjectModel>>,
    sub_model_array: ObjectType,
}

impl BaseModel for ArrayModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        match &self.sub_model_array {
            ObjectType::Array(array) => {
                array.iter().flat_map(|sub| sub.get_all_template_value_key()).collect()
            }
            ObjectType::Object(obj) => obj.get_all_template_value_key(),
            ObjectType::None => vec![],
        }
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
        match &mut self.sub_model_array {
            ObjectType::Array(array) => {
                match array.get(0) {
                    //list 里面没有元素不做处理
                    None => {}
                    Some(e) => {
                        let sub_patterns = e.get_all_template_value_key();
                        //获取这些子patterns相关的数据
                        let a_obj = e.to_any().downcast::<Box<ObjectModel>>()
                            .expect("this is not a obj ,down cast error");
                        let vec: Vec<String> = a_obj.get_sub_model_template_value_key();

                        for sub_model in array {
                            sub_model.replace_template_value(patterns, data);
                        }
                    }
                }
            }
            ObjectType::Object(obj) => obj.replace_template_value(patterns, data),
            ObjectType::None => {}
        }
    }

    fn get_final_json_result(&self) -> Value {
        match &self.sub_model_array {
            ObjectType::Array(array) => {
                let json = array.iter().map(|e| e.get_final_json_result()).collect::<Vec<Value>>();
                Value::Array(json)
            }
            ObjectType::Object(obj) => obj.get_final_json_result(),
            ObjectType::None => Value::Null,
        }
    }
}

use std::collections::HashMap;

use serde_json::Value;

use crate::template::model::BaseModel;
use crate::template::model::object_model::ObjectModel;

impl From<Vec<Box<dyn BaseModel>>> for ArrayModel {
    fn from(value: Vec<Box<dyn BaseModel>>) -> Self {
        //其实这里需要考虑字符串数组的场景
        let array = value.into_iter()
            .map(|e| {
                let x = e.to_any();
                x.downcast::<ObjectModel>().unwrap()
            })
            .collect();
        ArrayModel {
            model_array: array
        }
    }
}


pub struct ArrayModel {
    model_array: Vec<Box<ObjectModel>>,
}

impl BaseModel for ArrayModel {
    fn get_all_template_value_key(&self) -> Vec<String> {
        self.model_array.iter().flat_map(|sub| sub.get_all_template_value_key()).collect()
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
    /// 上面的模板只有一行数据，但是如果转出的结果又5条的话
    /// 则array需要考虑clone五个模板给ObjectModel处理数据
    ///
    fn replace_template_value(&mut self, patterns: &Vec<String>, data: &Vec<HashMap<String, String>>) {
        for sub_model in &mut self.model_array {
            sub_model.replace_template_value(patterns, data);
        }
    }

    fn get_final_json_result(&self) -> Value {
        let json = self.model_array.iter().map(|e| e.get_final_json_result()).collect::<Vec<Value>>();
        Value::Array(json)
    }
}

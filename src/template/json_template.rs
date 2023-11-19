use std::collections::{HashMap, HashSet};
use std::fs::{copy, File};
use std::io::Read;
use std::ops::Index;
use regex::Regex;
use serde_json::Value;
use serde_json::Map;

//解析json模板 用${}解析
pub struct BaseModel {
    origin_json: String,
    path: String,
    //替换parse用的,存入${any}
    parser_map: HashMap<String, String>,
    to_json: Vec<Map<String, Value>>,
}


pub fn parse(path: &str) -> BaseModel {
    BaseModel::parse(path)
}


impl BaseModel {
    ///开始解析
    // 1.正则解析${}
    // 2.初始化to_json
    fn parse(path: &str) -> Self {
        let mut file = File::open(path).expect("unKnow file json template");
        let context = &mut String::new();
        file.read_to_string(context);

        let json_value: Map<String, Value> = serde_json::from_slice(context.as_bytes()).unwrap();

        BaseModel::new(json_value, String::from(path))
    }

    //
    // 获取所有要替换的template_key
    //
    pub fn get_all_template_value_key(&self) -> Vec<String> {
        self.parser_map.iter().map(|(pattern, _)| pattern.to_string()).collect()
    }
    ///
    ///按照顺序将解析的结果替换到模板上
    /// pattern:可以通过get_all_template_value_key方法获取
    ///
    pub fn replace_template_value(&mut self, patterns: Vec<String>, data: HashMap<usize, Vec<String>>) {
        for (line, data) in data {
            let line_sub = line - 1;
            self.copy_json_template()
            let json_line = self.to_json.get(line_sub).unwrap();



        }
    }
    ///执行替换
    fn do_replace(&self,json_line: &Map<String, Value>, patterns: &Vec<String>) {
        for pattern in patterns {
            let json_index = self.parser_map.get(pattern).unwrap();
            let json_index_key: Vec<String> = json_index.split(".").collect();
            for ref key in json_index_key {
                json_line.get(key)
            }
        }



    }
    fn copy_json_template(&mut self) {
        if let Some(data) = self.to_json.get(self.to_json.len() - 1) {
            let new_data = data.clone();
            self.to_json.push(new_data);
        }
    }
    ///init
    fn new(value: Map<String, Value>, path: String) -> Self {
        let to_json = vec![value.clone()];

        let all_pattern: HashMap<String, String> = try_capture("", &value);
        BaseModel {
            origin_json: serde_json::to_string(&value).unwrap(),
            parser_map: all_pattern,
            path,
            to_json,
        }
    }
}


// 写一个递归方法负责递归json所有节点，提取所有${}
fn try_capture(parent_key: &str, json: &Map<String, Value>) -> HashMap<String, String> {
    if json.is_empty() {
        return HashMap::new();
    }
    let mut res: HashMap<String, String> = HashMap::new();
    for (current_key, value) in json {
        let mut current_path: String = "".to_string();
        if parent_key.is_empty() {
            current_path = current_key.clone();
        } else {
            current_path = format!("{}.{}", parent_key, current_key);
        }
        match value {
            Value::Object(sub_json) => {
                let sub_vec = try_capture(&current_path, sub_json);
                res.extend(sub_vec);
            }
            Value::String(ref maybe_pattern) => {
                if let Some(pattern) = extract(maybe_pattern).take() {
                    res.insert(pattern, current_path.to_string());
                }
            }
            _ => {}
        }
    }
    res
}

fn extract(json: &str) -> Option<String> {
    let regex = Regex::new("(\\$\\{[\\w]+\\})").unwrap();

    if let Some(caps) = regex.captures(json) {
        let caps = regex.captures(json).unwrap();
        let pattern = caps.get(0).unwrap().as_str().to_string();
        Some(pattern.replace("${", "").replace("}", ""))
    } else {
        None
    }
}


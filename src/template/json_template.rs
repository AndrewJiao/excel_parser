use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use serde_json::Map;
use serde_json::Value;

pub fn parse(path: &str) -> RootModel {
    RootModel::parse(path)
}

///
/// 写一个递归方法负责递归json所有节点，提取所有${}
///
fn try_extract_object_model(parent_key: &str, template_json: &Map<String, Value>) -> Option<dyn BaseModel> {
    if template_json.is_empty() { None }
    let mut res: HashMap<String, ParseDescription> = HashMap::new();
    let mut sub_base:Vec<dyn BaseModel> = Vec::new();
    for (current_key, value) in template_json {
        let mut current_path: String = "".to_string();
        if parent_key.is_empty() {
            current_path = current_key.clone();
        } else {
            current_path = format!("{}.{}", parent_key, current_key);
        }
        match value {
            Value::Array(sub_json_array)=>{
                for sub_json in sub_json_array {
                    match sub_json {
                        Value::Object(each_json) => {
                            if let Some(base) = try_extract_object_model("", each_json) {
                                sub_base.push(base);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Value::Object(sub_json) => {
                let sub_vec = try_extract_object_model(&current_path, sub_json);
                res.extend(sub_vec);
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
    Some(ObjectModel {
        parser_index :res,
        json_template: template_json,
        sub_model: sub_base,
    })
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
        let caps = regex.captures(json).unwrap();
        let pattern = caps.get(0).unwrap().as_str().to_string();
        Some(pattern.replace("${", "").replace("}", ""))
    } else {
        None
    }
}
//解析json模板 用${}解析
#[derive(Debug)]
pub struct ObjectModel<'a> {
    //替换parse用的,存入${any}
    //key = pattern:${any:number} value = json_index
    //
    parser_index: HashMap<String, ParseDescription>,

    //内部用，用于复制用
    json_template: &'a Map<String, Value>,

    sub_model: Vec<dyn BaseModel>,
}

impl BaseModel for ObjectModel{
    fn parse(path: &str) -> Self {
        todo!()
    }
}


#[derive(Debug)]
pub struct ArrayModel {
    model_array: Vec<ObjectModel>
}

impl BaseModel for ArrayModel{
    fn parse(path: &str) -> Self {
        todo!()
    }
}

///
/// 数据都存在root,其他用引用
/// 
pub  struct RootModel{
    //内部用，用于复制用
    json_template: Map<String, Value>,
    //to_json_template的模板替换完值后往这里复制
    to_json: Map<String, Value>,
    //
    sub_model: Vec<dyn BaseModel>,
    
}

impl BaseModel for RootModel{
    ///开始解析
    // 1.正则解析${}
    // 2.初始化to_json
    fn parse(path: &str) -> Self {
        let mut file = File::open(path).expect("unKnow file json template");
        let context = &mut String::new();
        file.read_to_string(context);

        let json_template: Map<String, Value> = serde_json::from_slice(context.as_bytes()).unwrap();
        
        if let Some(sub_model)= try_extract_object_model("", &json_template){
            RootModel{
                json_template,
                to_json: Map::new(),
                sub_model:vec![sub_model]
            }
        }else {
            panic!("un know value exception")
        }

    }
}
pub trait BaseModel {
    fn parse(path: &str) -> Self;
}

impl ObjectModel {

    //
    // 获取所有要替换的template_key
    //
    pub fn get_all_template_value_key(&self) -> Vec<String> {
        self.parser_index.iter().map(|(pattern, _)| pattern.to_string()).collect()
    }
    ///
    ///按照顺序将解析的结果替换到模板上
    /// pattern:可以通过get_all_template_value_key方法获取
    ///
    pub fn replace_template_value(&mut self, patterns: Vec<String>, data: &Vec<HashMap<String, String>>) {
        for (head_value_map) in data {
            let mut json_line = self.copy_json_template();
            self.do_replace(&mut json_line, &patterns, head_value_map);
            self.insert_data(json_line);
        }
    }
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
        self.json_template.clone()
    }
    ///init
    fn insert_data(&mut self, data: Map<String, Value>) {
        self.to_json.push(data);
    }
    pub fn to_json(&self) -> &Vec<Map<String, Value>> {
        &self.to_json
    }
}

///
/// 用于存储表达式${?}
/// 
#[derive(Debug)]
pub struct ParseDescription {
    json_index: Vec<String>,
    pattern_type: String,
    pattern_key: String,
}

impl ParseDescription {
    pub fn new(json_index: String, pattern_type: String, pattern_value: String) -> Self {
        let json_index = vec![json_index];
        Self { json_index, pattern_type, pattern_key: pattern_value }
    }
    pub fn pattern_key(&self) -> &str {
        &self.pattern_key
    }
    pub fn put_description(&mut self, mut desc: ParseDescription) {
        self.json_index.push(desc.json_index.remove(0));
    }
    pub fn generate_value(&self, data: &str) -> Value {
        match self.pattern_type.as_str() {
            "num" => Value::from(data.parse::<i32>().unwrap()),
            "Number" => Value::from(data.parse::<i32>().unwrap()),
            "Num" => Value::from(data.parse::<i32>().unwrap()),
            "Boolean" => Value::from(data.parse::<bool>().unwrap()),
            "bool" => Value::from(data.parse::<bool>().unwrap()),
            "boolean" => Value::from(data.parse::<bool>().unwrap()),
            "double" => Value::from(data.parse::<f64>().unwrap()),
            "float" => Value::from(data.parse::<f32>().unwrap()),
            _ => Value::from(data.to_string()),
        }
    }
}


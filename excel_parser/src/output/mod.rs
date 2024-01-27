use std::env;
use std::fs::File;
use std::io::Write;
use serde_json::Value;

use crate::output::regex::FileType;
use crate::ParserError;

pub mod regex;

pub struct OutPutJson {
    file: File,
}

impl Default for OutPutJson {
    fn default() -> Self {
        let mut file_path = env::var("OUT_PUT_FILE").expect("get value for out put file error");
        file_path.push('\\');
        let file_name = FileType::TXT.build_file_name(&file_path);
        OutPutJson {
            file: File::create(&file_name).unwrap_or_else(|_|panic!("create file error for file name {}", &file_name)),
        }
    }
}


impl OutPutJson {
    pub fn new(file_name: &str, file_type: FileType) -> Self {
        let mut file_path = env::var("OUT_PUT_FILE").expect("get value for out put file error");
        let file_name = file_type.build_file_name(file_name);
        file_path.push_str(&file_name);
        Self {
            file: File::create(file_path).unwrap_or_else(|_| panic!("create file error for file name {}", &file_name)),
        }
    }
    pub fn do_out_put(&mut self, value: &Value) -> Result<(), ParserError> {
        let json = serde_json::to_string_pretty(value)?;
        let _ = &self.file.write(json.as_bytes());
        Ok(())
    }
}

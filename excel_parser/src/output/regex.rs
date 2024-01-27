
#[derive()]
pub enum FileType {
    TXT,
    JSON,
    // XLSX,
    // XSL,
}

impl FileType {
    pub fn default_value(&self) -> String {
        match self {
            FileType::TXT => ".txt",
            FileType::JSON => ".json",
            // FileType::XLSX => ".xlsx",
            // FileType::XSL => ".xsl"
        }.to_string()
    }

    pub fn build_file_name(&self, file_name: &str) -> String {
        let mut default = String::new();
        default.push_str(file_name);
        let type_str = self.default_value();
        default.push_str(&type_str);
        default
    }
}



use std::collections::HashMap;

use crate::ParserError;
pub use model::Model;
pub use model::root_model::RootModel;

pub mod excel;
mod model;

pub use model::json_template;


///
/// 有多种实现
///
pub trait Parser {
    ///
    ///
    /// path 路径
    /// header
    ///
    /// return 以key-value形式的键值对
    ///
    fn do_parse(&self, path: &str, header: &[&str]) -> Result<Vec<HashMap<String, String>>, ParserError>;
}

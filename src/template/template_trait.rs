use std::collections::HashMap;
use std::fmt::{Debug, Display};

use thiserror::Error;

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


#[derive(Debug,Error)]
pub enum  ParserError{
    #[error("io error:{0}")]
    IoError(#[from] std::io::Error),

    #[error("xsl error : {0}")]
    XslError(#[from] calamine::XlsxError),

    #[error("de error : {0}")]
    DeError(#[from] calamine::DeError),
}

// impl From<calamine::Error> for  {
//     fn from(value: Error) -> Self {
//         value.
//
//     }
// }

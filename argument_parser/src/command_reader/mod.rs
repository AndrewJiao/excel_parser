use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// 构造一个结构体用于承载我的所有Command配置
/// 将来我将基于这些structs来生成命令行解析器
///
#[derive(Serialize, Deserialize, Debug)]
pub struct AutoCommand {
    name: String,
    about: Option<String>,
    subcommand_required: Option<bool>,
    subcommand_required_else_help: Option<bool>,
    subcommand: Option<String>,
    args: Option<Vec<AutoArgument>>,
    version: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AutoArgument {
    name: String,
    required: Option<bool>,
    num_args: Option<usize>,
    default_value: Option<String>,
    default_missing_value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllCommand {}

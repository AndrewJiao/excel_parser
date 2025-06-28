///
/// 构造一个结构体用于承载我的所有Command配置
/// 将来我将基于这些structs来生成命令行解析器
///
pub struct AutoCommand<'a> {
    name: &'a str,
    about: &'a str,
    subcommand_required: bool,
    subcommand_required_else_help: bool,
    subcommand: Option<&'a str>,
    // args: Vec<clap::Arg<'a>>,
    version: Option<&'a str>,
}

pub struct AutoArgument<'a> {
    name: &'a str,
    required: bool,
    num_args: Option<usize>,
    default_value: Option<&'a str>,
    default_missing_value: Option<&'a str>,
    value_parser: Option<fn(&str) -> Result<String, String>>,
}



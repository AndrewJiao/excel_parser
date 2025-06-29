use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

///
/// 构造一个结构体用于承载我的所有Command配置
/// 将来我将基于这些structs来生成命令行解析器
///
#[derive(Serialize, Deserialize, Debug)]
#[cfg(feature = "command_from_builder")]
pub struct AutoCommand {
    name: String,
    about: Option<String>,
    subcommand_required: Option<bool>,
    arg_required_else_help: Option<bool>,
    subcommand: Option<Vec<AutoCommand>>,
    args: Option<Vec<AutoArgument>>,
    version: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
#[cfg(feature = "command_from_builder")]
pub struct AutoArgument {
    name: String,
    required: Option<bool>,
    num_args: Option<usize>,
    default_value: Option<String>,
    default_missing_value: Option<String>,
}

///
/// 针对当前的命令类型创建命令
///
pub fn execute_base_commands(cmd: &HashMap<String, AutoCommand>) -> () {
    let mut cmd = Command::new("");
    for (name, command) in cmd {
        let command = execute_command(command);
    }
}

///
/// 针对当前的命令类型创建命令
///
///
#[cfg(feature = "command_from_builder")]
pub fn execute_command(cmd: &AutoCommand) -> Command {
    let mut command = Command::new(&cmd.name);
    if let Some(about) = &cmd.about {
        command = command.about(about);
    }
    if let Some(subcommand_required) = cmd.subcommand_required {
        command = command.subcommand_required(subcommand_required);
    }
    if let Some(arg_required_else_help) = cmd.arg_required_else_help {
        command = command.arg_required_else_help(arg_required_else_help);
    }
    if let Some(version) = &cmd.version {
        command = command.version(version);
    }

    if let Some(subcommands) = &cmd.subcommand {
        for subcommand in subcommands {
            let subcommand_matches = execute_command(subcommand);
            command = command.subcommand(subcommand_matches);
        }
    }
    if let Some(args) = &cmd.args {
        for arg in args {
            command = command.arg(execute_arg(arg));
        }
    }
    command
}

pub fn execute_arg(arg: &AutoArgument) -> Arg {
    let mut command_arg = Arg::new(&arg.name);

    if let Some(required) = arg.required {
        command_arg = command_arg.required(required);
    }

    if let Some(num_args) = arg.num_args {
        command_arg = command_arg.num_args(num_args);
    }

    if let Some(default_value) = &arg.default_value {
        command_arg = command_arg.default_value(default_value.to_string());
    }

    if let Some(default_missing_value) = &arg.default_missing_value {
        command_arg = command_arg.default_missing_value(default_missing_value.to_string());
    }

    command_arg
}

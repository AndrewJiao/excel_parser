use std::ops::RangeInclusive;

use clap::{Args, CommandFactory, Parser, Subcommand, ValueEnum};
use clap::error::ErrorKind;
use crate::command_from_parser::EnumSubCommand::Operation1;


///
/// 这是一个demo
///
pub fn do_test_parse_demo() {
    do_demo();
}

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "Kevin K. <kbknapp@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    value: EnumValue,
    #[command(flatten)]
    sub_flatten_value: SubFlattenValue,
    #[arg(long, short, required = false, default_value = "default_two")]
    two: String,
    #[arg(long, short, default_value = "32", value_parser = verify_arg_one)]
    one: u16,

    #[command(subcommand)]
    sub_command_enum: EnumSubCommand,
}

#[derive(ValueEnum, Clone, Debug)]
enum EnumValue {
    ValueA,
    ValueB,
}


#[derive(Args)]
#[group(required = false, multiple = false)]
struct SubFlattenValue {
    #[arg(long, value_name = "version")]
    sub_version: Option<String>,
    #[arg(long)]
    first: bool,
    #[arg(long)]
    second: bool,
    #[arg(long)]
    third: bool,
}

impl SubFlattenValue {
    fn get_version(&self) -> String {
        if let Some(ver) = &self.sub_version {
            ver
        } else {
            let mut cmd = Cli::command();
            let (a, b, c) = (self.first, self.second, self.third);
            match (a, b, c) {
                (true, false, false) => "1.0.0",
                (false, true, false) => "0.1.0",
                (false, false, true) => "0.0.1",
                (false, true, true) => "0.1.1",
                (true, false, true) => "1.0.1",
                (true, true, false) => "1.1.0",
                (true, true, true) => "1.1.1",
                (false, false, false) => cmd.error(ErrorKind::ArgumentConflict, "no that version can be choose").exit(),
            }
        }.to_string()
    }
}


#[derive(Subcommand)]
enum EnumSubCommand {
    Operation1 { name: Option<String> }
}

const PORT_RANGE: RangeInclusive<usize> = 1..=65535;

fn verify_arg_one(arg: &str) -> Result<u16, String> {
    let port: usize = arg.parse().map_err(|_| format!("`{arg}` isn't a port number"))?;
    if PORT_RANGE.contains(&port) {
        Ok(port as u16)
    } else {
        Err(format!(
            "port not in range {}-{}",
            PORT_RANGE.start(),
            PORT_RANGE.end()
        ))
    }
}

fn do_demo() {
    let cli = Cli::parse();

    // println!("name: {:?}", cli.name);
    println!("name: {:?}", cli.value);
    println!("version = {:?}", cli.sub_flatten_value.get_version());
    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
    match cli.sub_command_enum {
        Operation1 { name: file_name } => {
            println!("sub command file name = {:?}", file_name)
        }
    }
}

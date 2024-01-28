use clap::{Parser, Subcommand};
use crate::EnumSubCommand::Operation_1;

///
/// 用于解析参数
///
pub fn do_test_arg() {
    do_test();
}

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(author = "Kevin K. <kbknapp@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
struct Cli {
    pub name: Vec<String>,
    #[arg(long, short, default_value = "default_two")]
    two: String,
    #[arg(long, short, default_value = "default_one")]
    one: String,

    #[command(subcommand)]
    sub_command_enum: EnumSubCommand,
}

#[derive(Subcommand)]
enum EnumSubCommand {
    Operation_1 { name: Option<String> }
}



fn do_test() {
    let cli = Cli::parse();

    println!("name: {:?}", cli.name);
    println!("two: {:?}", cli.two);
    println!("one: {:?}", cli.one);
    match cli.sub_command_enum {
        Operation_1 { name: file_name } => {
            println!("sub command file name = {:?}", file_name)
        }
    }
}

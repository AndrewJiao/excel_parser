use argument_parser::command_from_builder::do_command;
use argument_parser::command_reader::AutoCommand;
use std::collections::HashMap;

fn main() {
    //读取配置
    dotenv::dotenv().ok();
    let yml_path = dotenv::var("BASIC_COMMAND_CONFIG")
        .expect("no ARGUMENT_STAR_CONFIG set, please set it in .env file");
    println!("reading yml config : {yml_path}");
    //读取yml_path指向的文件
    let pattern_model = std::fs::read_to_string(&yml_path).expect("Failed to read YAML file");
    // Deserialize from YAML
    let deserialized_point: HashMap<String,AutoCommand> = serde_yml::from_str(&pattern_model).expect("Failed to deserialize YAML");
    // 解析命令
    // command_reader::execute_command(&deserialized_point);
    println!("deserialized_point: {:#?}", deserialized_point);



    // do_test_arg()
    do_command()
}
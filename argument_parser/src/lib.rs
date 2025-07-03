use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::Mutex;
use clap::Command;

pub mod command_from_builder;
pub mod command_from_parser;
pub mod command_reader;

pub mod command_excel_to_json_parser;

type InnerCommand = Box<dyn FnOnce() -> Command + Send + Sync>;
type CommandList = Vec<InnerCommand>;
static COMMAND_LIST:Mutex<Vec<InnerCommand>> = Mutex::new( vec![]);

pub fn register_command(command: InnerCommand) {
    let mut guard = COMMAND_LIST.lock().unwrap();
    guard.push(command);
}

pub fn init_all_commands() {
    let guard = COMMAND_LIST.lock().unwrap();
    for command in guard.iter(){
        let real_command = command();
        real_command.get_matches();
    }
}

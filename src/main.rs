use std::env;
mod parsearguments;
use parsearguments::{
    parse_command_type
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_type = parse_command_type(args);
    println!("{:?}", command_type);
}
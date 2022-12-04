use std::env;
mod parsearguments;
use parsearguments::rn_get_command_type;
fn main() {
    let args: Vec<String> = env::args().collect();
    let command_type = rn_get_command_type(args);
    println!("{:?}", command_type);
}

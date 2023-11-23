mod parsearguments;
mod rntool;

use parsearguments::{rn_get_command_type, CommandType};
use rntool::RnTool;
use std::env::{self};

fn main() {
    let args: Vec<String> = env::args().collect();
    let command_type = rn_get_command_type(args);
    let homedir = dirs::home_dir().unwrap();

    // Current working directory is always /home/<user>/.rn/
    let working_directory = format!("{}/.rn/", homedir.as_os_str().to_str().unwrap());

    match command_type {
        CommandType::Error(x) => {
            panic!("Illegal Argument(s): {}", x)
        }
        ctype => {
            let rntool = RnTool::new(working_directory, ctype);

            if let Err(x) = rntool.execute_command() {
                panic!("Command failed: {}", x)
            }
        }
    }
}

use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};
use std::collections::BTreeSet;

use crate::parsearguments::CommandType;

pub struct RnTool {
    working_directory: String,
    command_type: CommandType,
}

impl RnTool {
    pub fn new(working_directory: String, command_type: CommandType) -> RnTool {
        match fs::create_dir_all(&working_directory) {
            Err(error) => {
                panic!(
                    "Problem with working directory {}, {}",
                    working_directory, error
                );
            }
            _ => {}
        }
        RnTool {
            working_directory,
            command_type,
        }
    }
    pub fn execute_command(&self) -> Result<(), String> {
        match &self.command_type {
            CommandType::ListNoteFiles => {
                let paths = fs::read_dir(&self.working_directory).unwrap();

                for path in paths {
                    println!("{}", path.unwrap().file_name().to_str().unwrap())
                }

                Ok(())
            },
            CommandType::RemoveNoteFile(file) => {
                match fs::remove_file(format!("{}{}", &self.working_directory, file)) {
                    Err(e) => Err(e.to_string()),
                    _ => Ok(()),
                }
            },
            CommandType::OpenNoteFileInEditor(file) => {
                edit::edit_file(format!("{}{}", &self.working_directory, file)).unwrap();
                Ok(())
            },
            CommandType::ListNotesInFile(file) => {
                let file = File::open(format!("{}{}", &self.working_directory, file)).unwrap();
                let reader = BufReader::new(file);

                let mut index: usize = 0;

                for line in reader.lines() {
                    println!("{}: {}", index, line.unwrap());
                    index += 1;
                }
                Ok(())
            },
            CommandType::AddNoteToFile(file, note) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(format!("{}{}", &self.working_directory, file))
                    .unwrap();

                if let Err(e) = writeln!(file, "{}", note) {
                    return Err(e.to_string());
                }

                Ok(())
            },
            CommandType::EditNoteInFile(file, line_number, new_content) => {
                let contents = fs::read_to_string(format!("{}{}", &self.working_directory, file)).expect("can't read");
                let mut lines: Vec<&str> = contents.lines().collect();
                for i in 0..lines.len() {
                    if i == *line_number {
                        lines[i] = new_content;
                        break;
                    }
                }

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(format!("{}{}", &self.working_directory, file))
                    .unwrap();
                
                for line in lines {
                    file.write_all(line.as_bytes()).unwrap();
                    file.write_all("\n".as_bytes()).unwrap();
                }

                Ok(())
            },
            CommandType::RemoveNoteFromFile(file, line_number) => {

                let contents = fs::read_to_string(format!("{}{}", &self.working_directory, file)).expect("can't read");
                let mut lines: Vec<&str> = contents.lines().collect();
                for i in 0..lines.len() {
                    if i == *line_number {
                        lines.remove(i);
                        break;
                    }
                }

                let mut file = OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(format!("{}{}", &self.working_directory, file))
                    .unwrap();
                
                for line in lines {
                    file.write_all(line.as_bytes()).unwrap();
                    file.write_all("\n".as_bytes()).unwrap();
                }

                Ok(())
            },
            CommandType::PrintHelp => {
                println!("This is a work in progress");
                Ok(())
            }
            CommandType::Error(error) => Err(error.clone()),
        }
    }
}

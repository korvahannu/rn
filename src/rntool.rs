use std::fs::{self, File, OpenOptions};
use std::io::{prelude::*, BufReader};

use crate::parsearguments::CommandType;

pub struct RnTool {
    working_directory: String,
    command_type: CommandType,
}

impl RnTool {
    pub fn new(working_directory: String, command_type: CommandType) -> RnTool {
        if let Err(error) = fs::create_dir_all(&working_directory) {
            panic!(
                "Problem with working directory {}, {}",
                working_directory, error
            );
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
            }
            CommandType::RemoveNoteFile(file) => {
                match fs::remove_file(format!("{}{}", &self.working_directory, file)) {
                    Err(e) => Err(e.to_string()),
                    _ => Ok(()),
                }
            }
            CommandType::OpenNoteFileInEditor(file) => {
                edit::edit_file(format!("{}{}", &self.working_directory, file)).unwrap();
                Ok(())
            }
            CommandType::ListNotesInFile(file) => {
                let file_result = File::open(format!("{}{}", &self.working_directory, file));

                let file: File = match file_result {
                    Ok(f) => {
                         f
                    }
                    Err(_) => {
                        return Err("File did not exist".to_string());
                    }
                };

                let reader = BufReader::new(file);

                let mut index: usize = 0;

                for line in reader.lines() {
                    println!("{}: {}", index, line.unwrap());
                    index += 1;
                }

                if index == 0 {
                    return Err("File was empty.".to_string());
                }

                Ok(())
            }
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
            }
            CommandType::EditNoteInFile(file, line_number, new_content) => {
                let content= match fs::read_to_string(format!("{}{}", &self.working_directory, file)) {
                    Ok(c) => {
                        c
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let mut lines: Vec<&str> = content.lines().collect();
                let mut was_line_edited: bool = false;
                for (i, _) in lines.iter_mut().enumerate() {
                    if i == *line_number {
                        lines[i] = new_content;
                        was_line_edited = true;
                        break;
                    }
                }

                if !was_line_edited {
                    return Err("Unable to find line that user wanted to edit.".to_string());
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
            }
            CommandType::RemoveNoteFromFile(file, line_number) => {
                let content: String = match fs::read_to_string(format!("{}{}", &self.working_directory, file)) {
                    Ok(c) => {
                        c
                    }
                    Err(e) => {
                        return Err(e.to_string());
                    }
                };

                let mut lines: Vec<&str> = content.lines().collect();
                let mut was_line_removed = false;

                for i in 0..lines.len() {
                    if i == *line_number {
                        lines.remove(i);
                        was_line_removed = true;
                        break;
                    }
                }

                if !was_line_removed {
                    return Err("Unable to remove line that user wanted to edit.".to_string());
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
            }
            CommandType::PrintHelp => {
                println!("rn is a simple utility for taking and storing notes using the command line.");
                println!("");
                println!("Show help:");
                println!("rn help");
                println!("rn h");
                println!("rn");
                println!("");
                println!("Add a new note:");
                println!("rn <filename> <note>");
                println!("rn <filename> add <note>");
                println!("rn <filename> a <note>");
                println!("");
                println!("List all notes:");
                println!("rn list");
                println!("rn l");
                println!("");
                println!("List all entries inside a note file:");
                println!("rn <filename> list");
                println!("rn <filename> l");
                println!("rn <filename>");
                println!("");
                println!("Remove an entry from a file:");
                println!("rn <filename> remove <index>");
                println!("rn <filename> r <index>");
                println!("");
                println!("Edit note:");
                println!("rn <filename> edit <index> <newtext>");
                println!("rn <filename> e <index> <newtext>");
                println!("");
                println!("Open note file in a text editor:");
                println!("rn open <filename>");
                println!("rn o <filename>");
                println!("");
                println!("Remove note file and all the notes inside it:");
                println!("rn remove <filename>");
                println!("rn r <filename>");
                Ok(())
            }
            CommandType::Error(error) => Err(error.clone()),
        }
    }
}

#[allow(unused_variables)]
mod rntool_tests {
    use crate::parsearguments::{rn_get_command_type, CommandType};
    use crate::RnTool;

    #[test]
    fn test_help_with_no_params() -> Result<(), String> {
        let result = get_tool(vec![String::from("rn")]).execute_command();
        return result;
    }

    #[test]
    fn test_help_with_help_param() -> Result<(), String> {
        qtest("help", "", "", "")
    }

    #[test]
    fn test_help_with_h_param() -> Result<(), String> {
        qtest("h", "", "", "")
    }

    #[test]
    fn test_list_with_list_param() -> Result<(), String> {
        qtest("list", "", "", "")
    }

    #[test]
    fn test_list_with_l_param() -> Result<(), String> {
        qtest("list", "", "", "")
    }

    #[test]
    fn test_add_new_note_no_params() -> Result<(), String> {
        qtest("addtests", "this is my note", "", "")
    }

    #[test]
    fn test_add_new_note_with_add_param() -> Result<(), String> {
        qtest("addtests", "add", "this is my note", "")
    }

    #[test]
    fn test_add_new_note_with_a_param() -> Result<(), String> {
        qtest("addtests", "a", "this is my note", "")
    }

    #[test]
    fn test_list_entries_in_notefile_with_no_params() -> Result<(), String> {
        qtest("listtest1", "a", "this is my note", "").unwrap();
        qtest("listtests", "", "", "")
    }

    #[test]
    fn test_list_entries_in_notefile_with_list_param() -> Result<(), String> {
        qtest("listtest2", "a", "this is my note", "").unwrap();
        qtest("listtest2", "list", "", "")
    }

    #[test]
    fn test_list_entries_in_notefile_with_l_param() -> Result<(), String> {
        qtest("listtest3", "a", "this is my note", "").unwrap();
        qtest("listtest3", "l", "", "")
    }

    #[test]
    fn test_remove_entry_from_notefile_with_remove_param() -> Result<(), String> {
        qtest("removetest1", "a", "this is my note", "").unwrap();
        qtest("removetest1", "r", "0", "")
    }

    #[test]
    fn test_remove_entry_from_notefile_with_r_param() -> Result<(), String> {
        qtest("removetest2", "a", "this is my note", "").unwrap();
        qtest("removetest2", "remove", "0", "")
    }

    #[test]
    fn test_remove_notefile_with_remove() -> Result<(), String> {
        qtest("removenotefiletest1", "a", "this is my note", "").unwrap();
        qtest("remove", "removenotefiletest1", "", "")
    }

    #[test]
    fn test_remove_notefile_with_r() -> Result<(), String> {
        qtest("removenotefiletest2", "a", "this is my note", "").unwrap();
        qtest("r", "removenotefiletest2", "", "")
    }

    #[test]
    fn test_edit_note_with_e() -> Result<(), String> {
        qtest("edittest1", "a", "this is my note", "").unwrap();
        qtest("edittest1", "e", "0", "This is my new note")
    }

    #[test]
    fn test_edit_note_with_edit() -> Result<(), String> {
        qtest("edittest2", "a", "this is my note", "").unwrap();
        qtest("edittest2", "edit", "0", "This is my new note")
    }

    #[test]
    fn test_remove_no_file() -> Result<(), String> {
        match qtest("remove", "nonexistingnotefile", "", "") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Removing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[test]
    fn test_edit_no_file() -> Result<(), String> {
        match qtest("nonexistingnotefile", "edit", "0", "this should not work") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Editing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[test]
    fn test_edit_invalid_line() -> Result<(), String> {
        qtest("invalidlinetest", "hey", "", "").unwrap();
        match qtest("invalidlinetest", "edit", "5", "this should not work") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Editing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[test]
    fn test_remove_invalid_file() -> Result<(), String> {
        match qtest("invalidlinetestremoval_doesnotexist", "remove", "5", "") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Editing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[test]
    fn test_remove_invalid_line() -> Result<(), String> {
        qtest("invalidlinetestremoval", "hey", "", "").unwrap();
        match qtest("invalidlinetestremoval", "remove", "5", "") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Editing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[test]
    fn test_list_invalid_file() -> Result<(), String> {
        match qtest("this_file_does_not_exist", "list", "", "") {
            Err(e) => {
                return Ok(());
            }
            _ => {
                return Err("Editing a non-existing notefile did not throw an error.".to_string());
            }
        }
    }

    #[allow(dead_code)]
    fn qtest(arg1: &str, arg2: &str, arg3: &str, arg4: &str) -> Result<(), String> {
        get_tool(fake_args(arg1, arg2, arg3, arg4)).execute_command()
    }

    #[allow(dead_code)]
    fn fake_args(arg1: &str, arg2: &str, arg3: &str, arg4: &str) -> Vec<String> {
        vec![
            "rn".to_string(),
            arg1.to_string(),
            arg2.to_string(),
            arg3.to_string(),
            arg4.to_string(),
        ]
    }

    #[allow(dead_code)]
    fn get_tool(args: Vec<String>) -> RnTool {
        let command_type = rn_get_command_type(args);
        let working_directory = String::from("./rn-tests-data/");

        match command_type {
            CommandType::Error(x) => {
                panic!("Illegal Argument(s): {}", x)
            }
            ctype => {
                RnTool::new(working_directory, ctype)
            }
        }
    }
}

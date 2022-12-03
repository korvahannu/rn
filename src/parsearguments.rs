#[derive(Debug)]
pub enum CommandType {
    ListNoteFiles,
    SearchNoteFile(String),
    RemoveNoteFile(String),
    OpenNoteFileInEditor(String),

    ListNotesInFile(String),
    SearchForNoteFromFile(String, String),
    AddNoteToFile(String, String),
    EditNoteInFile(String, usize),
    RemoveNoteFromFile(String, usize),
    PrintHelp,
    Error(String),
}

pub fn parse_command_type(args: Vec<String>) -> CommandType {
    let help_args: [String; 2] = [String::from("help"), String::from("h")];

    let list_args: [String; 2] = [String::from("list"), String::from("l")];

    let search_args: [String; 2] = [String::from("search"), String::from("s")];

    let remove_args: [String; 2] = [String::from("remove"), String::from("r")];

    let edit_args: [String; 2] = [String::from("edit"), String::from("e")];

    let open_args: [String; 2] = [String::from("open"), String::from("o")];

    let add_args: [String; 2] = [String::from("add"), String::from("a")];

    if args.len() == 1 {
        return CommandType::PrintHelp;
    }

    let first_argument = &args[1];

    if help_args.contains(first_argument) {
        return CommandType::PrintHelp;
    }

    if list_args.contains(first_argument) {
        return CommandType::ListNoteFiles;
    }

    if search_args.contains(first_argument) {
        if args.len() < 3 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let file_to_search_for = args[2].clone();
            return CommandType::SearchNoteFile(file_to_search_for);
        }
    }

    if remove_args.contains(first_argument) {
        if args.len() < 3 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let file_to_remove = args[2].clone();
            return CommandType::RemoveNoteFile(file_to_remove);
        }
    }

    if open_args.contains(first_argument) {
        if args.len() < 3 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let file_to_remove = args[2].clone();
            return CommandType::OpenNoteFileInEditor(file_to_remove);
        }
    }

    if args.len() < 3 {
        return CommandType::Error(String::from("Too few arguments."));
    }

    let note_filename = first_argument;
    let second_argument = &args[2];

    if list_args.contains(second_argument) {
        return CommandType::ListNotesInFile(note_filename.to_string());
    }

    if remove_args.contains(second_argument) {
        if args.len() < 4 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let note_id_to_remove = args[3].clone().parse::<usize>();

            match note_id_to_remove {
                Ok(result) => {
                    return CommandType::RemoveNoteFromFile(note_filename.to_string(), result);
                }
                Err(_) => {
                    return CommandType::Error(format!(
                        "Unable to parse string to id: {}",
                        args[4]
                    ));
                }
            }
        }
    }

    if edit_args.contains(second_argument) {
        if args.len() < 4 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let note_id_to_edit = args[3].clone().parse::<usize>();

            match note_id_to_edit {
                Ok(result) => {
                    return CommandType::EditNoteInFile(note_filename.to_string(), result);
                }
                Err(_) => {
                    return CommandType::Error(format!(
                        "Unable to parse string to id: {}",
                        args[4]
                    ));
                }
            }
        }
    }

    if search_args.contains(second_argument) {
        if args.len() < 4 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let search_keyword = args[3].clone();
            return CommandType::SearchForNoteFromFile(note_filename.to_string(), search_keyword);
        }
    }

    if add_args.contains(second_argument) {
        if args.len() < 4 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let note_to_add = args[3].clone();
            return CommandType::SearchForNoteFromFile(note_filename.to_string(), note_to_add);
        }
    }

    CommandType::AddNoteToFile(note_filename.to_string(), second_argument.to_string())
}

#[allow(unused_variables)]

mod parse_command_type_tests {
    #[allow(unused_imports)]
    use crate::parsearguments::{parse_command_type, CommandType};

    #[test]
    fn parse_command_type_tests() -> Result<(), &'static str> {
        let args: Vec<String> = vec![
            String::from("rn"),
            String::from("remove"),
            String::from("notefile"),
        ];

        let result = parse_command_type(args);

        match result {
            CommandType::RemoveNoteFile(x) => {
                if x.eq("notefile") {
                    Ok(())
                } else {
                    Err("Not ok")
                }
            }
            _ => {
                return Err("Not ok");
            }
        }
    }
}

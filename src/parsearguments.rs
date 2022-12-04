#[derive(Debug, PartialEq)]
pub enum CommandType {
    ListNoteFiles,
    RemoveNoteFile(String),
    OpenNoteFileInEditor(String),

    ListNotesInFile(String),
    AddNoteToFile(String, String),
    EditNoteInFile(String, usize),
    RemoveNoteFromFile(String, usize),
    PrintHelp,
    Error(String),
}

/// Takes arguments given at bin call and gets what is wanted from the tool
pub fn rn_get_command_type(args: Vec<String>) -> CommandType {
    let help_args: [String; 2] = [String::from("help"), String::from("h")];

    let list_args: [String; 2] = [String::from("list"), String::from("l")];

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
                        args[3]
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

    if add_args.contains(second_argument) {
        if args.len() < 4 {
            return CommandType::Error(String::from("Too few arguments."));
        } else {
            let note_to_add = args[3].clone();
            return CommandType::AddNoteToFile(note_filename.to_string(), note_to_add);
        }
    }

    CommandType::AddNoteToFile(note_filename.to_string(), second_argument.to_string())
}

#[allow(unused_variables)]

mod parse_command_type_tests {
    use crate::parsearguments::{rn_get_command_type, CommandType};

    #[test]
    fn test_help() {
        let mut args = fake_args("help", "", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(result, CommandType::PrintHelp);
        args = fake_args("h", "", "");
        result = rn_get_command_type(args);
        assert_eq!(result, CommandType::PrintHelp);
        args = vec!["rn".to_string()];
        result = rn_get_command_type(args);
        assert_eq!(result, CommandType::PrintHelp);
    }

    #[test]
    fn test_add_notefile() {
        let mut args = fake_args("notefile", "test note", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::AddNoteToFile("notefile".to_string(), "test note".to_string())
        );
        args = fake_args("notefile", "add", "test note");
        result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::AddNoteToFile("notefile".to_string(), "test note".to_string())
        );
        args = fake_args("notefile", "a", "test note");
        result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::AddNoteToFile("notefile".to_string(), "test note".to_string())
        );
    }

    #[test]
    fn test_list_notefiles() {
        let mut args = fake_args("list", "", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(result, CommandType::ListNoteFiles);
        args = fake_args("l", "", "");
        result = rn_get_command_type(args);
        assert_eq!(result, CommandType::ListNoteFiles);
    }

    #[test]
    fn test_list_notes_in_notefiles() {
        let mut args = fake_args("notefile", "list", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(result, CommandType::ListNotesInFile("notefile".to_string()));
        args = fake_args("notefile", "l", "");
        result = rn_get_command_type(args);
        assert_eq!(result, CommandType::ListNotesInFile("notefile".to_string()));
    }

    #[test]
    fn test_remove_note_from_notefile() {
        let mut args = fake_args("notefile", "remove", "5");
        let mut result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::RemoveNoteFromFile("notefile".to_string(), 5)
        );
        args = fake_args("notefile", "r", "5");
        result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::RemoveNoteFromFile("notefile".to_string(), 5)
        );
    }

    #[test]
    fn test_edit_note_in_notefile() {
        let mut args = fake_args("notefile", "edit", "5");
        let mut result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::EditNoteInFile("notefile".to_string(), 5)
        );
        args = fake_args("notefile", "e", "5");
        result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::EditNoteInFile("notefile".to_string(), 5)
        );
    }

    #[test]
    fn test_open_notefile_in_editor() {
        let mut args = fake_args("open", "notefile", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::OpenNoteFileInEditor("notefile".to_string())
        );
        args = fake_args("o", "notefile", "");
        result = rn_get_command_type(args);
        assert_eq!(
            result,
            CommandType::OpenNoteFileInEditor("notefile".to_string())
        );
    }

    #[test]
    fn test_remove_notefile() {
        let mut args = fake_args("remove", "notefile", "");
        let mut result = rn_get_command_type(args);
        assert_eq!(result, CommandType::RemoveNoteFile("notefile".to_string()));
        args = fake_args("r", "notefile", "");
        result = rn_get_command_type(args);
        assert_eq!(result, CommandType::RemoveNoteFile("notefile".to_string()));
    }

    fn fake_args(arg1: &str, arg2: &str, arg3: &str) -> Vec<String> {
        vec![
            "rn".to_string(),
            arg1.to_string(),
            arg2.to_string(),
            arg3.to_string(),
        ]
    }
}

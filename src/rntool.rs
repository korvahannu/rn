use std::fs;

use crate::parsearguments::CommandType;

pub struct RnTool {
    working_directory: String,
    command_type: CommandType
}

impl RnTool {
    pub fn new(working_directory: String, command_type: CommandType) -> RnTool {
        match fs::create_dir(&working_directory) {
            Err(error) => {
                panic!("Problem with working directory {}, {}", working_directory, error);
            },
            _=> {}
        }
        RnTool { working_directory, command_type }
    }
    pub fn execute_command(&self) -> Result<(), String> {
        match &self.command_type {
            CommandType::ListNoteFiles => {
                Ok(())
            },
            CommandType::RemoveNoteFile(file) => {
                Ok(())
            },
            CommandType::OpenNoteFileInEditor(file) => {
                Ok(())
            },
            CommandType::ListNotesInFile(file) => {
                Ok(())
            },
            CommandType::AddNoteToFile(file, note) =>  {
                Ok(())
            },
            CommandType::EditNoteInFile(file, line) => {
                Ok(())
            },
            CommandType::RemoveNoteFromFile(file, line) => {
                Ok(())
            },
            CommandType::PrintHelp => {
                Ok(())
            },
            CommandType::Error(error) => {
                Err(error.clone())
            }
        }
    }
}
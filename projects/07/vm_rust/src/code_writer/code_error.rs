use crate::code_writer::commands::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CodeError {
    InvalidCommand(Commands),
    Error(String),
}

impl CodeError {
    pub fn invalid_command(cmd: Commands) -> Self {
        CodeError::InvalidCommand(cmd)
    }

    pub fn file_error(str: String) -> Self {
        CodeError::Error(str)
    }
}

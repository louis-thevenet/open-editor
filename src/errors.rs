use std::{fmt::Display, path::PathBuf};

#[derive(Debug)]
/// Errors that can occur when trying to open an editor.
pub enum OpenEditorError {
    NoEditorFound,
    EditorCallError {
        exit_code: Option<i32>,
        stderr: String,
    },
    CommandFail {
        error: std::io::Error,
    },
    EditorNotFound {
        binary_path: PathBuf,
    },
    EditorNotExecutable {
        binary_path: PathBuf,
        error: Option<std::io::Error>,
    },
}
impl Display for OpenEditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpenEditorError::NoEditorFound => write!(f, "No editor found in the system path."),
            OpenEditorError::EditorCallError { exit_code, stderr } => write!(
                f,
                "Editor call failed with exit code {exit_code:?} and stderr: {stderr}"
            ),
            OpenEditorError::CommandFail { error } => {
                write!(f, "Command failed: {error:?}")
            }
            OpenEditorError::EditorNotFound { binary_path } => write!(
                f,
                "Editor binary not found at path: {}",
                binary_path.display()
            ),
            OpenEditorError::EditorNotExecutable {
                error: _,
                binary_path,
            } => write!(
                f,
                "Editor binary is not executable at path: {}",
                binary_path.display()
            ),
        }
    }
}
impl std::error::Error for OpenEditorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OpenEditorError::EditorCallError {
                exit_code: _,
                stderr: _,
            } => todo!(),
            OpenEditorError::CommandFail { error } => Some(error),
            OpenEditorError::EditorNotFound { binary_path: _ } | OpenEditorError::NoEditorFound => {
                None
            }
            OpenEditorError::EditorNotExecutable {
                binary_path: _,
                error,
            } => error.as_ref().map(|e| e as &dyn std::error::Error),
        }
    }
}

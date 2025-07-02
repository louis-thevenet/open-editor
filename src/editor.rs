use std::{ffi::OsStr, path::PathBuf};

use crate::{editor_kind::EditorKind, errors::OpenEditorError};

#[derive(Debug)]
/// Represents an editor instance with its type and binary path.
pub struct Editor {
    pub(crate) editor_type: EditorKind,
    pub(crate) binary_path: PathBuf,
}

impl Editor {
    /// Creates a new `Editor` instance with the specified [`EditorKind`] and
    /// binary path.
    pub fn new(editor_type: EditorKind, binary_path: PathBuf) -> Self {
        Self {
            editor_type,
            binary_path,
        }
    }

    /// Validates that the binary path exists and is executable.
    /// Returns `Ok(())` if the binary is valid, or an `OpenEditorError` if it is not.
    pub(crate) fn validate_executable(&self) -> Result<(), OpenEditorError> {
        if !self.binary_path.exists() || !self.binary_path.is_file() {
            return Err(OpenEditorError::EditorNotFound {
                binary_path: self.binary_path.clone(),
            });
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = std::fs::metadata(&self.binary_path).map_err(|e| {
                OpenEditorError::EditorNotExecutable {
                    binary_path: self.binary_path.clone(),
                    error: Some(e),
                }
            })?;
            let permissions = metadata.permissions();
            if permissions.mode() & 0o111 == 0 {
                return Err(OpenEditorError::EditorNotExecutable {
                    binary_path: self.binary_path.clone(),
                    error: None,
                });
            }
        }

        Ok(())
    }
}

impl<T: Into<EditorKind>> From<T> for Editor {
    fn from(editor_type: T) -> Self {
        let editor_type = editor_type.into();
        let binary_path = match &editor_type {
            EditorKind::UnknownEditor(name) => get_full_path(name),
            v => get_full_path(v.as_str()),
        };

        Self {
            editor_type,
            binary_path,
        }
    }
}

/// Gets the full path of the editor binary based on the provided editor name.
fn get_full_path(editor_name: impl AsRef<OsStr>) -> PathBuf {
    match which::which(editor_name.as_ref()) {
        Ok(path) => path,
        Err(_) => PathBuf::from(editor_name.as_ref()), // Fallback to just the name but that's weird
    }
}

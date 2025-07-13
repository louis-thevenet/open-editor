use std::{ffi::OsString, path::PathBuf};

use crate::{editor_kind::EditorKind, errors::OpenEditorError};

#[derive(Debug, Clone)]
/// Represents an editor instance with its type and binary path.
pub struct Editor {
    pub(crate) editor_type: EditorKind,
    pub(crate) binary_path: PathBuf,
}

impl Editor {
    /// Creates a new `Editor` instance from a binary path. The editor type is set to `UnknownEditor`.
    #[must_use]
    pub fn from_bin_path(binary_path: PathBuf) -> Self {
        Self {
            editor_type: EditorKind::UnknownEditor,
            binary_path,
        }
    }
    /// Creates a new `Editor` instance from an editor kind. The binary path is determined using the `which` crate.
    #[must_use]
    pub fn from_editor_kind(editor_type: EditorKind) -> Self {
        let binary_path = Self::get_full_path(editor_type.to_string().into());
        Self {
            editor_type,
            binary_path,
        }
    }
    /// Creates a new `Editor` instance with the specified editor type and binary path.
    #[must_use]
    pub fn new(editor_type: EditorKind, binary_path: PathBuf) -> Self {
        Self {
            editor_type,
            binary_path,
        }
    }
    /// Gets the full path of the editor binary based on the provided editor name.
    pub(crate) fn get_full_path(editor_name: OsString) -> PathBuf {
        match which::which(editor_name.clone()) {
            Ok(path) => path,
            Err(_) => PathBuf::from(editor_name), // Fallback to just the name but that's weird
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

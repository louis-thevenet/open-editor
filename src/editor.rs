use std::path::PathBuf;

use crate::{editor_kind::EditorKind, errors::OpenEditorError};

#[derive(Debug)]
pub(crate) struct Editor {
    pub(crate) editor_type: EditorKind,
    pub(crate) binary_path: PathBuf,
}

impl Editor {
    pub(crate) fn new(editor_type: EditorKind, binary_path: PathBuf) -> Self {
        Self {
            editor_type,
            binary_path,
        }
    }
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

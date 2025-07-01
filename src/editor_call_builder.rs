use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{editor::Editor, editor_kind::EditorKind, errors::OpenEditorError};

pub struct EditorCallBuilder {
    editor: Editor,
    file_path: PathBuf,
    wait: bool,
    line_number: usize,
    column_number: usize,
}

impl EditorCallBuilder {
    /// Creates a new [`EditorCallBuilder`] with the given file path.
    /// You can optionally set the line and column numbers later using the `at_line` and `at_column` methods.
    /// Finally, you can call the `call_editor` method to open the editor.
    ///
    /// The editor to use is determined by the `VISUAL` and `EDITOR` environment
    /// variables, in that order.
    ///
    /// # Errors
    /// This function will return an error if the default editor cannot be found in the environment variables.
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self, OpenEditorError> {
        Self::new_with_env_vars(file_path, super::ENV_VARS)
    }
    /// Similar to [`EditorCallBuilder::new`], but allows specifying the
    /// environment variables to use to find the editor.
    pub fn new_with_env_vars<P: AsRef<Path>>(
        file_path: P,
        env_vars: &[&str],
    ) -> Result<Self, OpenEditorError> {
        Ok(Self {
            editor: Self::get_default_editor(env_vars)?,
            file_path: file_path.as_ref().to_path_buf(),
            wait: true,
            line_number: 1,
            column_number: 1,
        })
    }
    #[must_use]
    /// Sets the line number for the editor to open at.
    pub fn at_line(self, line: usize) -> Self {
        Self {
            line_number: line,
            ..self
        }
    }
    #[must_use]
    /// Sets the column number for the editor to open at.
    pub fn at_column(self, line: usize) -> Self {
        Self {
            column_number: line,
            ..self
        }
    }
    /// Whether to wait for the editor to close before returning.
    #[must_use]
    pub fn wait_for_editor(self, value: bool) -> Self {
        Self {
            wait: value,
            ..self
        }
    }
    /// Calls the editor with options from the [`EditorCallBuilder`].
    /// # Errors
    ///
    /// This function will return an error if the commands fails to execute or if the editor returns a non-zero exit code.
    pub fn call_editor(&self) -> Result<(), OpenEditorError> {
        self.editor.validate_executable()?;
        let command = Command::new(&self.editor.binary_path)
            .args(self.editor.editor_type.get_editor_args(
                &self.file_path,
                self.wait,
                self.line_number,
                self.column_number,
            ))
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        if !self.wait {
            return Ok(());
        }

        match command {
            Ok(output) => {
                let output = output
                    .wait_with_output()
                    .map_err(|e| OpenEditorError::CommandFail { error: e })?;
                if output.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    Err(OpenEditorError::EditorCallError {
                        exit_code: output.status.code(),
                        stderr: stderr.to_string(),
                    })
                }
            }
            Err(e) => Err(OpenEditorError::CommandFail { error: e }),
        }
    }
    /// Gets the full path of the editor binary based on the provided editor name.
    fn get_full_path(editor_name: OsString) -> PathBuf {
        match which::which(editor_name.clone()) {
            Ok(path) => path,
            Err(_) => PathBuf::from(editor_name), // Fallback to just the name but that's weird
        }
    }
    /// Gets the default editor from the environment variables `VISUAL` or `EDITOR`.
    fn get_default_editor(env_vars: &[&str]) -> Result<Editor, OpenEditorError> {
        env_vars
            .iter()
            .filter_map(env::var_os)
            .filter(|var| !var.is_empty())
            .map(|v| {
                let path = EditorCallBuilder::get_full_path(v.clone());
                (v.into_string().ok(), path)
            })
            .filter_map(|(v, path)| v.map(|v| (v, path)))
            .map(|(v, cmd)| (Editor::new(EditorKind::from(v), cmd)))
            .next()
            .ok_or(OpenEditorError::NoEditorFound)
    }
}

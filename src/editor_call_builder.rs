use std::{
    env::{self, temp_dir},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use crate::{ENV_VARS, editor::Editor, editor_kind::EditorKind, errors::OpenEditorError};

pub struct EditorCallBuilder {
    editor: Option<Editor>,
    file_path: Option<PathBuf>,
    custom_env_vars: Vec<String>,
    wait: bool,
    line_number: usize,
    column_number: usize,
}
impl Default for EditorCallBuilder {
    fn default() -> Self {
        Self {
            editor: None,
            file_path: None,
            custom_env_vars: vec![],
            wait: true,
            line_number: 1,
            column_number: 1,
        }
    }
}

impl EditorCallBuilder {
    /// Creates a new [`EditorCallBuilder`].
    /// You can optionally set the line and column numbers later using the `at_line` and `at_column` methods.
    ///
    /// Finally, you can call the editor with the `open_editor`, `edit_string`, or `edit_string_mut` methods.
    ///
    /// # Errors
    /// This function will return an error if the default editor cannot be found in the environment variables.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
    /// Add additional environment variables to look for the editor in. These variables
    /// will have higher priority than `VISUAL` and `EDITOR`.
    #[must_use]
    pub fn with_env_vars(self, env_vars: &[&str]) -> Self {
        let mut custom_env_vars = self.custom_env_vars;
        custom_env_vars.extend(env_vars.iter().map(|&s| s.to_string()));
        Self {
            custom_env_vars,
            ..self
        }
    }
    /// Open the default editor and returns what was written in it.
    ///
    /// # Errors
    /// If the editor call fails, or if the temporary file cannot be read or cleaned up,
    /// or if the editor call fails.
    pub fn open_editor(&self) -> Result<String, OpenEditorError> {
        self.edit_string("")
    }
    /// Open the default editor and allows editing of a mutable string.
    ///
    /// # Errors
    ///
    /// If the editor call fails, or if the temporary file cannot be read or cleaned up,
    /// or if the editor call fails.
    pub fn edit_string_mut(&self, string: &mut String) -> Result<(), OpenEditorError> {
        *string = self.edit_string(string)?;
        Ok(())
    }

    /// Open the default editor and allows editing of a string which is then returned.
    ///
    /// # Errors
    /// If the editor call fails, or if the temporary file cannot be read or cleaned up,
    /// or if the editor call fails.
    pub fn edit_string(&self, string: &str) -> Result<String, OpenEditorError> {
        let file_path = match &self.file_path {
            Some(path) => path,
            None => &{
                let mut filename = temp_dir();
                filename.push(String::from("open_editor_tmp_file"));
                filename
            },
        };
        // Write the initial content to the temporary file
        std::fs::write(file_path, string).map_err(OpenEditorError::FileManipulationFail)?;
        self.open_file(file_path)?;
        let result =
            std::fs::read_to_string(file_path).map_err(OpenEditorError::FileManipulationFail)?;

        // Clean up the temporary file after reading
        std::fs::remove_file(file_path).map_err(|_| {
            OpenEditorError::TempFileCleanupFail(file_path.to_string_lossy().into_owned())
        })?;

        Ok(result)
    }
    /// Opens the specified file in the editor.
    ///
    /// # Errors
    /// This function will return an error if the editor call fails or if the file cannot be read.
    pub fn open_file(&self, file_path: &Path) -> Result<(), OpenEditorError> {
        let editor = match &self.editor {
            Some(editor) => editor,
            None => &self.get_default_editor()?,
        };

        // Build the actual Editor Call
        let editor_call = EditorCall {
            editor: editor.clone(),
            file_path: file_path.to_path_buf(),
            wait: self.wait,
            line_number: self.line_number,
            column_number: self.column_number,
        };
        editor_call.call()
    }
    /// Gets the default editor from the environment variables `VISUAL` or `EDITOR`.
    fn get_default_editor(&self) -> Result<Editor, OpenEditorError> {
        self.custom_env_vars
            .clone()
            .into_iter()
            .chain(ENV_VARS.iter().map(|&s| s.to_string()))
            .filter_map(env::var_os)
            .filter(|var| !var.is_empty())
            .map(|v| {
                let path = Editor::get_full_path(v.clone());
                (v.into_string().ok(), path)
            })
            .filter_map(|(v, path)| v.map(|v| (v, path)))
            .map(|(v, cmd)| (Editor::new(EditorKind::from(v), cmd)))
            .next()
            .ok_or(OpenEditorError::NoEditorFound)
    }
}
/// Represents a call to an editor with specific options.
struct EditorCall {
    editor: Editor,
    file_path: PathBuf,
    wait: bool,
    line_number: usize,
    column_number: usize,
}
impl EditorCall {
    /// Calls the editor with options from the [`EditorCallBuilder`].
    /// # Errors
    ///
    /// This function will return an error if the commands fails to execute or if the editor returns a non-zero exit code.
    pub fn call(&self) -> Result<(), OpenEditorError> {
        self.editor.validate_executable()?; // Ensure the editor binary is valid
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
}

use std::env::temp_dir;

use crate::errors::OpenEditorError;

mod editor;
mod editor_call_builder;
mod editor_kind;
pub mod errors;

pub use {editor::Editor, editor_call_builder::EditorCallBuilder, editor_kind::EditorKind};

static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];

pub struct EditOptions {
    /// Editor to use.
    pub editor: Option<editor::Editor>,

    /// Environment variables to use to find the editor, if `editor` is not
    /// specified.
    pub env_vars: Vec<String>,
}

impl Default for EditOptions {
    fn default() -> Self {
        Self {
            editor: None,
            env_vars: ENV_VARS.iter().map(|v| v.to_string()).collect(),
        }
    }
}

/// Open the default editor and allows editing of a string.
///
/// The default editor is determined by the `VISUAL` and `EDITOR` environment
/// variables, in that order.
///
/// # Errors
///
/// This function will return an error if the editor call fails, if the file
/// cannot be read, or if the temporary file cleanup fails.
pub fn edit_in_editor(string: &str) -> Result<String, OpenEditorError> {
    edit_in_editor_with_opts(string, EditOptions::default())
}

/// Similar to [`edit_in_editor`], but allows specifying [`EditOptions`].
pub fn edit_in_editor_with_opts(
    string: &str,
    opts: EditOptions,
) -> Result<String, OpenEditorError> {
    let mut filename = temp_dir();
    filename.push(String::from("open_editor_tmp_file"));

    // Write the initial content to the temporary file
    std::fs::write(&filename, string).map_err(OpenEditorError::FileManipulationFail)?;

    let builder = match opts.editor {
        Some(editor) => EditorCallBuilder::new_with_editor(filename.clone(), editor)?,
        None => EditorCallBuilder::new_with_env_vars(filename.clone(), &opts.env_vars)?,
    };

    builder.call_editor()?;
    let result =
        std::fs::read_to_string(&filename).map_err(OpenEditorError::FileManipulationFail)?;

    // Clean up the temporary file after reading
    std::fs::remove_file(&filename).map_err(|_| {
        OpenEditorError::TempFileCleanupFail(filename.to_string_lossy().into_owned())
    })?;

    Ok(result)
}

/// Open the default editor and allows editing of a mutable string.
///
/// The default editor is determined by the `VISUAL` and `EDITOR` environment
/// variables, in that order.
///
/// # Errors
///
/// This function will return an error if the editor call fails, if the file
/// cannot be read, or if the temporary file cleanup fails.
pub fn edit_mut_in_editor(string: &mut String) -> Result<(), OpenEditorError> {
    edit_mut_in_editor_with_opts(string, EditOptions::default())
}

/// Similar to [`edit_mut_in_editor`], but allows specifying [`EditOptions`].
pub fn edit_mut_in_editor_with_opts(
    string: &mut String,
    opts: EditOptions,
) -> Result<(), OpenEditorError> {
    *string = edit_in_editor_with_opts(string, opts)?;
    Ok(())
}

/// Open the default editor and returns what was written in it.
///
/// The default editor is determined by the `VISUAL` and `EDITOR` environment
/// variables, in that order.
///
/// # Errors
///
/// If the editor call fails, or if the file cannot be read, an error will be
/// returned.
pub fn open_editor() -> Result<String, OpenEditorError> {
    open_editor_with_opts(EditOptions::default())
}

/// Similar to [`open_editor`], but allows specifying [`EditOptions`].
pub fn open_editor_with_opts(opts: EditOptions) -> Result<String, OpenEditorError> {
    edit_in_editor_with_opts("", opts)
}

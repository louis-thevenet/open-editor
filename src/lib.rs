use std::env::temp_dir;

use crate::errors::OpenEditorError;

mod editor;
pub mod editor_call_builder;
mod editor_kind;
pub mod errors;

pub use editor_call_builder::EditorCallBuilder;

static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];

/// Open the default editor and allows editing of a string.
///
/// The default editor is determined by the `VISUAL` and `EDITOR` environment
/// variables, in that order.
///
/// # Errors
/// This function will return an error if the editor call fails, if the file cannot be read, or if the temporary file cleanup fails.
pub fn edit_in_editor(string: &str) -> Result<String, OpenEditorError> {
    edit_in_editor_with_env_vars(string, ENV_VARS)
}

/// Similar to [`edit_in_editor`], but allows specifying the environment
/// variables to use to find the editor.
pub fn edit_in_editor_with_env_vars(
    string: &str,
    env_vars: &[&str],
) -> Result<String, OpenEditorError> {
    let mut filename = temp_dir();
    filename.push(String::from("open_editor_tmp_file"));

    // Write the initial content to the temporary file
    std::fs::write(&filename, string).map_err(OpenEditorError::FileManipulationFail)?;

    EditorCallBuilder::new_with_env_vars(filename.clone(), env_vars)?.call_editor()?;
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
/// This function will return an error if the editor call fails, if the file cannot be read, or if the temporary file cleanup fails.
pub fn edit_mut_in_editor(string: &mut String) -> Result<(), OpenEditorError> {
    edit_mut_in_editor_with_env_vars(string, ENV_VARS)
}

/// Similar to [`edit_mut_in_editor`], but allows specifying the environment
/// variables to use to find the editor.
pub fn edit_mut_in_editor_with_env_vars(
    string: &mut String,
    env_vars: &[&str],
) -> Result<(), OpenEditorError> {
    *string = edit_in_editor_with_env_vars(string, env_vars)?;
    Ok(())
}

/// Open the default editor and returns what was written in it.
///
/// The default editor is determined by the `VISUAL` and `EDITOR` environment
/// variables, in that order.
///
/// # Errors
/// If the editor call fails, or if the file cannot be read, an error will be returned.
pub fn open_editor() -> Result<String, OpenEditorError> {
    open_editor_with_env_vars(ENV_VARS)
}

/// Similar to [`open_editor`], but allows specifying the environment variables
/// to use to find the editor.
pub fn open_editor_with_env_vars(env_vars: &[&str]) -> Result<String, OpenEditorError> {
    edit_in_editor_with_env_vars("", env_vars)
}

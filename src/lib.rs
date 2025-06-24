use std::env::temp_dir;

use crate::{editor_call_builder::EditorCallBuilder, errors::OpenEditorError};

mod editor;
pub mod editor_call_builder;
mod editor_kind;
pub mod errors;

/// Open the default editor and allows editing of a string.
///
/// # Errors
/// This function will return an error if the editor call fails, if the file cannot be read, or if the temporary file cleanup fails.
pub fn edit_in_editor(string: &str) -> Result<String, OpenEditorError> {
    let mut filename = temp_dir();
    filename.push(String::from("open_editor_tmp_file"));

    // Write the initial content to the temporary file
    std::fs::write(&filename, string).map_err(OpenEditorError::FileManipulationFail)?;

    EditorCallBuilder::new(filename.clone())?.call_editor()?;
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
/// # Errors
///
/// This function will return an error if the editor call fails, if the file cannot be read, or if the temporary file cleanup fails.
pub fn edit_mut_in_editor(string: &mut String) -> Result<(), OpenEditorError> {
    *string = edit_in_editor(string)?;
    Ok(())
}
/// Open the default editor and returns what was written in it.
/// # Errors
/// If the editor call fails, or if the file cannot be read, an error will be returned.
pub fn open_editor() -> Result<String, OpenEditorError> {
    edit_in_editor("")
}

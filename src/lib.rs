use crate::{editor_call_builder::EditorCallBuilder, errors::OpenEditorError};

mod editor;
pub mod editor_call_builder;
mod editor_kind;
pub mod errors;

/// Open the default editor and returns what was written in it.
/// # Errors
/// If the editor call fails, or if the file cannot be read, an error will be returned.
pub fn open_editor() -> Result<String, OpenEditorError> {
    let filename = String::from("/tmp/open_editor_tmp_file");
    EditorCallBuilder::new(filename.clone())?.call_editor()?;
    let result = std::fs::read_to_string(&filename).map_err(OpenEditorError::FileReadFail)?;
    // Clean up the temporary file after reading
    std::fs::remove_file(&filename)
        .map_err(|_| OpenEditorError::TempFileCleanupFail(filename.clone()))?;
    Ok(result)
}

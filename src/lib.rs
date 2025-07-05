mod editor;
pub mod editor_call_builder;
mod editor_kind;
pub mod errors;

use std::path::Path;

pub use editor_call_builder::EditorCallBuilder;

use crate::errors::OpenEditorError;

static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];
/// Macro to implement static methods for `EditorCallBuilder`.
macro_rules! impl_static_editor_methods {
    (
        $(
            $(#[$doc:meta])*
            $static_name:ident($($param:ident: $param_type:ty),*) -> $return_type:ty => $instance_method:ident
        ),* $(,)?
    ) => {
            $(
                $(#[$doc])*
                pub fn $static_name($($param: $param_type),*) -> $return_type {
                    EditorCallBuilder::new().$instance_method($($param),*)
                }
            )*

    };
}

impl_static_editor_methods! {
    /// Open the default editor and return what was written in it.
    ///
    /// This is a static convenience method equivalent to `EditorCallBuilder::new().open_editor()`.
    ///
    /// # Errors
    /// Returns an error if the editor call fails, or if the temporary file cannot be read or cleaned up.
    open_editor() -> Result<String, OpenEditorError> => open_editor,

    /// Edit a string in the default editor and return the result.
    ///
    /// This is a static convenience method equivalent to `EditorCallBuilder::new().edit_string(string)`.
    ///
    /// # Errors
    /// Returns an error if the editor call fails, or if the temporary file cannot be read or cleaned up.
    edit_string(string: &str) -> Result<String, OpenEditorError> => edit_string,

    /// Edit a mutable string in place using the default editor.
    ///
    /// This is a static convenience method equivalent to `EditorCallBuilder::new().edit_string_mut(string)`.
    ///
    /// # Errors
    /// Returns an error if the editor call fails, or if the temporary file cannot be read or cleaned up.
    edit_string_mut(string: &mut String) -> Result<(), OpenEditorError> => edit_string_mut,

    /// Open a file in the default editor.
    ///
    /// This is a static convenience method equivalent to `EditorCallBuilder::new().open_file(file_path)`.
    ///
    /// # Errors
    /// Returns an error if the editor call fails or if the file cannot be read.
    open_file(file_path: &Path) -> Result<(), OpenEditorError> => open_file,
}

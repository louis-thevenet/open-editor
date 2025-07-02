mod editor;
pub mod editor_call_builder;
mod editor_kind;
pub mod errors;

pub use editor_call_builder::EditorCallBuilder;

static ENV_VARS: &[&str] = &["VISUAL", "EDITOR"];

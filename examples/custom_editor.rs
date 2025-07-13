use open_editor::{Editor, EditorCallBuilder, EditorKind, errors::OpenEditorError};

fn main() -> Result<(), OpenEditorError> {
    let user_input = EditorCallBuilder::new()
        .with_editor(Editor::from_editor_kind(EditorKind::Nano))
        .open_editor()?;
    println!("User input:\n{user_input}");
    Ok(())
}

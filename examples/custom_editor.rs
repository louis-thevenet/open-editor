use open_editor::{EditOptions, EditorKind, errors::OpenEditorError, open_editor_with_opts};

fn main() -> Result<(), OpenEditorError> {
    let user_input = open_editor_with_opts(EditOptions {
        editor: Some(EditorKind::Vi.into()),
        ..Default::default()
    })?;

    println!("User input:\n{user_input}");
    Ok(())
}

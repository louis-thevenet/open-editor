use open_editor::{EditorCallBuilder, errors::OpenEditorError};

fn main() -> Result<(), OpenEditorError> {
    let user_input = EditorCallBuilder::new().open_editor()?;
    println!("User input:\n{user_input}");
    Ok(())
}

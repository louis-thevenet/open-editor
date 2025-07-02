use open_editor::{EditorCallBuilder, errors::OpenEditorError};

fn main() -> Result<(), OpenEditorError> {
    let user_input = EditorCallBuilder::new()
        .with_env_vars(&["MY_EDITOR"])
        .open_editor()?;
    println!("User input:\n{user_input}");
    Ok(())
}

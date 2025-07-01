use open_editor::{errors::OpenEditorError, open_editor_with_env_vars};

fn main() -> Result<(), OpenEditorError> {
    let user_input = open_editor_with_env_vars(&["MY_EDITOR"])?;
    println!("User input:\n{user_input}");
    Ok(())
}

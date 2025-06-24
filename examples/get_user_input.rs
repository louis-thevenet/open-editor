use open_editor::{errors::OpenEditorError, open_editor};

fn main() -> Result<(), OpenEditorError> {
    let user_input = open_editor()?;
    println!("User input:\n{user_input}");
    Ok(())
}

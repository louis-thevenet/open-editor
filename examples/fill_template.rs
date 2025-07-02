use open_editor::EditorCallBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = "Hello, {name}!\nWelcome to {place}.";
    let filled_template = EditorCallBuilder::new().edit_string(template)?;
    println!("Filled Template:\n{filled_template}");
    Ok(())
}

use open_editor::edit_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = "Hello, {name}!\nWelcome to {place}.";
    // Also works:
    // let filled_template = EditorCallBuilder::new().edit_string(template)?;
    let filled_template = edit_string(template)?;
    println!("Filled Template:\n{filled_template}");
    Ok(())
}

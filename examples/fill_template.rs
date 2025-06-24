use open_editor::edit_in_editor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let template = "Hello, {name}!\nWelcome to {place}.";
    let filled_template = edit_in_editor(template)?;
    println!("Filled Template:\n{filled_template}");
    Ok(())
}

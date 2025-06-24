use open_editor::editor_call_builder::EditorCallBuilder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "./test";

    EditorCallBuilder::new(filename)?
        .wait_for_editor(false)
        .call_editor()?;

    Ok(())
}

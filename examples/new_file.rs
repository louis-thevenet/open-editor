use open_editor::{editor_call_builder::EditorCallBuilder, errors::OpenEditorError};

fn main() -> Result<(), OpenEditorError> {
    let filename = String::from("/tmp/new_file.txt");
    EditorCallBuilder::new(filename)?
        .at_line(5)
        .at_column(8)
        .call_editor()
}

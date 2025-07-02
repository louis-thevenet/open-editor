use std::{fs, path::PathBuf, str::FromStr, thread::sleep, time::Duration};

use open_editor::editor_call_builder::EditorCallBuilder;

/// This example is not very practical with TUI editors but it shows how to
/// spawn an editor and do other stuff.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = PathBuf::from_str("./test")?;

    // Spawn editor without waiting
    EditorCallBuilder::new()
        .wait_for_editor(false)
        .open_file(&filename)?;

    println!("Editor launched. Press Ctrl+C to stop.\n");

    loop {
        let contents = fs::read_to_string(&filename).unwrap_or_default();

        println!("--- Content of {} ---\n{}\n", filename.display(), contents);

        sleep(Duration::from_secs(1));
    }
}

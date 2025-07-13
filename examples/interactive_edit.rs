use open_editor::EditorCallBuilder;
use std::{
    io::{self, Write},
    path::PathBuf,
    process::exit,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (file_path, line, column) = get_parameters()?;

    EditorCallBuilder::new()
        .at_line(line)
        .at_column(column)
        .open_file(&file_path)?;
    Ok(())
}

fn get_parameters() -> Result<(PathBuf, usize, usize), Box<dyn std::error::Error>> {
    print!("Path to file to open [./Cargo.toml]: ");
    io::stdout().flush()?;
    let mut file_path = String::new();
    io::stdin().read_line(&mut file_path)?;
    if file_path.trim().is_empty() {
        file_path = "./Cargo.toml".to_string();
    }
    let file_path = PathBuf::from(file_path.trim());
    print!("Line number: ");
    io::stdout().flush()?;
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let line: usize = if let Ok(num) = line.trim().parse() {
        num
    } else {
        eprintln!("Error: Invalid line number");
        exit(-1)
    };
    print!("Column number: ");
    io::stdout().flush()?;
    let mut column = String::new();
    io::stdin().read_line(&mut column)?;
    let column: usize = if let Ok(num) = column.trim().parse() {
        num
    } else {
        eprintln!("Error: Invalid column number");
        exit(-1)
    };
    Ok((file_path, line, column))
}

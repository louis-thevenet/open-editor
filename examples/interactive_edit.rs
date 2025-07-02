use open_editor::EditorCallBuilder;
use std::{
    io::{self, Write},
    process::exit,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (filename, line, column) = get_parameters()?;

    EditorCallBuilder::new(filename)?
        .at_line(line)
        .at_column(column)
        .call_editor()?;

    Ok(())
}

fn get_parameters() -> Result<(String, usize, usize), Box<dyn std::error::Error>> {
    print!("Path to file to open [./Cargo.toml]: ");
    io::stdout().flush()?;
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)?;
    let mut filename = filename.trim();
    if filename.is_empty() {
        filename = "./Cargo.toml";
    }
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
    Ok((filename.to_owned(), line, column))
}

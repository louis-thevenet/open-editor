use std::path::Path;

#[derive(Default, Debug)]
pub(crate) enum EditorKind {
    // CLI
    Vi,
    Vim,
    Nvim,
    Emacs,
    Nano,
    Pico,
    Helix,
    // GUI
    Code,
    Atom,
    Subl,
    Gvim,
    Mate,
    #[default]
    UnknownEditor,
}

impl From<String> for EditorKind {
    /// Convert a string to an [`EditorKind`].
    fn from(value: String) -> Self {
        println!("Getting editor from: {value}");
        match value.as_str() {
            "vi" => EditorKind::Vi,
            "vim" => EditorKind::Vim,
            "nvim" => EditorKind::Nvim,
            "emacs" => EditorKind::Emacs,
            "nano" => EditorKind::Nano,
            "pico" => EditorKind::Pico,
            "hx" => EditorKind::Helix,
            "code" | "vscode" => EditorKind::Code,
            "atom" => EditorKind::Atom,
            "sublime" | "subl" => EditorKind::Subl,
            "gvim" => EditorKind::Gvim,
            "mate" => EditorKind::Mate,
            _ => EditorKind::UnknownEditor,
        }
    }
}
/// Get Editor specific arguments for opening a file at a specific line and column.
impl EditorKind {
    pub(crate) fn get_editor_args(
        &self,
        file_path: &Path,
        line: usize,
        column: usize,
    ) -> Vec<String> {
        let path = file_path.to_string_lossy().into_owned();
        match self {
            EditorKind::Emacs => {
                vec!["+".to_string(), format!("{}:{}", line, column), path]
            }
            EditorKind::Nano | EditorKind::Pico => {
                vec![format!("+{},{}", line, column), path]
            }
            EditorKind::Helix | EditorKind::Subl => {
                vec![format!("{}:{}:{}", path, line, column)]
            }
            EditorKind::Code | EditorKind::Atom => {
                vec![
                    "--goto".to_string(),
                    format!("{}:{}:{}", path, line, column),
                ]
            }
            EditorKind::Gvim | EditorKind::Vi | EditorKind::Vim | EditorKind::Nvim => {
                vec![format!("+call cursor({}, {})", line, column), path]
            }
            EditorKind::Mate => {
                vec!["--line".to_string(), line.to_string(), path]
            }
            EditorKind::UnknownEditor => vec![path],
        }
    }
}

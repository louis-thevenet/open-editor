use std::path::Path;

#[derive(Default, Debug, Clone)]
pub(crate) enum EditorKind {
    // CLI
    Vi,
    Vim,
    Nvim,
    Emacs,
    Nano,
    Pico,
    Helix,
    Kakoune,
    // GUI
    Code,
    Gvim,
    #[default]
    UnknownEditor,
}

impl From<String> for EditorKind {
    /// Convert a string to an [`EditorKind`].
    fn from(value: String) -> Self {
        match value.as_str() {
            "vi" => EditorKind::Vi,
            "vim" => EditorKind::Vim,
            "nvim" => EditorKind::Nvim,
            "emacs" => EditorKind::Emacs,
            "nano" => EditorKind::Nano,
            "pico" => EditorKind::Pico,
            "hx" => EditorKind::Helix,
            "kak" => EditorKind::Kakoune,
            "code" | "vscode" => EditorKind::Code,
            "gvim" => EditorKind::Gvim,
            _ => EditorKind::UnknownEditor,
        }
    }
}
/// Get Editor specific arguments for opening a file at a specific line and column.
impl EditorKind {
    pub(crate) fn get_editor_args(
        &self,
        file_path: &Path,
        wait: bool,
        line: usize,
        column: usize,
    ) -> Vec<String> {
        let path = file_path.to_string_lossy().into_owned();
        match self {
            EditorKind::Emacs => {
                vec![format!("+{}:{}", line, column), path]
            }
            EditorKind::Nano | EditorKind::Pico => {
                vec![format!("+{},{}", line, column), path]
            }
            EditorKind::Helix => {
                vec![format!("{}:{}:{}", path, line, column)]
            }
            EditorKind::Kakoune => {
                vec![format!("{}", path), format!("+{}:{}", line, column)]
            }
            EditorKind::Code => {
                vec![
                    if wait {
                        "-w".to_string()
                    } else {
                        String::new()
                    },
                    "--goto".to_string(),
                    format!("{}:{}:{}", path, line, column),
                ]
            }
            EditorKind::Gvim | EditorKind::Vi | EditorKind::Vim | EditorKind::Nvim => {
                vec![format!("+call cursor({}, {})", line, column), path]
            }
            EditorKind::UnknownEditor => vec![path],
        }
    }
}

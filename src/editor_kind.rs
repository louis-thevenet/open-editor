use std::path::Path;

#[derive(Debug)]
pub enum EditorKind {
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
    UnknownEditor(String),
}

impl EditorKind {
    /// Returns the editor name as a string.
    pub(crate) fn as_str(&self) -> &str {
        match self {
            EditorKind::Vi => "vi",
            EditorKind::Vim => "vim",
            EditorKind::Nvim => "nvim",
            EditorKind::Emacs => "emacs",
            EditorKind::Nano => "nano",
            EditorKind::Pico => "pico",
            EditorKind::Helix => "hx",
            EditorKind::Kakoune => "kak",
            EditorKind::Code => "code",
            EditorKind::Gvim => "gvim",
            EditorKind::UnknownEditor(name) => name.as_str(),
        }
    }
}

impl<T: AsRef<str>> From<T> for EditorKind {
    /// Convert a string to an [`EditorKind`].
    fn from(value: T) -> Self {
        match value.as_ref() {
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
            v => EditorKind::UnknownEditor(v.to_owned()),
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
            EditorKind::UnknownEditor(_) => vec![path],
        }
    }
}

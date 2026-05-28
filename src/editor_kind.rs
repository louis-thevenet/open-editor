use std::{fmt::Display, path::Path};

#[derive(Default, Debug, Clone)]
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
    // Windows
    Notepad,
    NotePadPlusPlus,
    SublimeText,
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
            "notepad" | "notepad.exe" => EditorKind::Notepad,
            "notepad++" | "notepad++.exe" | "notepadplusplus" => EditorKind::NotePadPlusPlus,
            "subl" | "sublime" | "sublime_text" | "sublime_text.exe" => EditorKind::SublimeText,
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
            EditorKind::Code => [
                if wait { vec!["-w".to_string()] } else { vec![] },
                vec!["--goto".to_string()],
                vec![format!("{}:{}:{}", path, line, column)],
            ]
            .concat(),
            EditorKind::Gvim | EditorKind::Vi | EditorKind::Vim | EditorKind::Nvim => [
                vec![format!("+call cursor({},{})", line, column)],
                if wait {
                    vec![]
                } else {
                    vec!["--nofork".to_string()]
                },
                vec![path],
            ]
            .concat(),
            EditorKind::Notepad => {
                // Notepad doesn't support line/column arguments
                vec![path]
            }
            EditorKind::NotePadPlusPlus => {
                // Notepad++ supports -n for line number
                vec![format!("-n{}", line), path]
            }
            EditorKind::SublimeText => {
                // Sublime Text supports :line:column syntax
                vec![format!("{}:{}:{}", path, line, column)]
            }
            EditorKind::UnknownEditor => vec![path],
        }
    }
}
impl Display for EditorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorKind::Vi => write!(f, "vi"),
            EditorKind::Vim => write!(f, "vim"),
            EditorKind::Nvim => write!(f, "nvim"),
            EditorKind::Emacs => write!(f, "emacs"),
            EditorKind::Nano => write!(f, "nano"),
            EditorKind::Pico => write!(f, "pico"),
            EditorKind::Helix => write!(f, "hx"),
            EditorKind::Kakoune => write!(f, "kak"),
            EditorKind::Code => write!(f, "code"),
            EditorKind::Gvim => write!(f, "gvim"),
            EditorKind::Notepad => write!(f, "notepad"),
            EditorKind::NotePadPlusPlus => write!(f, "notepad++"),
            EditorKind::SublimeText => write!(f, "sublime"),
            EditorKind::UnknownEditor => Err(std::fmt::Error),
        }
    }
}

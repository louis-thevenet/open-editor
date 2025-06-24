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
            EditorKind::Helix => {
                vec![format!("{}:{}:{}", path, line, column)]
            }

            EditorKind::Code | EditorKind::Atom => {
                vec![
                    "--goto".to_string(),
                    format!("{}:{}:{}", path, line, column),
                ]
            }
            EditorKind::Subl => {
                // Sublime supports file:line:col suffix :contentReference[oaicite:1]{index=1}
                vec![format!("{}:{}:{}", path, line, column)]
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

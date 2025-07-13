[![crates.io](https://img.shields.io/crates/v/open-editor.svg)](https://crates.io/crates/open-editor) [![docs.rs](https://docs.rs/open-editor/badge.svg)](https://docs.rs/open-editor/) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/louis-thevenet/open-editor/ci.yml)

# `open-editor`

## About

`open-editor` allows you to open the system default editor to edit files or simply get the result as a String.

It also supports calling a specific editor or opening on specific line and column numbers.

## Quick Start

See the examples for more details.

### Writing in a file

```rust
use open_editor::EditorCallBuilder;

let filename = PathBuf::from_str("./file.txt")?;

EditorCallBuilder::new()
    .at_line(5)
    .at_column(42)
    .open_file(&filename)?;
```

### Getting the result as a String

```rust
use open_editor::open_editor;

let content = open_editor()?;
assert!(!content.is_empty(), "Nothing was written");
```

### Editing Strings

```rust
use open_editor::edit_string;

let template = "Hello, {name}!\nWelcome to {place}.";
let filled_template = edit_string(template)?;
```

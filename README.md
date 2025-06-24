# `open-editor`

## About

`open-editor` allows you to open the system default editor to edit files or simply get the result as a String.

It supports calling the editor with specific line and column numbers.

## Quick Start

See the examples for more details.

### Writing in a file

```rust
use open_editor::editor_call_builder::EditorCallBuilder;

let filename =  "./file.txt";

EditorCallBuilder::new(filename)?
    .at_line(5)
    .at_column(42)
    .call_editor()?;
```

### Getting the result as a String

```rust
use open_editor::open_editor;

let content = open_editor()?;
assert!(!content.is_empty(), "Editor returned empty content");
```

### Editing Strings

```rust
let template = "Hello, {name}!\nWelcome to {place}.";
let filled_template = edit_in_editor(template)?;
```

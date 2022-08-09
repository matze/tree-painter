# tree-painter

[![Rust](https://github.com/matze/tree-painter/actions/workflows/rust.yml/badge.svg)](https://github.com/matze/tree-painter/actions/workflows/rust.yml)

A source code syntax highlighting library based on tree-sitter and Helix editor
themes and rendering to HTML and CSS.

Unlike [syntect] which uses a regex-based parser and Sublime-2-based theme definitions,
tree-painter employs [tree-sitter] to parse source code quickly and correctly as well as [Helix
TOML themes] to match colors and text styles with recognized language items.

[syntect]: https://github.com/trishume/syntect
[tree-sitter]: https://tree-sitter.github.io/tree-sitter
[Helix TOML themes]: https://docs.helix-editor.com/themes.html


## Usage

First, you need to define which kind of language you want to highlight:

```rust
// If you know what you are going to highlight ...
let cpp_lang = tree_painter::Lang::Cpp;

// ... if you don't, you can guess from the file extension.
let rust_lang = tree_painter::Lang::from("file.rs").unwrap();
```

Then load a Helix theme:

```rust
// Load theme from a file ...
let data = std::fs::read_to_string("custom.toml").unwrap();
let custom = tree_painter::Theme::from_helix(&data).unwrap();

// ... or use a bundled theme
let theme = tree_painter::Theme::from_helix(&tree_painter::themes::CATPPUCCIN_MOCHA).unwrap();
```

Finally render the code:

```rust
let mut renderer = tree_painter::Renderer::new(theme);
let source =  std::fs::read_to_string("file.rs").unwrap();

for line in renderer.render(&rust_lang, source.as_bytes()).unwrap() {
    println!("{line}");
}
```

Note that each line is formatted using `<span>`s and CSS classes. In order to map the CSS
classes to the theme's color include the output of [`Renderer::css()`] appropriately.


## Feature flags

The default feature flag enables support for all tree-sitter grammars supporting tree-sitter
0.20 as well as a couple of bundled [`themes`]. You can opt out and enable
specific grammars to reduce bloat with

```toml
[dependencies]
tree-painter = { version = "0", default-features = false, features = ["tree-sitter-c"] }
```

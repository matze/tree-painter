//! tree-painter – a source code syntax highlighting library based on tree-sitter and Helix editor
//! themes and rendering to HTML and CSS.
//!
//! Unlike [syntect] which uses a regex-based parser and Sublime-2-based theme definitions,
//! tree-painter employs [tree-sitter] to parse source code quickly and correctly as well as [Helix
//! TOML themes] to match colors and text styles with recognized language items.
//!
//! [syntect]: https://github.com/trishume/syntect
//! [tree-sitter]: https://tree-sitter.github.io/tree-sitter
//! [Helix TOML themes]: https://docs.helix-editor.com/themes.html
//!
//! # Usage
//!
//! First, you need to define which kind of language you want to highlight:
//!
//! ```
//! // If you know what you are going to highlight ...
//! let cpp_lang = tree_painter::Lang::Cpp;
//!
//! // ... if you don't, you can guess from the file extension.
//! let rust_lang = tree_painter::Lang::from("file.rs").unwrap();
//! ```
//!
//! Then load a Helix theme:
//!
//! ```no_run
//! let data = std::fs::read_to_string("catppuccin.toml").unwrap();
//! let theme = tree_painter::Theme::from_helix(&data).unwrap();
//! ```
//!
//! Finally render the code:
//!
//! ```no_run
//! # let rust_lang = tree_painter::Lang::from("file.rs").unwrap();
//! # let data = std::fs::read_to_string("catppuccin.toml").unwrap();
//! # let theme = tree_painter::Theme::from_helix(&data).unwrap();
//! let mut renderer = tree_painter::Renderer::new(theme);
//! let source =  std::fs::read_to_string("file.rs").unwrap();
//!
//! for line in renderer.render(&rust_lang, source.as_bytes()).unwrap() {
//!     println!("{line}");
//! }
//! ```
//!
//! Note that each line is formatted using `<span>`s and CSS classes. In order to map the CSS
//! classes to the theme's color include the output of [`Renderer::css()`] appropriately.
//!
//! # Feature flags
//!
//! The default feature flag enables support for all tree-sitter grammars supporting tree-sitter
//! 0.20. You can opt out and enable specific grammars to reduce bloat with
//!
//! ```toml
//! [dependencies]
//! tree-painter = { version = "0", default-features = false, features = ["tree-sitter-c"] }
//! ```

use std::path::Path;
use tree_sitter_highlight::HighlightConfiguration;

mod error;
mod renderer;
mod theme;

pub use error::Error;
pub use renderer::Renderer;
pub use theme::Theme;

/// Languages supported for syntax highlighting.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Lang {
    #[cfg(feature = "tree-sitter-c")]
    C,
    #[cfg(feature = "tree-sitter-commonlisp")]
    CommonLisp,
    #[cfg(feature = "tree-sitter-cpp")]
    Cpp,
    #[cfg(feature = "tree-sitter-c-sharp")]
    CSharp,
    #[cfg(feature = "tree-sitter-cuda")]
    Cuda,
    #[cfg(feature = "tree-sitter-go")]
    Go,
    #[cfg(feature = "tree-sitter-javascript")]
    Js,
    #[cfg(feature = "tree-sitter-json")]
    Json,
    #[cfg(feature = "tree-sitter-python")]
    Python,
    #[cfg(feature = "tree-sitter-rust")]
    Rust,
}

impl Lang {
    /// Try to guess [`Lang`] from a file extension or return [`None`].
    ///
    /// # Examples
    ///
    /// ```
    /// let lang = tree_painter::Lang::from("file.rs");
    /// assert!(matches!(Some(tree_painter::Lang::Rust), lang));
    ///
    /// let lang = tree_painter::Lang::from("file.bin");
    /// assert_eq!(lang, None);
    /// ```
    pub fn from<T: AsRef<Path>>(path: T) -> Option<Self> {
        path.as_ref()
            .extension()
            .and_then(|e| e.to_str())
            .and_then(|e| match e {
                #[cfg(feature = "tree-sitter-c")]
                "c" => Some(Lang::C),
                #[cfg(feature = "tree-sitter-c-sharp")]
                "cs" => Some(Lang::CSharp),
                #[cfg(feature = "tree-sitter-commonlisp")]
                "lisp" | "lsp" | "l" | "cl" => Some(Lang::CommonLisp),
                #[cfg(feature = "tree-sitter-cpp")]
                "cpp" | "cc" | "cxx" => Some(Lang::Cpp),
                #[cfg(feature = "tree-sitter-cuda")]
                "cu" => Some(Lang::Cuda),
                #[cfg(feature = "tree-sitter-go")]
                "go" => Some(Lang::Go),
                #[cfg(feature = "tree-sitter-javascript")]
                "js" => Some(Lang::Js),
                #[cfg(feature = "tree-sitter-json")]
                "json" => Some(Lang::Json),
                #[cfg(feature = "tree-sitter-python")]
                "py" => Some(Lang::Python),
                #[cfg(feature = "tree-sitter-rust")]
                "rs" => Some(Lang::Rust),
                &_ => None,
            })
    }

    fn config(&self) -> HighlightConfiguration {
        match self {
            #[cfg(feature = "tree-sitter-c")]
            Lang::C => HighlightConfiguration::new(
                tree_sitter_c::language(),
                tree_sitter_c::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-c"),
            #[cfg(feature = "tree-sitter-c-sharp")]
            Lang::CSharp => {
                HighlightConfiguration::new(tree_sitter_c_sharp::language(), "", "", "")
                    .expect("loading tree-sitter-c-sharp")
            }
            #[cfg(feature = "tree-sitter-commonlisp")]
            Lang::CommonLisp => {
                HighlightConfiguration::new(tree_sitter_commonlisp::language(), "", "", "")
                    .expect("loading tree-sitter-commonlisp")
            }
            #[cfg(feature = "tree-sitter-cpp")]
            Lang::Cpp => HighlightConfiguration::new(
                tree_sitter_cpp::language(),
                tree_sitter_cpp::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-cpp"),
            #[cfg(feature = "tree-sitter-cuda")]
            Lang::Cuda => HighlightConfiguration::new(tree_sitter_cuda::language(), "", "", "")
                .expect("loading tree-sitter-cuda"),
            #[cfg(feature = "tree-sitter-go")]
            Lang::Go => HighlightConfiguration::new(
                tree_sitter_go::language(),
                tree_sitter_go::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-cpp"),
            #[cfg(feature = "tree-sitter-javascript")]
            Lang::Js => HighlightConfiguration::new(
                tree_sitter_javascript::language(),
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_javascript::INJECTION_QUERY,
                tree_sitter_javascript::LOCALS_QUERY,
            )
            .expect("loading tree-sitter-javascript"),
            #[cfg(feature = "tree-sitter-json")]
            Lang::Json => HighlightConfiguration::new(
                tree_sitter_json::language(),
                tree_sitter_json::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-json"),
            #[cfg(feature = "tree-sitter-python")]
            Lang::Python => HighlightConfiguration::new(
                tree_sitter_python::language(),
                tree_sitter_python::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-cpp"),
            #[cfg(feature = "tree-sitter-rust")]
            Lang::Rust => HighlightConfiguration::new(
                tree_sitter_rust::language(),
                tree_sitter_rust::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-rust"),
        }
    }
}

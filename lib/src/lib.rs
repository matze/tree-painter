//! HTML renderer for source code based on tree-sitter and Helix editor themes.
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
    #[cfg(feature = "tree-sitter-cpp")]
    Cpp,
    #[cfg(feature = "tree-sitter-javascript")]
    Js,
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
                #[cfg(feature = "tree-sitter-cpp")]
                "cpp" | "cc" | "cxx" => Some(Lang::Cpp),
                #[cfg(feature = "tree-sitter-javascript")]
                "js" => Some(Lang::Js),
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
            #[cfg(feature = "tree-sitter-cpp")]
            Lang::Cpp => HighlightConfiguration::new(
                tree_sitter_cpp::language(),
                tree_sitter_cpp::HIGHLIGHT_QUERY,
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

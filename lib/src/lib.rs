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
//! let data = std::fs::read_to_string("custom.toml").unwrap();
//! let custom = tree_painter::Theme::from_helix(&data).unwrap();
//! // or use a bundled theme
//! let theme = tree_painter::Theme::from_helix(&tree_painter::themes::CATPPUCCIN_MOCHA).unwrap();
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
//! 0.20 as well as a couple of bundled [`themes`]. You can opt out and enable
//! specific grammars to reduce bloat with
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

#[cfg(feature = "themes")]
/// Bundled themes for use with [`Theme::from_helix()`].
pub mod themes {
    /// Ayu Dark.
    pub const AYU_DARK: &str = include_str!("themes/ayu_dark.toml");
    /// Ayu Light.
    pub const AYU_LIGHT: &str = include_str!("themes/ayu_light.toml");
    /// Ayu Mirage.
    pub const AYU_MIRAGE: &str = include_str!("themes/ayu_mirage.toml");
    /// Catppuccin Frappe.
    pub const CATPPUCCIN_FRAPPE: &str = include_str!("themes/catppuccin_frappe.toml");
    /// Catppuccin Latte.
    pub const CATPPUCCIN_LATTE: &str = include_str!("themes/catppuccin_latte.toml");
    /// Catppuccin Macchiato.
    pub const CATPPUCCIN_MACCHIATO: &str = include_str!("themes/catppuccin_macchiato.toml");
    /// Catppuccin Mocha.
    pub const CATPPUCCIN_MOCHA: &str = include_str!("themes/catppuccin_mocha.toml");
}

/// Languages supported for syntax highlighting.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Lang {
    // #[cfg(feature = "tree-sitter-bash")]
    // Bash,
    #[cfg(feature = "tree-sitter-c")]
    C,
    // #[cfg(feature = "tree-sitter-clojure")]
    // Clojure,
    #[cfg(feature = "tree-sitter-cpp")]
    Cpp,
    #[cfg(feature = "tree-sitter-c-sharp")]
    CSharp,
    #[cfg(feature = "tree-sitter-css")]
    Css,
    #[cfg(feature = "tree-sitter-dockerfile")]
    Docker,
    #[cfg(feature = "tree-sitter-go")]
    Go,
    #[cfg(feature = "tree-sitter-haskell")]
    Haskell,
    // #[cfg(feature = "tree-sitter-html")]
    // Html,
    #[cfg(feature = "tree-sitter-java")]
    Java,
    #[cfg(feature = "tree-sitter-javascript")]
    Js,
    #[cfg(feature = "tree-sitter-json")]
    Json,
    // #[cfg(feature = "tree-sitter-julia")]
    // Julia,
    #[cfg(feature = "tree-sitter-kotlin")]
    Kotlin,
    #[cfg(feature = "tree-sitter-latex")]
    Latex,
    #[cfg(feature = "tree-sitter-lua")]
    Lua,
    #[cfg(feature = "tree-sitter-md")]
    Markdown,
    #[cfg(feature = "tree-sitter-nix")]
    Nix,
    #[cfg(feature = "tree-sitter-ocaml")]
    Ocaml,
    // #[cfg(feature = "tree-sitter-perl")]
    // Perl,
    // #[cfg(feature = "tree-sitter-php")]
    // Php,
    #[cfg(feature = "tree-sitter-python")]
    Python,
    // #[cfg(feature = "tree-sitter-ruby")]
    // Ruby,
    #[cfg(feature = "tree-sitter-rust")]
    Rust,
    // #[cfg(feature = "tree-sitter-scala")]
    // Scala,
    // #[cfg(feature = "tree-sitter-swift")]
    // Swift,
    #[cfg(feature = "tree-sitter-typescript")]
    Ts,
    #[cfg(feature = "tree-sitter-zig")]
    Zig,
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
            .and_then(|e| Self::from_extension(e))
    }

    /// Guesses a language from a plain file extension.
    ///
    /// Examples:
    ///
    /// ```
    /// let lang = tree_painter::Lang::from_extension("rs");
    /// assert!(matches!(Some(tree_painter::Lang::Rust), lang));
    /// ```
    pub fn from_extension(e: &str) -> Option<Self> {
        match e {
            // #[cfg(feature = "tree-sitter-bash")]
            // "sh" => Some(Lang::Bash),
            #[cfg(feature = "tree-sitter-c")]
            "c" => Some(Lang::C),
            #[cfg(feature = "tree-sitter-c-sharp")]
            "cs" => Some(Lang::CSharp),
            #[cfg(feature = "tree-sitter-cpp")]
            "cpp" | "cc" | "cxx" => Some(Lang::Cpp),
            #[cfg(feature = "tree-sitter-dockerfile")]
            "docker" => Some(Lang::Docker),
            // #[cfg(feature = "tree-sitter-clojure")]
            // "clj" | "cljs" | "cljc" => Some(Lang::Clojure),
            #[cfg(feature = "tree-sitter-go")]
            "go" => Some(Lang::Go),
            #[cfg(feature = "tree-sitter-haskell")]
            "hs" | "lhs" => Some(Lang::Haskell),
            // #[cfg(feature = "tree-sitter-html")]
            // "html" => Some(Lang::Html),
            #[cfg(feature = "tree-sitter-java")]
            "java" => Some(Lang::Js),
            #[cfg(feature = "tree-sitter-javascript")]
            "js" => Some(Lang::Js),
            #[cfg(feature = "tree-sitter-json")]
            "json" => Some(Lang::Json),
            // #[cfg(feature = "tree-sitter-julia")]
            // "jl" => Some(Lang::Julia),
            #[cfg(feature = "tree-sitter-kotlin")]
            "kt" => Some(Lang::Kotlin),
            #[cfg(feature = "tree-sitter-latex")]
            "tex" => Some(Lang::Latex),
            #[cfg(feature = "tree-sitter-lua")]
            "lua" => Some(Lang::Lua),
            #[cfg(feature = "tree-sitter-md")]
            "md" => Some(Lang::Markdown),
            #[cfg(feature = "tree-sitter-nix")]
            "nix" => Some(Lang::Nix),
            #[cfg(feature = "tree-sitter-ocaml")]
            "ml" => Some(Lang::Ocaml),
            // #[cfg(feature = "tree-sitter-perl")]
            // "pl" => Some(Lang::Perl),
            // #[cfg(feature = "tree-sitter-php")]
            // "php" => Some(Lang::Php),
            #[cfg(feature = "tree-sitter-python")]
            "py" => Some(Lang::Python),
            // #[cfg(feature = "tree-sitter-ruby")]
            // "rb" => Some(Lang::Rust),
            #[cfg(feature = "tree-sitter-rust")]
            "rs" => Some(Lang::Rust),
            // #[cfg(feature = "tree-sitter-scala")]
            // "scala" | "sc" => Some(Lang::Scala),
            // #[cfg(feature = "tree-sitter-swift")]
            // "swift" => Some(Lang::Swift),
            #[cfg(feature = "tree-sitter-typescript")]
            "ts" => Some(Lang::Ts),
            #[cfg(feature = "tree-sitter-zig")]
            "zig" => Some(Lang::Zig),
            &_ => None,
        }
    }

    fn config(&self) -> HighlightConfiguration {
        match self {
            // #[cfg(feature = "tree-sitter-bash")]
            // Lang::Bash => HighlightConfiguration::new(
            //     tree_sitter_bash::language(),
            //     tree_sitter_bash::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-bash"),
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
            // #[cfg(feature = "tree-sitter-clojure")]
            // Lang::Clojure => HighlightConfiguration::new(
            //     tree_sitter_clojure::language(),
            //     tree_sitter_clojure::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-cpp"),
            #[cfg(feature = "tree-sitter-cpp")]
            Lang::Cpp => HighlightConfiguration::new(
                tree_sitter_cpp::language(),
                tree_sitter_cpp::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-cpp"),
            #[cfg(feature = "tree-sitter-css")]
            Lang::Css => HighlightConfiguration::new(
                tree_sitter_css::language(),
                tree_sitter_css::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-css"),
            #[cfg(feature = "tree-sitter-dockerfile")]
            Lang::Docker => {
                HighlightConfiguration::new(tree_sitter_dockerfile::language(), "", "", "")
                    .expect("loading tree-sitter-dockerfile")
            }
            #[cfg(feature = "tree-sitter-go")]
            Lang::Go => HighlightConfiguration::new(
                tree_sitter_go::language(),
                tree_sitter_go::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-go"),
            #[cfg(feature = "tree-sitter-haskell")]
            Lang::Haskell => HighlightConfiguration::new(
                tree_sitter_haskell::language(),
                tree_sitter_haskell::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-haskell"),
            // #[cfg(feature = "tree-sitter-html")]
            // Lang::Html => HighlightConfiguration::new(
            //     tree_sitter_html::language(),
            //     tree_sitter_html::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-html"),
            #[cfg(feature = "tree-sitter-java")]
            Lang::Java => HighlightConfiguration::new(
                tree_sitter_java::language(),
                tree_sitter_java::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-java"),
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
            #[cfg(feature = "tree-sitter-kotlin")]
            Lang::Kotlin => HighlightConfiguration::new(tree_sitter_kotlin::language(), "", "", "")
                .expect("loading tree-sitter-md"),
            // #[cfg(feature = "tree-sitter-julia")]
            // Lang::Julia => HighlightConfiguration::new(
            //     tree_sitter_julia::language(),
            //     tree_sitter_julia::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-julia"),
            #[cfg(feature = "tree-sitter-latex")]
            Lang::Latex => HighlightConfiguration::new(tree_sitter_latex::language(), "", "", "")
                .expect("loading tree-sitter-json"),
            #[cfg(feature = "tree-sitter-lua")]
            Lang::Lua => HighlightConfiguration::new(tree_sitter_lua::language(), "", "", "")
                .expect("loading tree-sitter-lua"),
            #[cfg(feature = "tree-sitter-md")]
            Lang::Markdown => HighlightConfiguration::new(
                tree_sitter_md::language(),
                tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
                tree_sitter_md::INJECTION_QUERY_BLOCK,
                "",
            )
            .expect("loading tree-sitter-md"),
            #[cfg(feature = "tree-sitter-nix")]
            Lang::Nix => HighlightConfiguration::new(
                tree_sitter_nix::language(),
                tree_sitter_nix::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-ocaml"),
            #[cfg(feature = "tree-sitter-ocaml")]
            Lang::Ocaml => HighlightConfiguration::new(
                tree_sitter_ocaml::language_ocaml(),
                tree_sitter_ocaml::HIGHLIGHTS_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-ocaml"),
            // #[cfg(feature = "tree-sitter-perl")]
            // Lang::Perl => HighlightConfiguration::new(
            //     tree_sitter_perl::language(),
            //     tree_sitter_perl::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-perl"),
            // #[cfg(feature = "tree-sitter-php")]
            // Lang::Php => HighlightConfiguration::new(
            //     tree_sitter_php::language(),
            //     tree_sitter_php::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-php"),
            #[cfg(feature = "tree-sitter-python")]
            Lang::Python => HighlightConfiguration::new(
                tree_sitter_python::language(),
                tree_sitter_python::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-python"),
            // #[cfg(feature = "tree-sitter-ruby")]
            // Lang::Ruby => HighlightConfiguration::new(
            //     tree_sitter_ruby::language(),
            //     tree_sitter_ruby::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-ruby"),
            #[cfg(feature = "tree-sitter-rust")]
            Lang::Rust => HighlightConfiguration::new(
                tree_sitter_rust::language(),
                tree_sitter_rust::HIGHLIGHT_QUERY,
                "",
                "",
            )
            .expect("loading tree-sitter-rust"),
            // #[cfg(feature = "tree-sitter-scala")]
            // Lang::Scala => HighlightConfiguration::new(
            //     tree_sitter_scala::language(),
            //     tree_sitter_scala::HIGHLIGHT_QUERY,
            //     "",
            //     "",
            // )
            // .expect("loading tree-sitter-scala"),
            // #[cfg(feature = "tree-sitter-swift")]
            // Lang::Swift => HighlightConfiguration::new(
            //     tree_sitter_swift::language(),
            //     tree_sitter_swift::HIGHLIGHT_QUERY,
            //     "",
            //     tree_sitter_swift::LOCALS_QUERY,
            // )
            // .expect("loading tree-sitter-swift"),
            Lang::Ts => HighlightConfiguration::new(
                tree_sitter_typescript::language_typescript(),
                tree_sitter_typescript::HIGHLIGHT_QUERY,
                "",
                tree_sitter_typescript::LOCALS_QUERY,
            )
            .expect("loading tree-sitter-typescript"),
            Lang::Zig => HighlightConfiguration::new(
                tree_sitter_zig::language(),
                tree_sitter_zig::HIGHLIGHTS_QUERY,
                tree_sitter_zig::INJECTIONS_QUERY,
                "",
            )
            .expect("loading tree-sitter-zig"),
        }
    }
}

/// Language info.
pub struct Info {
    /// Identifier matching the most common file extension.
    pub id: &'static str,
    /// Human-readable string.
    pub name: &'static str,
}

impl Info {
    const fn new(id: &'static str, name: &'static str) -> Self {
        Self { id, name }
    }
}

/// Language info mappings.
pub const INFOS: [Info; 20] = [
    #[cfg(feature = "tree-sitter-c")]
    Info::new("c", "C"),
    #[cfg(feature = "tree-sitter-cpp")]
    Info::new("cpp", "C++"),
    #[cfg(feature = "tree-sitter-c-sharp")]
    Info::new("cs", "C#"),
    #[cfg(feature = "tree-sitter-css")]
    Info::new("css", "CSS"),
    #[cfg(feature = "tree-sitter-dockerfile")]
    Info::new("dockerfile", "Dockerfile"),
    #[cfg(feature = "tree-sitter-go")]
    Info::new("go", "Go"),
    #[cfg(feature = "tree-sitter-haskell")]
    Info::new("hs", "Haskell"),
    #[cfg(feature = "tree-sitter-java")]
    Info::new("java", "Java"),
    #[cfg(feature = "tree-sitter-javascript")]
    Info::new("js", "JavaScript"),
    #[cfg(feature = "tree-sitter-json")]
    Info::new("json", "JSON"),
    #[cfg(feature = "tree-sitter-kotlin")]
    Info::new("kt", "Kotlin"),
    #[cfg(feature = "tree-sitter-latex")]
    Info::new("tex", "LaTeX"),
    #[cfg(feature = "tree-sitter-lua")]
    Info::new("lua", "Lua"),
    #[cfg(feature = "tree-sitter-md")]
    Info::new("md", "Markdown"),
    #[cfg(feature = "tree-sitter-nix")]
    Info::new("nix", "Nix"),
    #[cfg(feature = "tree-sitter-ocaml")]
    Info::new("ml", "OCaml"),
    #[cfg(feature = "tree-sitter-python")]
    Info::new("py", "Python"),
    #[cfg(feature = "tree-sitter-rust")]
    Info::new("rs", "Rust"),
    #[cfg(feature = "tree-sitter-typescript")]
    Info::new("ts", "TypeScript"),
    #[cfg(feature = "tree-sitter-zig")]
    Info::new("zig", "Zig"),
];

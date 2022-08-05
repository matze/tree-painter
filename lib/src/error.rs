/// Errors occuring during parsing of themes and rendering.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Helix theme could not be parsed as valid TOML
    #[error("toml parse eror: {0}")]
    Toml(#[from] toml::de::Error),
    /// TOML data is not structured like a valid Helix theme.
    #[error("toml does not contain valid helix theme")]
    InvalidTheme,
    /// A color is referenced but is not defined.
    #[error("toml color {0} not found")]
    InvalidColorReference(String),
    /// Failed to highlight source.
    #[error("highlighting failed: {0}")]
    Highlighting(#[from] tree_sitter_highlight::Error),
}

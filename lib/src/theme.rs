use crate::renderer::HIGHLIGHT_NAMES;
use crate::Error;
use std::collections::HashMap;
use std::convert::From;
use toml::value::Table;
use toml::Value;

pub(crate) struct Style {
    pub color: String,
    pub is_bold: bool,
    pub is_italic: bool,
}

impl From<&String> for Style {
    fn from(color: &String) -> Self {
        Style {
            color: color.clone(),
            is_bold: false,
            is_italic: false,
        }
    }
}

/// A theme defining colors and modifiers to be used for syntax highlighting.
pub struct Theme {
    pub(crate) style_map: HashMap<usize, Style>,
    pub(crate) foreground: Style,
    pub(crate) background: Style,
}

impl Theme {
    /// Load theme from a Helix [compatible](https://docs.helix-editor.com/themes.html) theme
    /// description stored in `data`.
    ///
    /// # Errors
    ///
    /// If the theme cannot be parsed either because it is not a TOML file or does not adhere to
    /// the Helix syntax expectations, this function returns an [`Error`].
    pub fn from_helix(data: &str) -> Result<Self, Error> {
        let root = match data.parse::<toml::Value>()? {
            Value::Table(table) => table,
            _ => return Err(Error::InvalidTheme),
        };

        let palette = root.get("palette").ok_or(Error::InvalidTheme)?;

        let referenced_color = |table: &Table, name: &str| -> Result<Style, Error> {
            if let Some(Value::String(reference)) = table.get(name) {
                if let Some(Value::String(color)) = palette.get(reference) {
                    return Ok(Style::from(color));
                }
            }

            Err(Error::InvalidColorReference(name.to_string()))
        };

        let fg_color = |name: &str| -> Result<Option<Style>, Error> {
            if let Some(value) = root.get(name) {
                match value {
                    Value::String(reference) => {
                        if let Some(Value::String(color)) = palette.get(reference) {
                            return Ok(Some(Style::from(color)));
                        }
                    }
                    Value::Table(table) => {
                        let mut style = referenced_color(table, "fg")?;

                        if let Some(Value::Array(modifiers)) = table.get("modifiers") {
                            for modifier in modifiers {
                                if let Value::String(modifier) = modifier {
                                    if modifier == "italic" {
                                        style.is_italic = true;
                                    } else if modifier == "bold" {
                                        style.is_bold = true;
                                    }
                                }
                            }
                        }

                        return Ok(Some(style));
                    }
                    _ => {}
                }
            }

            Ok(None)
        };

        let mut style_map = HashMap::default();

        for (index, name) in HIGHLIGHT_NAMES.iter().enumerate() {
            if let Some(style) = fg_color(*name)? {
                style_map.insert(index, style);
            }
        }

        let background = match root.get("ui.background") {
            Some(Value::Table(table)) => referenced_color(table, "bg")?,
            _ => Style::from(&"#000".to_string()),
        };

        let foreground = fg_color("ui.text")?.unwrap_or_else(|| Style::from(&"#fff".to_string()));

        Ok(Self {
            style_map,
            foreground,
            background,
        })
    }
}

use crate::{theme, Error, Lang};
use std::collections::HashMap;
use std::fmt::Write;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, Highlighter, HtmlRenderer};

pub(crate) const HIGHLIGHT_NAMES: [&str; 27] = [
    "attribute",
    "comment",
    "constant",
    "constant.builtin",
    "constructor",
    "escape",
    "function",
    "function.builtin",
    "function.method",
    "function.macro",
    "include",
    "keyword",
    "label",
    "namespace",
    "number",
    "operator",
    "property",
    "punctuation",
    "punctuation.bracket",
    "punctuation.delimiter",
    "repeat",
    "string",
    "type",
    "type.builtin",
    "variable",
    "variable.builtin",
    "variable.parameter",
];

/// HTML syntax highlighting renderer.
pub struct Renderer {
    renderer: HtmlRenderer,
    theme: theme::Theme,
    css_classes: HashMap<usize, String>,
    configs: HashMap<Lang, HighlightConfiguration>,
}

impl Renderer {
    /// Create a new renderer based on `theme`.
    pub fn new(theme: theme::Theme) -> Self {
        let mut css_classes = HashMap::default();

        for index in theme.style_map.keys() {
            css_classes.insert(
                *index,
                format!(r#"class="tsc-{}""#, HIGHLIGHT_NAMES[*index]),
            );
        }

        Self {
            renderer: HtmlRenderer::new(),
            theme,
            css_classes,
            configs: HashMap::default(),
        }
    }

    /// Generate CSS block to be included in the `<style></style>` block or in an external CSS
    /// file.
    pub fn css(&self) -> String {
        let mut css = String::new();

        let _ = writeln!(
            css,
            ":root {{ --tsc-main-fg-color: {}; --tsc-main-bg-color: {}; }}",
            self.theme.foreground.color, self.theme.background.color
        );

        for (index, style) in &self.theme.style_map {
            let _ = write!(
                css,
                ".tsc-{} {{ color: {};",
                HIGHLIGHT_NAMES[*index], style.color
            );

            if style.is_bold {
                css.push_str("font-weight: bold;");
            }

            if style.is_italic {
                css.push_str("font-style: italic;");
            }

            css.push_str("}\n");
        }

        css.push_str(".tsc-line { word-wrap: normal; white-space: pre; }\n");
        css
    }

    /// Render `source` based on the `lang`.
    pub fn render<'a>(
        &'a mut self,
        lang: &Lang,
        source: &[u8],
    ) -> Result<impl Iterator<Item = &'a str>, Error> {
        fn foo<'a>(_: &str) -> Option<&'a HighlightConfiguration> {
            None
        }

        let config = match self.configs.get(lang) {
            Some(config) => config,
            None => {
                let mut config = lang.config();
                config.configure(&HIGHLIGHT_NAMES);
                self.configs.insert(lang.clone(), config);
                self.configs.get(lang).unwrap()
            }
        };

        let mut highlighter = Highlighter::new();
        let events = highlighter.highlight(config, source, None, foo)?;

        self.renderer.reset();
        self.renderer.render(
            events,
            source,
            &|attr: Highlight| match self.css_classes.get(&attr.0) {
                Some(class) => class.as_bytes(),
                None => "".as_bytes(),
            },
        )?;

        Ok(self.renderer.lines())
    }
}

use cosmic::iced::Color;
use cosmic::iced_core::text::Highlighter;
use std::ops::Range;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

pub struct SyntectHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
    theme: Theme,
    syntax: SyntaxReference,
    current_extension: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SyntectSettings {
    pub theme: String,
    pub extension: String,
}

impl Default for SyntectSettings {
    fn default() -> Self {
        Self {
            theme: "base16-ocean.dark".to_string(),
            extension: "md".to_string(),
        }
    }
}

impl SyntectHighlighter {
    fn load_syntax(&mut self, extension: &str) {
        self.syntax = self
            .syntax_set
            .find_syntax_by_extension(extension)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
            .clone();
        self.current_extension = extension.to_string();
    }

    fn load_theme(&mut self, theme_name: &str) {
        self.theme = self
            .theme_set
            .themes
            .get(theme_name)
            .unwrap_or(&self.theme_set.themes["base16-ocean.dark"])
            .clone();
    }
}

impl Highlighter for SyntectHighlighter {
    type Settings = SyntectSettings;
    type Highlight = Style;
    type Iterator<'a> = Box<dyn Iterator<Item = (Range<usize>, Self::Highlight)> + 'a>;

    fn new(settings: &Self::Settings) -> Self {
        let syntax_set = SyntaxSet::load_defaults_newlines();
        let theme_set = ThemeSet::load_defaults();

        let syntax = syntax_set
            .find_syntax_by_extension(&settings.extension)
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text())
            .clone();

        let theme = theme_set
            .themes
            .get(&settings.theme)
            .unwrap_or(&theme_set.themes["base16-ocean.dark"])
            .clone();

        Self {
            syntax_set,
            theme_set,
            theme,
            syntax,
            current_extension: settings.extension.clone(),
        }
    }

    fn update(&mut self, settings: &Self::Settings) {
        if self.current_extension != settings.extension {
            self.load_syntax(&settings.extension);
        }
        self.load_theme(&settings.theme);
    }

    fn change_line(&mut self, _line: usize) {
        // Called when the current line changes
    }

    fn highlight_line(&mut self, line: &str) -> Self::Iterator<'_> {
        let mut highlighter = HighlightLines::new(&self.syntax, &self.theme);

        let ranges: Vec<(Range<usize>, Style)> = highlighter
            .highlight_line(line, &self.syntax_set)
            .unwrap_or_default()
            .into_iter()
            .scan(0, |offset, (style, text)| {
                let start = *offset;
                let end = start + text.len();
                *offset = end;
                Some((start..end, style))
            })
            .collect();

        Box::new(ranges.into_iter())
    }

    fn current_line(&self) -> usize {
        0
    }
}

pub fn to_format(
    style: &Style,
) -> cosmic::iced_core::text::highlighter::Format<cosmic::font::Font> {
    cosmic::iced_core::text::highlighter::Format {
        color: Some(Color::from_rgb8(
            style.foreground.r,
            style.foreground.g,
            style.foreground.b,
        )),
        font: None,
    }
}

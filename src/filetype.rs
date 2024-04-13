use std::path::Path;

pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default, Copy, Clone)]
pub struct HighlightingOptions {
    /// Whether to highlight numbers.
    numbers: bool,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No filetype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn highlight_options(&self) -> HighlightingOptions {
        self.hl_opts
    }

    #[must_use]
    pub fn from(filename: &str) -> Self {
        let filename = Path::new(filename);
        if filename
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("rs"))
        {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions { numbers: true },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    #[must_use]
    pub fn numbers(self) -> bool {
        self.numbers
    }
}

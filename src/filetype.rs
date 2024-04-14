use std::path::Path;

pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    keywords: Vec<String>,
    data_types: Vec<String>,
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
    pub fn highlight_options(&self) -> &HighlightingOptions {
        &self.hl_opts
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
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    // The currently in used keywords in Rust:
                    // https://doc.rust-lang.org/book/appendix-01-keywords.html#keywords-currently-in-use
                    keywords: vec![
                        "as".to_owned(),
                        "async".to_owned(),
                        "await".to_owned(),
                        "break".to_owned(),
                        "const".to_owned(),
                        "continue".to_owned(),
                        "crate".to_owned(),
                        "dyn".to_owned(),
                        "else".to_owned(),
                        "enum".to_owned(),
                        "extern".to_owned(),
                        "false".to_owned(),
                        "fn".to_owned(),
                        "for".to_owned(),
                        "if".to_owned(),
                        "impl".to_owned(),
                        "in".to_owned(),
                        "let".to_owned(),
                        "loop".to_owned(),
                        "match".to_owned(),
                        "mod".to_owned(),
                        "move".to_owned(),
                        "mut".to_owned(),
                        "pub".to_owned(),
                        "ref".to_owned(),
                        "return".to_owned(),
                        "Self".to_owned(),
                        "self".to_owned(),
                        "static".to_owned(),
                        "struct".to_owned(),
                        "super".to_owned(),
                        "trait".to_owned(),
                        "true".to_owned(),
                        "type".to_owned(),
                        "union".to_owned(),
                        "unsafe".to_owned(),
                        "use".to_owned(),
                        "where".to_owned(),
                        "while".to_owned(),
                    ],
                    // The data types in Rust.
                    data_types: vec![
                        "i8".to_owned(),
                        "i16".to_owned(),
                        "i32".to_owned(),
                        "i64".to_owned(),
                        "i128".to_owned(),
                        "u8".to_owned(),
                        "u16".to_owned(),
                        "u32".to_owned(),
                        "u64".to_owned(),
                        "u128".to_owned(),
                        "f32".to_owned(),
                        "f64".to_owned(),
                        "isize".to_owned(),
                        "usize".to_owned(),
                        "bool".to_owned(),
                        "char".to_owned(),
                        "str".to_owned(),
                        "String".to_owned(),
                        "Box".to_owned(),
                        "Rc".to_owned(),
                        "Arc".to_owned(),
                        "Vec".to_owned(),
                        "HashMap".to_owned(),
                        "BTreeMap".to_owned(),
                        "HashSet".to_owned(),
                        "BTreeSet".to_owned(),
                        "Option".to_owned(),
                        "Result".to_owned(),
                        "Some".to_owned(),
                        "None".to_owned(),
                        "Ok".to_owned(),
                        "Err".to_owned(),
                        "true".to_owned(),
                        "false".to_owned(),
                    ],
                },
            };
        }
        Self::default()
    }
}

impl HighlightingOptions {
    #[must_use]
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    #[must_use]
    pub fn strings(&self) -> bool {
        self.strings
    }

    #[must_use]
    pub fn characters(&self) -> bool {
        self.characters
    }

    #[must_use]
    pub fn comments(&self) -> bool {
        self.comments
    }

    #[must_use]
    pub fn keywords(&self) -> &Vec<String> {
        &self.keywords
    }

    #[must_use]
    pub fn data_types(&self) -> &Vec<String> {
        &self.data_types
    }
}

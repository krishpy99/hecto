use termion::color;

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct HighlightingOptions {
    pub numbers: bool,
    pub strings: bool,
    pub characters: bool,
    pub comments: bool,
    pub multiline_comments: bool,
    pub keywords: Vec<String>,
    pub data_types: Vec<String>,
}

#[allow(clippy::enum_variant_names)] // The word "Type" in DataType has different meaning.
#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    None,
    Number,
    // Search results.
    Search,
    String,
    Character,
    Comment,
    MultilineComment,
    Keyword,
    DataType,
}

impl Type {
    pub fn as_color(&self) -> &dyn color::Color {
        match self {
            Type::Number => &color::Rgb(255, 128, 0), // Orange
            Type::Search => &color::Blue,
            Type::String => &color::Yellow,
            Type::Character => &color::LightBlue,
            Type::Comment | Type::MultilineComment => &color::LightBlack,
            Type::Keyword => &color::Magenta,
            Type::DataType => &color::LightMagenta,
            Type::None => &color::Reset,
        }
    }
}

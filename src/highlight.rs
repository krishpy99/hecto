use termion::color;

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
    Keyword,
    DataType,
}

impl Type {
    pub fn as_color(&self) -> &dyn color::Color {
        match self {
            Type::Number => &color::Red,
            Type::Search => &color::Blue,
            Type::String => &color::Green,
            Type::Character => &color::LightBlue,
            Type::Comment => &color::LightBlack,
            Type::Keyword => &color::Magenta,
            Type::DataType => &color::LightMagenta,
            Type::None => &color::Reset,
        }
    }
}

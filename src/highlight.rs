use termion::color;

#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    None,
    Number,
    // Search results.
    Search,
    String,
    Character,
    Comment,
}

impl Type {
    pub fn as_color(&self) -> &dyn color::Color {
        match self {
            Type::Number => &color::Red,
            Type::Search => &color::Blue,
            Type::String => &color::Magenta,
            Type::Character => &color::LightBlue,
            Type::Comment => &color::LightBlack,
            Type::None => &color::Reset,
        }
    }
}

use termion::color;

#[derive(PartialEq, Copy, Clone)]
pub enum Type {
    None,
    Number,
    Search,
}

impl Type {
    pub fn as_color(&self) -> &dyn color::Color {
        match self {
            Type::Number => &color::Red,
            Type::Search => &color::Blue,
            Type::None => &color::Reset,
        }
    }
}

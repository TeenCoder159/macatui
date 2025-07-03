#[derive(Default)]
pub enum Alignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Default)]
pub enum Thickness {
    #[default]
    Normal,
    Bold,
    Thin,
    Custom(u64),
}

#[derive(Default)]
pub enum Styling {
    #[default]
    Normal,
    Italicised,
    Strikethrough,
}

#[derive(Default)]
pub enum Color {
    #[default]
    Normal,
    Black,
    Red,
    Blue,
    Custom(),
}

#[derive(Default)]
pub enum Font {
    #[default]
    Normal,
}

#[derive(Default)]
pub struct Border {
    border_radius: u32,
    border_width: u32,
    border_colour: Color,
}

impl Border {
    pub fn set_radius(&mut self, radius: u32) {
        self.border_radius = radius;
    }

    pub fn set_width(&mut self, width: u32) {
        self.border_width = width;
    }

    pub fn set_color(&mut self, color: Color) {
        self.border_colour = color;
    }
}

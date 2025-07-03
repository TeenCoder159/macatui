use crate::rendering::widgets::defaults::*;

#[derive(Default)]
pub struct Text {
    content: String,
    style: Styling,
    color: Color,
    horizontal_align: Alignment,
    vertical_align: Alignment,
    padding: (u32, u32, u32, u32),
    margin: (u32, u32, u32, u32),
    border: Option<Border>,
}

impl Text {
    pub fn new(content: &str) -> Self {
        let mut text = Text::default();
        text.content = content.to_owned();
        text
    }

    pub fn style(&mut self, style: Styling) {
        self.style = style;
    }

    pub fn color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn align_horizontal(&mut self, horizontal_align: Alignment) {
        self.horizontal_align = horizontal_align;
    }

    pub fn align_vertical(&mut self, vertical_align: Alignment) {
        self.vertical_align = vertical_align;
    }

    pub fn padding(
        &mut self,
        left_padding: u32,
        top_padding: u32,
        right_padding: u32,
        bottom_padding: u32,
    ) {
        self.padding = (left_padding, top_padding, right_padding, bottom_padding);
    }

    pub fn margin(
        &mut self,
        left_margin: u32,
        top_margin: u32,
        right_margin: u32,
        bottom_margin: u32,
    ) {
        self.margin = (left_margin, top_margin, right_margin, bottom_margin);
    }

    pub fn border(&mut self, border: Border) {
        self.border = Some(border);
    }
}

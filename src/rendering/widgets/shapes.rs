use crate::rendering::actions::move_cursor;
use std::io::{self, Write};

/// Draw a quadrilateral at pos_x with a width and height
pub fn draw_quad(pos_x: u16, pos_y: u16, width: u16, height: u16) {
    assert!(width >= 2 && height >= 2, "Square must be at least 2x2");

    // Move cursor to starting position
    move_cursor(pos_x, pos_y);
    print!("┌{}┐", "─".repeat((width - 2) as usize));

    // Draw sides
    for i in 1..(height - 1) {
        move_cursor(pos_x, pos_y + i);
        print!("│{}│", " ".repeat((width - 2) as usize));
    }

    // Draw bottom
    move_cursor(pos_x, pos_y + height - 1);
    print!("└{}┘", "─".repeat((width - 2) as usize));

    io::stdout().flush().unwrap();
}

pub fn clear() {
    print!("\x1B[2J\x1B[H");
}

pub fn move_cursor(x: u16, y: u16) {
    // ANSI escape code to move cursor to (x,y), 1-indexed
    print!("\x1B[{};{}H", y + 1, x + 1);
}

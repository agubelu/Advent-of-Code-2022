#[cfg(windows)]
pub const DOUBLE_NEWLINE: &str = "\r\n\r\n";
#[cfg(not(windows))]
pub const DOUBLE_NEWLINE: &str = "\n\n";

// General directions to move around a 2D array
pub const UP: (i32, i32) = (0, -1);
pub const DOWN: (i32, i32) = (0, 1);
pub const RIGHT: (i32, i32) = (1, 0);
pub const LEFT: (i32, i32) = (-1, 0);

pub type Pos2D = (usize, usize);

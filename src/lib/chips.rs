use colored::{ColoredString, Colorize};

pub fn x_chip() -> ColoredString {
    "X".blue().bold()
}
pub fn o_chip() -> ColoredString {
    "O".magenta().bold()
}
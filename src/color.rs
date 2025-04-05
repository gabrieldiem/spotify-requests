pub struct Color;

#[allow(dead_code)]
impl Color {
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const WHITE: &'static str = "\x1b[37m";

    pub const BOLD_RED: &'static str = "\x1b[1;31m";
    pub const BOLD_GREEN: &'static str = "\x1b[1;32m";
    pub const BOLD_YELLOW: &'static str = "\x1b[1;33m";
    pub const BOLD_BLUE: &'static str = "\x1b[1;34m";
    pub const BOLD_WHITE: &'static str = "\x1b[1;37m";

    pub const RESET: &'static str = "\x1b[0m";
}

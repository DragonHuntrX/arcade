/*
    This is a bunch of colors you can add to &strs to change their color.
    B siffix means it is a background.
    B prefix is bright.
*/

/* Foreground */
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const BBLACK: &str = "\x1b[90m";
pub const BRED: &str = "\x1b[91m";
pub const BGREEN: &str = "\x1b[92m";
pub const BYELLOW: &str = "\x1b[99m";
pub const BBLUE: &str = "\x1b[94m";
pub const BMAGENTA: &str = "\x1b[95m";
pub const BCYAN: &str = "\x1b[96m";
pub const BWHITE: &str = "\x1b[97m";

/* Background */
pub const BLACKB: &str = "\x1b[40m";
pub const REDB: &str = "\x1b[41m";
pub const GREENB: &str = "\x1b[42m";
pub const YELLOWB: &str = "\x1b[43m";
pub const BLUEB: &str = "\x1b[44m";
pub const MAGENTAB: &str = "\x1b[45m";
pub const CYANB: &str = "\x1b[46m";
pub const WHITEB: &str = "\x1b[47m";

pub const BBLACKB: &str = "\x1b[100m";
pub const BREDB: &str = "\x1b[101m";
pub const BGREENB: &str = "\x1b[102m";
pub const BYELLOWB: &str = "\x1b[103m";
pub const BBLUEB: &str = "\x1b[104m";
pub const BMAGENTAB: &str = "\x1b[105m";
pub const BCYANB: &str = "\x1b[106m";
pub const BWHITEB: &str = "\x1b[107m";

/* Effects */
pub const CLEAR: &str = "\x1b[2J";
pub const DEFAULT: &str = "\x1b[0m";


// Styles: 0-29.

pub const RESET:                    &'static str = "\x1b[0m";
pub const BOLD:                     &'static str = "\x1b[1m";
pub const DIM:                      &'static str = "\x1b[2m";
pub const ITALIC:                   &'static str = "\x1b[3m";
pub const UNDERLINE:                &'static str = "\x1b[4m";
pub const SLOW_BLINK:               &'static str = "\x1b[5m";
pub const RAPID_BLINK:              &'static str = "\x1b[6m";
pub const INVERTED:                 &'static str = "\x1b[7m";
pub const HIDDEN:                   &'static str = "\x1b[8m";
pub const STRIKETHROUGH:            &'static str = "\x1b[9m";
pub const PRIMARY_FONT:             &'static str = "\x1b[10m";
pub const ALT_FONT_1:               &'static str = "\x1b[11m";
pub const ALT_FONT_2:               &'static str = "\x1b[12m";
pub const ALT_FONT_3:               &'static str = "\x1b[13m";
pub const ALT_FONT_4:               &'static str = "\x1b[14m";
pub const ALT_FONT_5:               &'static str = "\x1b[15m";
pub const ALT_FONT_6:               &'static str = "\x1b[16m";
pub const ALT_FONT_7:               &'static str = "\x1b[17m";
pub const ALT_FONT_8:               &'static str = "\x1b[18m";
pub const ALT_FONT_9:               &'static str = "\x1b[19m";
pub const FRAKTUR_FONT:             &'static str = "\x1b[20m";
pub const DOUBLE_UNDERLINE:         &'static str = "\x1b[21m"; // identical because of competing standards
pub const NOT_BOLD:                 &'static str = "\x1b[21m"; // identical because of competing standards
pub const NORMAL_INTENSITY:         &'static str = "\x1b[22m";
pub const NOT_ITALIC_NOT_BOLD:      &'static str = "\x1b[23m";
pub const NOT_UNDERLINED:           &'static str = "\x1b[24m";
pub const NOT_BLINKING:             &'static str = "\x1b[25m";
pub const PROPORTIONAL_SPACING:     &'static str = "\x1b[26m";
pub const NOT_INVERTED:             &'static str = "\x1b[27m";
pub const NOT_HIDDEN:               &'static str = "\x1b[28m";
pub const NOT_STRIKETHROUGH:        &'static str = "\x1b[29m";

pub mod fg {

    // Standard foreground colors: 30-37.

    pub const BLACK:                &'static str = "\x1b[30m";
    pub const RED:                  &'static str = "\x1b[31m";
    pub const GREEN:                &'static str = "\x1b[32m";
    pub const YELLOW:               &'static str = "\x1b[33m";
    pub const BLUE:                 &'static str = "\x1b[34m";
    pub const PURPLE:               &'static str = "\x1b[35m";
    pub const CYAN:                 &'static str = "\x1b[36m";
    pub const WHITE:                &'static str = "\x1b[37m";

    // Fine-control foreground colors: 38.

    pub fn color_256(n: u8) -> String {
        format!("\x1b[38;5;{n}m")
    }

    pub fn truecolor(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{r};{g};{b}m")
    }

    // Reset foreground color: 39.

    pub const DEFAULT:              &'static str = "\x1b[39m";

    // Bright foreground colors: 90-97 (discontinuous range).

    pub const BRIGHT_BLACK:         &'static str = "\x1b[90m";
    pub const BRIGHT_RED:           &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN:         &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW:        &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE:          &'static str = "\x1b[94m";
    pub const BRIGHT_PURPLE:        &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN:          &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE:         &'static str = "\x1b[97m";
}

pub mod bg {

    // Standard background colors: 40-47.

    pub const BLACK:                &'static str = "\x1b[40m";
    pub const RED:                  &'static str = "\x1b[41m";
    pub const GREEN:                &'static str = "\x1b[42m";
    pub const YELLOW:               &'static str = "\x1b[43m";
    pub const BLUE:                 &'static str = "\x1b[44m";
    pub const PURPLE:               &'static str = "\x1b[45m";
    pub const CYAN:                 &'static str = "\x1b[46m";
    pub const WHITE:                &'static str = "\x1b[47m";

    // Fine-control background colors: 48.

    pub fn color_256(n: u8) -> String {
        format!("\x1b[48;5;{n}m")
    }

    pub fn truecolor(r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{r};{g};{b}m")
    }

    // Reset background color: 49.

    pub const DEFAULT:              &'static str = "\x1b[49m";

    // Bright background colors: 100-107 (discontinuous range).

    pub const BRIGHT_BLACK:         &'static str = "\x1b[100m";
    pub const BRIGHT_RED:           &'static str = "\x1b[101m";
    pub const BRIGHT_GREEN:         &'static str = "\x1b[102m";
    pub const BRIGHT_YELLOW:        &'static str = "\x1b[103m";
    pub const BRIGHT_BLUE:          &'static str = "\x1b[104m";
    pub const BRIGHT_PURPLE:        &'static str = "\x1b[105m";
    pub const BRIGHT_CYAN:          &'static str = "\x1b[106m";
    pub const BRIGHT_WHITE:         &'static str = "\x1b[107m";
}

// Additional styles: 50-55.

pub const NO_PROPORTIONAL_SPACING:  &'static str = "\x1b[50m";
pub const FRAMED:                   &'static str = "\x1b[51m";
pub const ENCIRCLED:                &'static str = "\x1b[52m";
pub const OVERLINE:                 &'static str = "\x1b[53m";
pub const NOT_FRAMED_NOT_ENCIRCLED: &'static str = "\x1b[54m";
pub const NOT_OVERLINED:            &'static str = "\x1b[55m";

// Test (requires manual inspection of outputs).

#[cfg(test)]
mod tests {
    use super::*;
    use super::fg::*;

    #[test]
    fn print_and_verify_visually() {
        println!();
        println!("{GREEN}green{RESET}");
        println!("{BOLD}{RED}BOLD RED{RESET}");
        println!("normal {WHITE}white {BRIGHT_WHITE}bright white{RESET}");
        println!("{BLUE}blue {BRIGHT_BLUE}bright blue{RESET}");
        println!("normal {ITALIC}italic {BOLD}and bold {UNDERLINE}and underline{RESET}");
        println!("normal {DOUBLE_UNDERLINE}double underline{RESET}");
        println!("normal normal");
        println!("normal {OVERLINE}overline{RESET}");
        println!("normal {ENCIRCLED}encircled{RESET}");
        println!("normal {FRAMED}framed{RESET}");
        println!(
            "{}g{}r{}e{}y{}s{}c{}a{}l{}e{} {}c{}o{}l{}o{}r{}s{}",
            fg::color_256(232),
            fg::color_256(235),
            fg::color_256(238),
            fg::color_256(241),
            fg::color_256(244),
            fg::color_256(247),
            fg::color_256(250),
            fg::color_256(253),
            fg::color_256(255),
            RESET,
            bg::color_256(255),
            bg::color_256(251),
            bg::color_256(247),
            bg::color_256(243),
            bg::color_256(239),
            bg::color_256(235),
            RESET,
        );
        println!(
            "{}r{}a{}i{}n{}b{}o{}w{}i{}c{} {}c{}o{}l{}o{}r{}s{}",
            fg::color_256(132),
            fg::color_256(135),
            fg::color_256(138),
            fg::color_256(141),
            fg::color_256(144),
            fg::color_256(147),
            fg::color_256(150),
            fg::color_256(153),
            fg::color_256(155),
            RESET,
            bg::color_256(155),
            bg::color_256(151),
            bg::color_256(147),
            bg::color_256(143),
            bg::color_256(139),
            bg::color_256(135),
            RESET,
        );

        println!(
            "{BOLD}{}t{}r{}u{}e{}c{}o{}l{}o{}r{} {BOLD}{}r{}a{}i{}n{}b{}o{}w{}",
            fg::truecolor(255, 0, 0),
            fg::truecolor(170, 0, 0),
            fg::truecolor(90, 0, 0),
            fg::truecolor(30, 0, 0),
            fg::truecolor(0, 30, 0),
            fg::truecolor(0, 90, 0),
            fg::truecolor(0, 170, 0),
            fg::truecolor(0, 200, 0),
            fg::truecolor(0, 255, 0),
            RESET,
            fg::truecolor(0, 255, 0),
            fg::truecolor(0, 150, 0),
            fg::truecolor(0, 50, 0),
            fg::truecolor(0, 0, 50),
            fg::truecolor(0, 0, 150),
            fg::truecolor(0, 0, 170),
            fg::truecolor(0, 0, 255),
            RESET,
        );
        println!("hidden: {HIDDEN}hidden{NOT_HIDDEN} revealed");
        println!("{}green fg {}reset fg", fg::truecolor(0, 255, 0), fg::DEFAULT);
        println!("{}green bg {}reset bg", bg::truecolor(0, 255, 0), bg::DEFAULT);
        println!();
    }
}


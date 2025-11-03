
#![warn(missing_docs)]

//! Constants and simple functions for invoking ANSI control codes used for text-styling in terminals (including color codes). No support for cursor movement or any other control codes.
//!
//! This crate provides constant bindings for text-styling ANSI control codes like `BOLD` (bound to the string `\x1b[1m`) and `GREEN` (bound to `\x1b[32m`):
//!
//! ```
//! use flower_pot::*;
//!
//! println!("{GREEN}ok{RESET}");           // prints a green "ok"
//! println!("{BOLD}{RED}error!{RESET}");   // prints a bold, red "error!"
//! println!("{BLUE_BG}cloud{RESET}");      // prints white text on a blue background
//!
//! // Note that you must print the RESET code after the end of
//! // the text you want styled, or else all text printed to the
//! // terminal after that point will also be styled that way,
//! // including text outputted by other programs.
//!
//! ```
//!
//! It also provides functions to invoke the 8-bit color palette:
//!
//! ```
//! use flower_pot::*;
//!
//! // Prints text in "palette-color #237" (often a shade of grey)
//! // with a background color of "palette-color #214" (often a
//! // shade of orange):
//!
//! println!("{}{}example text{RESET}", color_256(237), color_256_bg(214));
//!
//! ```
//!
//! And functions that invoke truecolor functionality for terminals that support it:
//!
//! ```
//! use flower_pot::*;
//!
//! // Prints text in RGB color [127, 45, 68] with a background
//! // color of RGB color [0, 255, 255]:
//!
//! println!("{}{}example text{RESET}", truecolor(127, 45, 68), truecolor_bg(0, 255, 255));
//!
//! ```
//!
//! The functions `color_256`, `color_256_bg`, `truecolor`, and `truecolor_bg` all return Strings.
//!
//! Note that not all terminals support all of the codes defined in this library. The basic workflow of ANSI control codes is that a program outputs sequences of special characters describing what it wants (such as "make the following text bold" or "make the following text green") to stdout, and then the terminal that the program is running in decides what to do with those characters. The codes themselves are reasonably well-standardized, but not every terminal understands all of them. Some terminals might ignore some codes, or might do weird things when you use them (such as displaying the text following the code incorrectly). This is a feature of the ANSI control code ecosystem, and not something a library can fix.
//!
//! Once you've outputted a control code, all text that follows it will be styled in the manner requested. If you want to go back to unstyled text, output the `RESET` code or one of the more specific style-resetting codes such as `NOT_UNDERLINED`.
//!
//! The list of control codes is taken from [the Wikipedia page on ANSI control codes](https://en.wikipedia.org/wiki/ANSI_escape_code). Codes which are not widely supported (as reported by Wikipedia) are marked as such below.

// Styles: 0-29.

/// Unset all styles and return to default text formatting.
pub const RESET:                    &'static str = "\x1b[0m";
/// Make the following text bold.
pub const BOLD:                     &'static str = "\x1b[1m";
/// Make the following text dim.
pub const DIM:                      &'static str = "\x1b[2m";
/// Make the following text italic.
pub const ITALIC:                   &'static str = "\x1b[3m";
/// Underline the following text.
pub const UNDERLINE:                &'static str = "\x1b[4m";
/// Make the following text blink slowly.
pub const SLOW_BLINK:               &'static str = "\x1b[5m";
/// Make the following text blink quickly. Not widely supported according to Wikipedia.
pub const RAPID_BLINK:              &'static str = "\x1b[6m";
/// Swap the current foreground color and current background color for the following text.
pub const INVERTED:                 &'static str = "\x1b[7m";
/// Hide the following text. Not widely supported according to Wikipedia.
pub const HIDDEN:                   &'static str = "\x1b[8m";
/// Make the following text strikethrough. Not supported in Terminal.app according to Wikipedia.
pub const STRIKETHROUGH:            &'static str = "\x1b[9m";
/// Switch to the default font.
pub const DEFAULT_FONT:             &'static str = "\x1b[10m";
/// Switch to alternative font #1.
pub const ALT_FONT_1:               &'static str = "\x1b[11m";
/// Switch to alternative font #2.
pub const ALT_FONT_2:               &'static str = "\x1b[12m";
/// Switch to alternative font #3.
pub const ALT_FONT_3:               &'static str = "\x1b[13m";
/// Switch to alternative font #4.
pub const ALT_FONT_4:               &'static str = "\x1b[14m";
/// Switch to alternative font #5.
pub const ALT_FONT_5:               &'static str = "\x1b[15m";
/// Switch to alternative font #6.
pub const ALT_FONT_6:               &'static str = "\x1b[16m";
/// Switch to alternative font #7.
pub const ALT_FONT_7:               &'static str = "\x1b[17m";
/// Switch to alternative font #8.
pub const ALT_FONT_8:               &'static str = "\x1b[18m";
/// Switch to alternative font #9.
pub const ALT_FONT_9:               &'static str = "\x1b[19m";
/// Switch to Fraktur font. Rarely supported according to Wikipedia.
pub const FRAKTUR:                  &'static str = "\x1b[20m";
/// Double-underline the following text. **WARNING:** this constant contains the exact same control
/// code as the constant `NOT_BOLD`, because different terminals interpret the code to mean
/// different things. If you use either constant, be aware that your text may be rendered
/// differently by different terminals.
pub const DOUBLE_UNDERLINE:         &'static str = "\x1b[21m";
/// Make the following text not bold. **WARNING:** this constant contains the exact same control
/// code as the constant `DOUBLE_UNDERLINE`, because different terminals interpret the code to mean
/// different things. If you use either constant, be aware that your text may be rendered
/// differently by different terminals.
pub const NOT_BOLD:                 &'static str = "\x1b[21m";
/// Return to ordinary intensity (neither bold nor dim) for the following text.
pub const NORMAL_INTENSITY:         &'static str = "\x1b[22m";
/// Make the following text neither bold nor italic.
pub const NEITHER_BOLD_NOR_ITALIC:  &'static str = "\x1b[23m";
/// Make the following text not underlined.
pub const NOT_UNDERLINED:           &'static str = "\x1b[24m";
/// Make the following text not blink.
pub const NOT_BLINKING:             &'static str = "\x1b[25m";
/// Use a font with proportional spacing (i.e., a non-monospace font) for the following text.
/// Rarely supported according to Wikipedia.
pub const PROPORTIONAL_SPACING:     &'static str = "\x1b[26m";
/// Unswap the foreground and background colors for the following text.
pub const NOT_INVERTED:             &'static str = "\x1b[27m";
/// Make the following text not hidden.
pub const NOT_HIDDEN:               &'static str = "\x1b[28m";
/// Make the following text not strikethrough.
pub const NOT_STRIKETHROUGH:        &'static str = "\x1b[29m";

/// Set forground color to black for the following text.
pub const BLACK:                    &'static str = "\x1b[30m";
/// Set foreground color to red for the following text.
pub const RED:                      &'static str = "\x1b[31m";
/// Set foreground color to green for the following text.
pub const GREEN:                    &'static str = "\x1b[32m";
/// Set foreground color to yellow for the following text.
pub const YELLOW:                   &'static str = "\x1b[33m";
/// Set foreground color to blue for the following text.
pub const BLUE:                     &'static str = "\x1b[34m";
/// Set foreground color to magenta for the following text.
pub const MAGENTA:                  &'static str = "\x1b[35m";
/// Set foreground color to cyan for the following text.
pub const CYAN:                     &'static str = "\x1b[36m";
/// Set foreground color to white for the following text.
pub const WHITE:                    &'static str = "\x1b[37m";

/// Set the foreground color for the following text to the *n*th color in the 256-color palette. Commonly, the set of 256 available colors consists of the 8 named foreground colors, the 8 bright versions of these colors, a 6×6×6 RGB cube (for a total of 216 colors distributed evenly across RGB-space), and then a scale of 24 shades of gray. Different terminals may differ in what colors they provide here.

pub fn color_256(n: u8) -> String {
    format!("\x1b[38;5;{n}m")
}

/// Set the foreground color to the RGB value (r, g, b). Not supported on all terminals. Terminals which do support this feature are called "truecolor terminals".

pub fn truecolor(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[38;2;{r};{g};{b}m")
}

/// Return to the default foreground color for the following text.
pub const DEFAULT:                  &'static str = "\x1b[39m";

/// Set background color to black for the following text.
pub const BLACK_BG:                 &'static str = "\x1b[40m";
/// Set background color to red for the following text.
pub const RED_BG:                   &'static str = "\x1b[41m";
/// Set background color to green for the following text.
pub const GREEN_BG:                 &'static str = "\x1b[42m";
/// Set background color to yellow for the following text.
pub const YELLOW_BG:                &'static str = "\x1b[43m";
/// Set background color to blue for the following text.
pub const BLUE_BG:                  &'static str = "\x1b[44m";
/// Set background color to magenta for the following text.
pub const MAGENTA_BG:               &'static str = "\x1b[45m";
/// Set background color to cyan for the following text.
pub const CYAN_BG:                  &'static str = "\x1b[46m";
/// Set background color to white for the following text.
pub const WHITE_BG:                 &'static str = "\x1b[47m";

/// Set the background color for the following text to the *n*th color in the 256-color palette. Commonly, the set of 256 available colors consists of the 8 named background colors, the 8 bright versions of these colors, a 6×6×6 RGB cube (for a total of 216 colors distributed evenly across RGB-space), and then a scale of 24 shades of gray. Different terminals may differ in what colors they provide here.

pub fn color_256_bg(n: u8) -> String {
    format!("\x1b[48;5;{n}m")
}

/// Set the background color to the RGB value (r, g, b). Not supported on all terminals. Terminals which do support this feature are called "truecolor terminals".

pub fn truecolor_bg(r: u8, g: u8, b: u8) -> String {
    format!("\x1b[48;2;{r};{g};{b}m")
}

/// Return to the default background color for the following text.
pub const DEFAULT_BG:               &'static str = "\x1b[49m";

/// Return to a non-proportionally spaced font for the following text. Rarely meaningful because
/// the `PROPORTIONAL_SPACING` control code is rarely supported to begin with.
pub const NO_PROPORTIONAL_SPACING:  &'static str = "\x1b[50m";
/// Frame the following text.
pub const FRAMED:                   &'static str = "\x1b[51m";
/// Encircle the following text.
pub const ENCIRCLED:                &'static str = "\x1b[52m";
/// Add an overline to the following text.
pub const OVERLINE:                 &'static str = "\x1b[53m";
/// Make the following text neither framed nor encircled.
pub const NEITHER_FRAMED_NOR_ENCIRCLED:
                                    &'static str = "\x1b[54m";
/// Make the following text not overlined.
pub const NOT_OVERLINED:            &'static str = "\x1b[55m";

/// Set the foreground color to bright black for the following text.
pub const BRIGHT_BLACK:             &'static str = "\x1b[90m";
/// Set the foreground color to bright red for the following text.
pub const BRIGHT_RED:               &'static str = "\x1b[91m";
/// Set the foreground color to bright green for the following text.
pub const BRIGHT_GREEN:             &'static str = "\x1b[92m";
/// Set the foreground color to bright yellow for the following text.
pub const BRIGHT_YELLOW:            &'static str = "\x1b[93m";
/// Set the foreground color to bright blue for the following text.
pub const BRIGHT_BLUE:              &'static str = "\x1b[94m";
/// Set the foreground color to bright magenta for the following text.
pub const BRIGHT_MAGENTA:           &'static str = "\x1b[95m";
/// Set the foreground color to bright cyan for the following text.
pub const BRIGHT_CYAN:              &'static str = "\x1b[96m";
/// Set the foreground color to bright white for the following text.
pub const BRIGHT_WHITE:             &'static str = "\x1b[97m";

/// Set the background color to bright black for the following text.
pub const BRIGHT_BLACK_BG:          &'static str = "\x1b[100m";
/// Set the background color to bright red for the following text.
pub const BRIGHT_RED_BG:            &'static str = "\x1b[101m";
/// Set the background color to bright green for the following text.
pub const BRIGHT_GREEN_BG:          &'static str = "\x1b[102m";
/// Set the background color to bright yellow for the following text.
pub const BRIGHT_YELLOW_BG:         &'static str = "\x1b[103m";
/// Set the background color to bright blue for the following text.
pub const BRIGHT_BLUE_BG:           &'static str = "\x1b[104m";
/// Set the background color to bright magenta for the following text.
pub const BRIGHT_MAGENTA_BG:        &'static str = "\x1b[105m";
/// Set the background color to bright cyan for the following text.
pub const BRIGHT_CYAN_BG:           &'static str = "\x1b[106m";
/// Set the background color to bright white for the following text.
pub const BRIGHT_WHITE_BG:          &'static str = "\x1b[107m";

// Test (requires manual inspection of outputs).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_and_verify_visually() {
        println!();
        println!("{GREEN}green{RESET}");
        println!("{BOLD}{RED}BOLD RED{RESET}");
        println!("normal {WHITE}white {BRIGHT_WHITE}bright white{RESET}");
        println!("{BLUE}blue {BRIGHT_BLUE}bright blue{RESET}");
        println!("normal {ITALIC}italic {BOLD}and bold {UNDERLINE}and underline{RESET}");
        println!("normal {DOUBLE_UNDERLINE}double underline{RESET}");
        println!("normal {ENCIRCLED}encircled{RESET}");
        println!("normal {FRAMED}framed{RESET}");
        println!("normal {OVERLINE}overline{RESET}");

        println!(
            "{}g{}r{}e{}y{}s{}c{}a{}l{}e{} {}c{}o{}l{}o{}r{}s{}",
            color_256(232),
            color_256(235),
            color_256(238),
            color_256(241),
            color_256(244),
            color_256(247),
            color_256(250),
            color_256(253),
            color_256(255),
            RESET,
            color_256_bg(255),
            color_256_bg(251),
            color_256_bg(247),
            color_256_bg(243),
            color_256_bg(239),
            color_256_bg(235),
            RESET,
        );

        println!(
            "{}r{}a{}i{}n{}b{}o{}w{}i{}c{} {}c{}o{}l{}o{}r{}s{}",
            color_256(132),
            color_256(135),
            color_256(138),
            color_256(141),
            color_256(144),
            color_256(147),
            color_256(150),
            color_256(153),
            color_256(155),
            RESET,
            color_256_bg(155),
            color_256_bg(151),
            color_256_bg(147),
            color_256_bg(143),
            color_256_bg(139),
            color_256_bg(135),
            RESET,
        );

        println!(
            "{BOLD}{}t{}r{}u{}e{}c{}o{}l{}o{}r{} {BOLD}{}r{}a{}i{}n{}b{}o{}w{}",
            truecolor(255, 0, 0),
            truecolor(170, 0, 0),
            truecolor(90, 0, 0),
            truecolor(30, 0, 0),
            truecolor(0, 30, 0),
            truecolor(0, 90, 0),
            truecolor(0, 170, 0),
            truecolor(0, 200, 0),
            truecolor(0, 255, 0),
            RESET,
            truecolor_bg(0, 255, 0),
            truecolor_bg(0, 150, 0),
            truecolor_bg(0, 50, 0),
            truecolor_bg(0, 0, 50),
            truecolor_bg(0, 0, 150),
            truecolor_bg(0, 0, 170),
            truecolor_bg(0, 0, 255),
            RESET,
        );

        println!("hidden: {HIDDEN}hidden{NOT_HIDDEN} revealed");
        println!("{}green fg {}reset fg", truecolor(0, 255, 0),    DEFAULT);
        println!("{}green bg {}reset bg", truecolor_bg(0, 255, 0), DEFAULT_BG);
        println!();

        println!("{GREEN}ok{RESET}");           // prints a green "ok"
        println!("{BOLD}{RED}error!{RESET}");   // prints a bold, red "error!"
        println!("{BLUE_BG}cloud{RESET}");      // prints
        println!();

        println!("{}{}example text{RESET}", color_256(237), color_256_bg(214));
        println!("{}{}truecolor text{RESET}", truecolor(127, 45, 68), truecolor_bg(0, 255, 255));
        println!();

        println!("{SLOW_BLINK}slow blink{RESET}");
        println!("{RAPID_BLINK}rapid blink{RESET}");
        println!("{ALT_FONT_1}alt font 1{RESET}");
        println!("{ALT_FONT_2}alt font 2{RESET}");
        println!("{ALT_FONT_3}alt font 3{RESET}");
        println!("{FRAKTUR}Fraktur font{RESET}");
        println!("{DEFAULT_FONT}default font{RESET}");
        println!("{INVERTED}{GREEN}green fg but it's bg{RESET}");
        println!();
    }
}


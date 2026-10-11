//! Drawing a project's emblem on a terminal.
//!
//! Two renderings, because terminals differ in what they can draw:
//!
//! - The word grid as block characters in the project color, which every
//!   terminal can show. The grid is the same data the emblem assets are
//!   generated from, so this is the wordmark exactly, not an approximation of
//!   a picture of it. Behind the `terminal` feature.
//! - The emblem itself — icon and all — as a real image, on the terminals that
//!   implement a graphics protocol. Behind the `terminal-graphics` feature.
//!
//! [`banner`] picks between them and is what a program should call.

use crate::emblem::Emblem;
use std::io::IsTerminal;

#[cfg(feature = "terminal-graphics")]
mod graphics;

#[cfg(feature = "terminal-graphics")]
pub use graphics::{image_banner, rasterize};

/// Whether anything should be drawn on stdout at all.
///
/// False when stdout is not a terminal — a pipe or a file wants the program's
/// text and not a picture of its name — and when the user has asked for no
/// color, since an emblem is decoration of exactly the kind `NO_COLOR` is
/// about.
pub fn supported() -> bool {
    if !std::io::stdout().is_terminal() {
        return false;
    }
    // https://no-color.org and the CLICOLOR convention
    if std::env::var_os("NO_COLOR").is_some_and(|value| !value.is_empty()) {
        return false;
    }
    if std::env::var("CLICOLOR").is_ok_and(|value| value == "0") {
        return false;
    }
    !matches!(std::env::var("TERM").as_deref(), Ok("dumb") | Ok(""))
}

/// The emblem's word as block characters in the project color, wrapped to end
/// with a newline, or `None` if it will not fit in `cols` columns.
///
/// This renders unconditionally; see [`supported`] for whether it should be
/// called, or [`banner`] to have that decided.
pub fn text_banner(emblem: &Emblem, cols: u16) -> Option<String> {
    // Square pixels where there is room for them, half-width where the word is
    // too long for the terminal to show it any other way
    let lines = ["██", "█"].into_iter().find_map(|fill| {
        let lines = emblem.word_lines(fill);
        let widest = lines.iter().map(|line| line.chars().count()).max()?;
        (widest <= cols as usize).then_some(lines)
    })?;

    let (r, g, b) = emblem.rgb();
    let mut out = String::new();
    for line in lines {
        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m{line}\x1b[39m\n"));
    }
    Some(out)
}

/// The emblem as terminal output: a real image where the terminal has said it
/// can draw one, the word in block characters otherwise, and `None` where
/// nothing should be drawn.
///
/// The result ends with a newline and no blank line, so a caller that wants
/// the emblem set apart from what follows adds that itself.
///
/// `max_cols` and `max_rows` bound the image; the block characters are bounded
/// by the terminal's width alone, being a single line of text per row.
pub fn banner(emblem: &Emblem, max_cols: u16, max_rows: u16) -> Option<String> {
    if !supported() {
        return None;
    }
    let cols = terminal_size().map_or(80, |(cols, _)| cols);

    #[cfg(feature = "terminal-graphics")]
    if let Some(image) = image_banner(emblem, max_cols.min(cols), max_rows) {
        return Some(image);
    }
    #[cfg(not(feature = "terminal-graphics"))]
    let (_, _) = (max_cols, max_rows);

    text_banner(emblem, cols)
}

/// The emblem for clap's `before_help`.
///
/// The same thing [`banner`] returns, less its final newline, because clap
/// puts a blank line after `before_help` itself and two would leave the
/// emblem floating.
pub fn before_help(emblem: &Emblem, max_cols: u16, max_rows: u16) -> Option<String> {
    let banner = banner(emblem, max_cols, max_rows)?;
    Some(match banner.strip_suffix('\n') {
        Some(trimmed) => trimmed.to_string(),
        None => banner,
    })
}

/// The terminal's size in cells, from an ioctl rather than a query, so that
/// nothing is written to the terminal and nothing is read back from it.
fn terminal_size() -> Option<(u16, u16)> {
    let (cols, rows) = crossterm::terminal::size().ok()?;
    (cols > 0 && rows > 0).then_some((cols, rows))
}

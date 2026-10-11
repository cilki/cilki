//! The emblem as a real image, on terminals that implement a graphics
//! protocol.
//!
//! Which protocol a terminal speaks is read out of the environment rather than
//! asked of the terminal. The usual way to find out is to write a capability
//! query and read the answer back, but a terminal that does not recognize the
//! query simply never answers, so the cost of asking is a timeout on every
//! program that asks — and an answer that arrives after the program has
//! exited is left in the shell's input buffer as a line of escape bytes the
//! shell then tries to run. Nothing here writes to the terminal or reads from
//! it until the whole image is built, and nothing reads from stdin at all.
//!
//! The trade is that a terminal which supports a protocol without saying so in
//! the environment gets block characters instead, and that `TERM=xterm-kitty`
//! exported across an `ssh` into a host whose terminal is not kitty will draw
//! escape bytes as garbage. The first is invisible; the second is loud, but it
//! requires the user to have lied about their terminal.

use crate::emblem::Emblem;
use ratatui::buffer::{Buffer, CellDiffOption};
use ratatui::layout::{Rect, Size};
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui_image::picker::{Picker, ProtocolType};
use ratatui_image::{FilterType, FontSize, Image, Resize};
use std::fmt::Write;

/// Rasterize the emblem's SVG at `scale`.
///
/// `background` of `None` keeps the SVG's own transparency, which is what the
/// emblem needs when it sits directly on the user's terminal background; an
/// opaque RGB is for where it sits on a background the program drew itself,
/// since the alpha the SVG's antialiased edges carry has to be composited onto
/// something.
pub fn rasterize(
    emblem: &Emblem,
    scale: f32,
    background: Option<(u8, u8, u8)>,
) -> Option<image::DynamicImage> {
    let svg = emblem.to_svg_string(None);
    let tree = resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default()).ok()?;

    let width = (tree.size().width() * scale).ceil() as u32;
    let height = (tree.size().height() * scale).ceil() as u32;
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)?;
    if let Some((r, g, b)) = background {
        pixmap.fill(resvg::tiny_skia::Color::from_rgba8(r, g, b, 255));
    }
    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    // tiny-skia pixels are premultiplied RGBA
    let pixels = pixmap
        .pixels()
        .iter()
        .flat_map(|pixel| {
            let pixel = pixel.demultiply();
            [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()]
        })
        .collect();
    Some(image::DynamicImage::ImageRgba8(image::RgbaImage::from_raw(
        width, height, pixels,
    )?))
}

/// The emblem as the bytes that draw it, fitted into `max_cols` by `max_rows`
/// cells, or `None` wherever anything at all is unknown, unsupported, or
/// fails.
///
/// The result ends on a fresh line, having moved the cursor only relative to
/// where it already was.
pub fn image_banner(emblem: &Emblem, max_cols: u16, max_rows: u16) -> Option<String> {
    if max_cols == 0 || max_rows == 0 {
        return None;
    }
    let picker = picker()?;

    // Rasterized well above the size it will be drawn at, so that fitting it
    // to the cell grid is a downscale
    let image = rasterize(emblem, 3.0, None)?;
    let protocol = picker
        .new_protocol(
            image,
            Size::new(max_cols, max_rows),
            // The default filter is nearest, which drops and doubles rows of
            // the block letters unevenly on the way down
            Resize::Fit(Some(FilterType::Lanczos3)),
        )
        .ok()?;

    // The `Image` widget renders nothing at all, and says nothing about it,
    // when the protocol is larger than the area it is given, so the area comes
    // from the protocol rather than from what was asked for.
    let size = protocol.size();
    if size.width == 0 || size.height == 0 || size.width > max_cols || size.height > max_rows {
        return None;
    }

    let area = Rect::new(0, 0, size.width, size.height);
    let mut buffer = Buffer::empty(area);
    Image::new(&protocol).render(area, &mut buffer);

    emit(&buffer, size)
}

/// The protocol this terminal has said it speaks, if any.
fn picker() -> Option<Picker> {
    // Ruled out before a `Picker` exists, because every one of its
    // constructors runs ratatui-image's own tmux detection, which on finding
    // tmux spawns `tmux set -p allow-passthrough on` — a child process and a
    // change to the user's pane options, neither of which printing a banner
    // gets to do.
    if in_multiplexer() {
        return None;
    }

    let kitty = kitty_from_env();
    let font_size = measured_font_size().unwrap_or(FontSize::new(10, 20));
    // The only public constructor that takes a real cell size; the alternative
    // hardcodes one.
    #[allow(deprecated)]
    let mut picker = Picker::from_fontsize(font_size);
    // `from_fontsize` detects the iTerm2 family from the environment itself,
    // but declines to guess kitty, whose marker variables go stale inside a
    // multiplexer — the case just ruled out, which is what makes the guess
    // safe to make here.
    if let Some(kitty) = kitty {
        picker.set_protocol_type(kitty);
    }

    matches!(
        picker.protocol_type(),
        ProtocolType::Kitty | ProtocolType::Iterm2
    )
    .then_some(picker)
}

/// Whether a terminal multiplexer is in the way.
///
/// A multiplexer redraws everything it is handed, so a graphics escape sent
/// through one is dropped or mangled unless it is wrapped for a passthrough
/// that only a query could confirm is enabled.
fn in_multiplexer() -> bool {
    let term = std::env::var("TERM").unwrap_or_default();
    if term.starts_with("tmux") || term.starts_with("screen") {
        return true;
    }
    if std::env::var("TERM_PROGRAM").is_ok_and(|program| program == "tmux") {
        return true;
    }
    ["TMUX", "STY", "ZELLIJ"]
        .iter()
        .any(|name| std::env::var_os(name).is_some_and(|value| !value.is_empty()))
}

/// Whether this is a terminal that draws kitty's protocol.
fn kitty_from_env() -> Option<ProtocolType> {
    let nonempty = |name: &str| std::env::var_os(name).is_some_and(|value| !value.is_empty());

    let term = std::env::var("TERM").unwrap_or_default();
    (term == "xterm-kitty"
        || term == "xterm-ghostty"
        || nonempty("KITTY_WINDOW_ID")
        || nonempty("GHOSTTY_RESOURCES_DIR")
        || std::env::var("TERM_PROGRAM").is_ok_and(|program| program == "ghostty"))
    .then_some(ProtocolType::Kitty)
}

/// The terminal's cell size in pixels, from the window-size ioctl.
///
/// Only a preference: a terminal that reports no pixel geometry — kitty over
/// a plain `ssh`, for one — still draws the image perfectly well at a guessed
/// cell size, just at a slightly wrong aspect, which beats no image.
fn measured_font_size() -> Option<FontSize> {
    let size = crossterm::terminal::window_size().ok()?;
    if size.columns == 0 || size.rows == 0 || size.width == 0 || size.height == 0 {
        return None;
    }
    let (width, height) = (size.width / size.columns, size.height / size.rows);
    // A cell outside these bounds means the report is nonsense, not a font
    ((4..=40).contains(&width) && (8..=80).contains(&height)).then(|| FontSize::new(width, height))
}

/// Serialize what the image protocol drew into `buffer` as the bytes that put
/// it on the screen, using no absolute cursor motion.
///
/// Every protocol draws by writing into a ratatui `Buffer` and nothing else:
/// kitty fills each cell with a placeholder character whose foreground carries
/// the image id, and the first cell with the one-time transmit sequence, while
/// iTerm2 puts one self-contained escape in the first cell and marks the rest
/// skipped. So a row's unskipped symbols in order, then a newline, is what a
/// ratatui terminal would have written — without needing to know which row of
/// the screen it is on, which is the only thing a cursor-position query would
/// have bought.
fn emit(buffer: &Buffer, size: Size) -> Option<String> {
    let mut out = String::new();

    // Reserve the rows, then come back up. A newline per row is what ratatui's
    // own inline viewport does: it scrolls the screen when the cursor is near
    // the bottom, so the rows the image lands on are certain to exist and it
    // can never spill past the last one.
    for _ in 0..size.height {
        out.push('\n');
    }
    // Guarded against zero by the caller; `ESC[0A` means one row on some
    // terminals rather than none.
    write!(out, "\x1b[{}A", size.height).ok()?;

    for y in 0..size.height {
        // Each row starts from column 0 of its own accord, so a disagreement
        // about how wide the placeholder character is cannot accumulate across
        // rows.
        out.push('\r');
        let mut foreground = None;
        for x in 0..size.width {
            let cell = buffer.cell((x, y))?;
            if cell.diff_option == CellDiffOption::Skip {
                continue;
            }
            if foreground != Some(cell.fg) {
                match cell.fg {
                    Color::Rgb(r, g, b) => write!(out, "\x1b[38;2;{r};{g};{b}m").ok()?,
                    Color::Reset => {}
                    // Only kitty sets a foreground here, and only ever an
                    // image id as an RGB triple. Anything else means this is
                    // not the buffer this was written for.
                    _ => return None,
                }
                foreground = Some(cell.fg);
            }
            out.push_str(cell.symbol());
        }
        if matches!(foreground, Some(Color::Rgb(..))) {
            out.push_str("\x1b[39m");
        }
        out.push_str("\r\n");
    }

    Some(out)
}

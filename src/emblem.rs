/// Each project has a unique emblem which is an icon plus block text.
pub struct Emblem {
    pub name: &'static str,
    pub word: [&'static str; 7],
    pub margin_px: usize,
    pub rect_side_px: usize,
    pub rect_gap_px: usize,
    pub color: &'static str,
    pub icon: &'static str,
    pub icon_width: Option<usize>,
    pub year: usize,
}

#[cfg(feature = "project-sandpolis")]
pub const SANDPOLIS: Emblem = Emblem {
    name: "sandpolis",
    word: [
        "              ◼         ◼      ",
        "              ◼         ◼      ",
        " ◼◼  ◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼  ◼◼",
        " ◼  ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼  ◼ ",
        "◼◼   ◼◼ ◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ◼ ◼◼ ",
        "                ◼              ",
        "                ◼              ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#c89437",
    icon: include_str!("../icons/sandpolis.svg"),
    icon_width: Some(50),
    year: 2018,
};

#[cfg(feature = "project-goldboot")]
pub const GOLDBOOT: Emblem = Emblem {
    name: "goldboot",
    word: [
        "        ◼   ◼ ◼           ◼ ",
        "        ◼   ◼ ◼           ◼◼",
        "◼◼◼ ◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼ ",
        "◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ",
        "◼◼◼ ◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼◼ ◼◼",
        "  ◼                         ",
        "◼◼◼                         ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#c8ab37",
    icon: include_str!("../icons/goldboot.svg"),
    icon_width: Some(50),
    year: 2022,
};

#[cfg(feature = "project-workset")]
pub const WORKSET: Emblem = Emblem {
    name: "workset",
    word: [
        "                         ◼ ",
        "                         ◼◼",
        "◼   ◼ ◼◼◼ ◼◼ ◼ ◼  ◼◼  ◼◼ ◼ ",
        "◼ ◼ ◼ ◼ ◼ ◼  ◼◼   ◼  ◼◼  ◼ ",
        "◼◼ ◼◼ ◼◼◼ ◼  ◼ ◼ ◼◼   ◼◼ ◼◼",
        "                           ",
        "                           ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#3776c8",
    icon: include_str!("../icons/workset.svg"),
    icon_width: Some(50),
    year: 2022,
};

#[cfg(feature = "project-turbine")]
pub const TURBINE: Emblem = Emblem {
    name: "turbine",
    word: [
        "◼         ◼            ",
        "◼◼        ◼            ",
        "◼  ◼ ◼ ◼◼ ◼◼◼ ◼ ◼◼◼  ◼◼",
        "◼  ◼ ◼ ◼  ◼ ◼ ◼ ◼ ◼ ◼◼ ",
        "◼◼ ◼◼◼ ◼  ◼◼◼ ◼ ◼ ◼  ◼◼",
        "                       ",
        "                       ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#378B2E",
    icon: include_str!("../icons/turbine.svg"),
    icon_width: Some(50),
    year: 2023,
};

#[cfg(feature = "project-outpost")]
pub const OUTPOST: Emblem = Emblem {
    name: "outpost",
    word: [
        "        ◼              ◼ ",
        "        ◼◼             ◼◼",
        "◼◼◼ ◼ ◼ ◼  ◼◼◼ ◼◼◼  ◼◼ ◼ ",
        "◼ ◼ ◼ ◼ ◼  ◼ ◼ ◼ ◼  ◼  ◼ ",
        "◼◼◼ ◼◼◼ ◼◼ ◼◼◼ ◼◼◼ ◼◼  ◼◼",
        "           ◼             ",
        "           ◼             ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#c85037",
    icon: include_str!("../icons/outpost.svg"),
    icon_width: Some(50),
    year: 2024,
};

#[cfg(feature = "project-cibox")]
pub const CIBOX: Emblem = Emblem {
    name: "cibox",
    word: [
        "      ◼          ",
        "      ◼          ",
        "◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼ ◼",
        "◼   ◼ ◼ ◼ ◼ ◼  ◼ ",
        "◼◼◼ ◼ ◼◼◼ ◼◼◼ ◼ ◼",
        "                 ",
        "                 ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#37c88b",
    icon: include_str!("../icons/cibox.svg"),
    icon_width: Some(50),
    year: 2025,
};

#[cfg(feature = "project-fossdb")]
pub const FOSSDB: Emblem = Emblem {
    name: "fossdb",
    word: [
        " ◼◼              ◼ ◼  ",
        " ◼               ◼ ◼  ",
        "◼◼◼ ◼◼◼  ◼◼ ◼◼ ◼◼◼ ◼◼◼",
        " ◼  ◼ ◼  ◼  ◼  ◼ ◼ ◼ ◼",
        " ◼  ◼◼◼ ◼◼ ◼◼  ◼◼◼ ◼◼◼",
        "                      ",
        "                      ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#37c88b",
    icon: include_str!("../icons/fossdb.svg"),
    icon_width: Some(50),
    year: 2025,
};

#[cfg(feature = "project-solder")]
pub const SOLDER: Emblem = Emblem {
    name: "solder",
    word: [
        "        ◼   ◼       ",
        "        ◼   ◼       ",
        " ◼◼ ◼◼◼ ◼ ◼◼◼  ◼◼ ◼◼",
        " ◼  ◼ ◼ ◼ ◼ ◼ ◼◼  ◼ ",
        "◼◼  ◼◼◼ ◼ ◼◼◼  ◼◼ ◼ ",
        "                    ",
        "                    ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#37c88b",
    icon: include_str!("../icons/solder.svg"),
    icon_width: Some(50),
    year: 2026,
};

#[cfg(feature = "project-attest")]
pub const ATTEST: Emblem = Emblem {
    name: "attest",
    word: [
        "                    ",
        "    ◼  ◼          ◼ ",
        "    ◼◼ ◼◼         ◼◼",
        " ◼◼ ◼  ◼   ◼◼  ◼◼ ◼ ",
        "◼ ◼ ◼  ◼  ◼◼   ◼  ◼ ",
        " ◼◼ ◼◼ ◼◼  ◼◼ ◼◼  ◼◼",
        "                    ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#248467",
    icon: include_str!("../icons/attest.svg"),
    icon_width: Some(50),
    year: 2026,
};

#[cfg(feature = "project-codemine")]
pub const CODEMINE: Emblem = Emblem {
    name: "codemine",
    word: [
        "          ◼                    ",
        "          ◼                    ",
        "◼◼◼ ◼◼◼ ◼◼◼  ◼◼ ◼◼◼◼◼ ◼ ◼◼◼  ◼◼",
        "◼   ◼ ◼ ◼ ◼ ◼◼  ◼ ◼ ◼ ◼ ◼ ◼ ◼◼ ",
        "◼◼◼ ◼◼◼ ◼◼◼  ◼◼ ◼ ◼ ◼ ◼ ◼ ◼  ◼◼",
        "                               ",
        "                               ",
    ],
    margin_px: 7,
    rect_side_px: 7,
    rect_gap_px: 1,
    color: "#a0a0a0",
    icon: include_str!("../icons/codemine.svg"),
    icon_width: Some(50),
    year: 2026,
};

impl Emblem {
    /// The project color as RGB components, parsed from its `#rrggbb` string.
    pub fn rgb(&self) -> (u8, u8, u8) {
        let hex = self.color.strip_prefix('#').unwrap_or(self.color);
        if hex.len() < 6 {
            return (0, 0, 0);
        }
        let byte = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).unwrap_or(0);
        (byte(0), byte(2), byte(4))
    }

    /// The word grid as lines of text, with each filled cell drawn as `fill`
    /// and each empty one as the same width in spaces.
    ///
    /// Rows that are entirely empty are dropped from the top and bottom, since
    /// the grid is a fixed seven rows that only the tallest words fill. The
    /// rows between are kept whatever they hold, so the letters keep their
    /// baseline.
    ///
    /// A terminal cell is about twice as tall as it is wide, so a two-column
    /// `fill` is what draws the word in square pixels; a one-column fill
    /// renders it at half width, which is the only way a long word fits a
    /// narrow terminal.
    pub fn word_lines(&self, fill: &str) -> Vec<String> {
        let blank = |row: &&str| row.chars().all(|c| c == ' ');
        let Some(first) = self.word.iter().position(|row| !blank(row)) else {
            return Vec::new();
        };
        // `rposition` finds at least the row `position` just found
        let last = self.word.iter().rposition(|row| !blank(row)).unwrap();

        let gap = " ".repeat(fill.chars().count());
        self.word[first..=last]
            .iter()
            .map(|row| {
                let mut line = String::new();
                for cell in row.chars() {
                    line.push_str(if cell == ' ' { &gap } else { fill });
                }
                // The grid pads every row out to a common width; that padding
                // is invisible except for the trailing whitespace it leaves in
                // whatever the caller prints this into.
                line.truncate(line.trim_end().len());
                line
            })
            .collect()
    }

    /// The icon alone as an SVG document, recolored from the black it ships
    /// as to the project color, and with the XML declaration dropped so that
    /// it can be nested or served as-is.
    ///
    /// `None` for an emblem that carries no icon.
    pub fn icon_svg(&self) -> Option<String> {
        self.icon_width?;
        Some(self.icon[self.icon.find("<svg").unwrap_or(0)..].replace("#000000", self.color))
    }

    /// The emblem as a standalone SVG document: the icon, recolored from the
    /// black it ships as, beside the word in block letters, over an optional
    /// background plate (an SVG `style` attribute, e.g. `fill:#333333`).
    ///
    /// This mirrors the geometry of the `EmblemRenderer` in this crate's
    /// binary, which generates the `emblems/` assets, but builds the document
    /// as text so that it needs no dependencies and can run at any time rather
    /// than only at asset-build time. The two draw the same picture; they do
    /// not produce the same bytes.
    pub fn to_svg_string(&self, bg_style: Option<&str>) -> String {
        let side = self.rect_side_px;
        let step = side + self.rect_gap_px;
        let margin = self.margin_px;
        let icon_width = self.icon_width.unwrap_or(0);
        let columns = self.word.first().map_or(0, |row| row.chars().count());

        // The block letters, column by column. An empty column pulls
        // everything after it a little closer, which is what keeps the letters
        // from reading as evenly spaced cells.
        let total_adjust: usize = (0..columns)
            .filter(|c| {
                self.word
                    .iter()
                    .all(|row| row.chars().nth(*c).unwrap_or(' ') == ' ')
            })
            .count()
            * 3;

        let mut rects = String::new();
        let mut adjust = 0;
        for c in 0..columns {
            let mut empty = true;
            for (r, row) in self.word.iter().enumerate() {
                if row.chars().nth(c).unwrap_or(' ') != ' ' {
                    empty = false;
                    let x = margin + icon_width + c * step - adjust;
                    let y = margin + r * step;
                    rects.push_str(&format!(
                        r#"<rect style="fill:{}" id="{r}-{c}" width="{side}" height="{side}" x="{x}" y="{y}" rx="1"/>"#,
                        self.color
                    ));
                }
            }
            if empty {
                adjust += 3;
            }
        }

        let width =
            margin * 2 + icon_width + columns.saturating_sub(1) * step + side - total_adjust;
        let height = margin * 2 + (self.word.len() - 1) * step + side;

        let background = bg_style.map_or(String::new(), |style| {
            format!(
                r#"<g><rect style="{style}" id="background" width="100%" height="100%" rx="3%"/></g>"#
            )
        });

        let icon = self.icon_svg().map_or(String::new(), |icon| {
            format!(
                r#"<g transform="translate({},{})">{icon}</g>"#,
                margin / 2,
                margin / 2
            )
        });

        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none">{background}{icon}<g>{rects}</g></svg>"#
        )
    }
}

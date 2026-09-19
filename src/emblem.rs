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
    icon: include_str!("../icons/fossdb.svg"),
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
    color: "#8b37c8",
    icon: include_str!("../icons/codemine.svg"),
    icon_width: Some(50),
    year: 2026,
};

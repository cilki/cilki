use crate::svg::*;
use cilki::emblem::Emblem;
use std::error::Error;

pub struct EmblemRenderer<'a> {
    pub emblem: &'a Emblem,
    pub bg_style: Option<&'static str>,
}

impl<'a> EmblemRenderer<'a> {
    pub fn new(emblem: &'a Emblem, bg_style: Option<&'static str>) -> Self {
        Self { emblem, bg_style }
    }

    pub fn to_svg(&self) -> Result<Svg, Box<dyn Error>> {
        let mut svg = Svg::default();
        svg.fill = Some("none".into());

        // Add SVG namespaces
        svg.xmlns = Some("http://www.w3.org/2000/svg".to_string());
        svg.xmlns_xlink = Some("http://www.w3.org/1999/xlink".to_string());

        // Add background
        if let Some(bg_style) = self.bg_style {
            svg.g.push(SvgGroup {
                rect: vec![SvgRect {
                    style: String::from(bg_style),
                    id: format!("background"),
                    width: format!("100%"),
                    height: format!("100%"),
                    x: format!(""),
                    y: format!(""),
                    rx: String::from("3%"),
                }],
                ..Default::default()
            });
        }

        // Add icon
        if self.emblem.icon_width.is_some() {
            // Replace #000000 with the project color in the icon SVG
            let icon_svg = self.emblem.icon.replace("#000000", self.emblem.color);
            let icon: Svg = quick_xml::de::from_str(icon_svg.as_str())?;

            let icon_group = icon.g.first().unwrap();

            // Position the icon, keeping its own nesting and transforms intact
            svg.g.push(SvgGroup {
                g: vec![icon_group.clone()],
                transform: Some(format!(
                    "translate({},{})",
                    self.emblem.margin_px / 2,
                    self.emblem.margin_px / 2
                )),
                ..Default::default()
            });
        }

        // Add letters
        svg.g.push(SvgGroup {
            rect: self.generate_rects(),
            ..Default::default()
        });

        // Set image dimensions
        svg.width = format!(
            "{}",
            (self.emblem.margin_px * 2)
                + self.emblem.icon_width.unwrap_or(0)
                + ((self.emblem.word.first().unwrap().chars().count() - 1)
                    * (self.emblem.rect_side_px + self.emblem.rect_gap_px))
                + self.emblem.rect_side_px
                - self.total_horizontal_adjust()
        );
        svg.height = format!(
            "{}",
            (self.emblem.margin_px * 2)
                + ((self.emblem.word.len() - 1)
                    * (self.emblem.rect_side_px + self.emblem.rect_gap_px))
                + self.emblem.rect_side_px
        );

        return Ok(svg);
    }

    fn generate_rects(&self) -> Vec<SvgRect> {
        let mut rects = Vec::new();

        // Makes the horizontal spacing a little bit nicer
        let mut horizontal_adjust = 0;

        for c in 0..self.emblem.word.first().unwrap().chars().count() {
            // Track empty columns
            let mut empty = true;
            for r in 0..self.emblem.word.len() {
                if self.emblem.word[r].chars().nth(c).unwrap() != ' ' {
                    empty = false;
                    rects.push(SvgRect {
                        style: format!("fill:{}", self.emblem.color),
                        id: format!("{r}-{c}"),
                        width: format!("{}", self.emblem.rect_side_px),
                        height: format!("{}", self.emblem.rect_side_px),
                        x: format!(
                            "{}",
                            // Leave some space for the margin
                            self.emblem.margin_px
                        // Leave some space for the icon if present
                        + self.emblem.icon_width.unwrap_or(0)
                        + (c * self.emblem.rect_side_px + c * self.emblem.rect_gap_px)
                        // Remove some space between each letter
                        - horizontal_adjust
                        ),
                        y: format!(
                            "{}",
                            self.emblem.margin_px
                                + (r * self.emblem.rect_side_px + r * self.emblem.rect_gap_px)
                        ),
                        rx: String::from("1"),
                    });
                }
            }

            if empty {
                horizontal_adjust += 3;
            }
        }

        return rects;
    }

    fn total_horizontal_adjust(&self) -> usize {
        // Makes the horizontal spacing a little bit nicer
        let mut horizontal_adjust = 0;

        for c in 0..self.emblem.word.first().unwrap().chars().count() {
            // Track empty columns
            let mut empty = true;
            for r in 0..self.emblem.word.len() {
                if self.emblem.word[r].chars().nth(c).unwrap() != ' ' {
                    empty = false;
                }
            }

            if empty {
                horizontal_adjust += 3;
            }
        }
        return horizontal_adjust;
    }
}

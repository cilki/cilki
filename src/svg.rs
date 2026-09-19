///! Implements a small subset of the SVG spec for our needs.
use quick_xml::se::Serializer;
use resvg::tiny_skia::{Pixmap, Transform};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};

#[derive(Serialize, Deserialize, Default)]
#[serde(rename = "svg")]
pub struct Svg {
    pub g: Vec<SvgGroup>,
    #[serde(rename = "@width")]
    pub width: String,
    #[serde(rename = "@height")]
    pub height: String,
    #[serde(rename = "@fill")]
    pub fill: Option<String>,
    #[serde(rename = "@xmlns", skip_serializing_if = "Option::is_none")]
    pub xmlns: Option<String>,
    #[serde(rename = "@xmlns:xlink", skip_serializing_if = "Option::is_none")]
    pub xmlns_xlink: Option<String>,
}

impl Svg {
    pub fn write_to(&self, path: &str) -> Result<bool, Box<dyn Error>> {
        let mut buffer = String::new();
        let mut ser = Serializer::new(&mut buffer);
        ser.indent(' ', 2);

        self.serialize(ser)?;

        let needs_write = if Path::new(path).exists() {
            std::fs::read_to_string(path)? != buffer
        } else {
            true
        };

        if needs_write {
            std::fs::write(path, &buffer)?;
        }

        Ok(needs_write)
    }

    pub fn to_string(&self) -> String {
        quick_xml::se::to_string(&self).unwrap()
    }

    /// Convert into PNG and write to the given path.
    /// Returns true if the file was written, false if it was already up to date.
    pub fn rasterize<P>(&self, path: P, width: u32, height: u32) -> Result<bool, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let svg = usvg::Tree::from_str(&self.to_string(), &usvg::Options::default())?;
        let mut pixmap = Pixmap::new(width, height).expect("invalid size");

        // Calculate scale to fit the SVG into the target dimensions
        let svg_size = svg.size();
        let scale = (width as f32 / svg_size.width()).min(height as f32 / svg_size.height());

        let transform = Transform::from_scale(scale, scale);
        resvg::render(&svg, transform, &mut pixmap.as_mut());

        let png_data = pixmap.encode_png()?;

        let needs_write = if path.as_ref().exists() {
            std::fs::read(path.as_ref())? != png_data
        } else {
            true
        };

        if needs_write {
            std::fs::write(path.as_ref(), &png_data)?;
        }

        Ok(needs_write)
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "g")]
pub struct SvgGroup {
    #[serde(default)]
    pub g: Vec<SvgGroup>,
    #[serde(default)]
    pub path: Vec<SvgPath>,
    #[serde(default)]
    pub rect: Vec<SvgRect>,
    #[serde(default)]
    pub ellipse: Vec<SvgEllipse>,
    #[serde(default)]
    pub circle: Vec<SvgCircle>,
    #[serde(rename = "@transform", skip_serializing_if = "Option::is_none")]
    pub transform: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "path")]
pub struct SvgPath {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "@d")]
    pub d: String,
    #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
    pub stroke_linecap: Option<String>,
    #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
    pub stroke_linejoin: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "rect")]
pub struct SvgRect {
    #[serde(rename = "@style")]
    pub style: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@width")]
    pub width: String,
    #[serde(rename = "@height")]
    pub height: String,
    #[serde(rename = "@x")]
    pub x: String,
    #[serde(rename = "@y")]
    pub y: String,
    #[serde(rename = "@rx")]
    pub rx: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "ellipse")]
pub struct SvgEllipse {
    #[serde(rename = "@cx")]
    pub cx: String,
    #[serde(rename = "@cy")]
    pub cy: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@rx")]
    pub rx: String,
    #[serde(rename = "@ry")]
    pub ry: String,
    #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
    pub stroke_linecap: Option<String>,
    #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
    pub stroke_linejoin: Option<String>,
    #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(rename = "@style")]
    pub style: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename = "circle")]
pub struct SvgCircle {
    #[serde(rename = "@cx")]
    pub cx: String,
    #[serde(rename = "@cy")]
    pub cy: String,
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@r")]
    pub rx: String,
    #[serde(rename = "@stroke", skip_serializing_if = "Option::is_none")]
    pub stroke: Option<String>,
    #[serde(rename = "@stroke-linecap", skip_serializing_if = "Option::is_none")]
    pub stroke_linecap: Option<String>,
    #[serde(rename = "@stroke-linejoin", skip_serializing_if = "Option::is_none")]
    pub stroke_linejoin: Option<String>,
    #[serde(rename = "@stroke-width", skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<String>,
    #[serde(rename = "@style")]
    pub style: Option<String>,
}

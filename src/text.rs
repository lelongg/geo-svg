use std::fmt::Display;

use geo_types::Coord;

use crate::{Style, ToSvgStr, ViewBox};

/// simple Text element for SVGs. This comes in handy if you want to enumerate some sort of
/// geometry for any purposes
pub struct Text<S>
where
    S: Display,
{
    /// anything that is display-able as a string
    text: S,
    /// the rough coordinates of the text
    position: Coord,
    /// the size of the font of the text
    font_size: f32,
}

impl<S> Text<S>
where
    S: Display,
{
    /// create new Text object and it's position
    pub fn new(text: S, position: Coord) -> Self {
        Self {
            text,
            position,
            font_size: 10.0,
        }
    }

    /// overwrite the existing font size
    pub fn with_font_size(self, font_size: f32) -> Self {
        Self { font_size, ..self }
    }
}

impl<S> ToSvgStr for Text<S>
where
    S: Display,
{
    fn to_svg_str(&self, _style: &Style) -> String {
        let Text {
            text,
            position: Coord { x, y },
            font_size,
        } = self;
        format!(r#"<text font-size="{font_size}" x="{x} y="{y}">{text}</text>"#)
    }

    // we can probably do better here by calculating a viewbox based on font and font size
    // something along the lines of
    //
    // https://stackoverflow.com/questions/71283347/difference-in-length-calculation-for-svg-text-element
    fn viewbox(&self, _style: &Style) -> ViewBox {
        ViewBox {
            min_x: None,
            min_y: None,
            max_x: None,
            max_y: None,
        }
    }
}

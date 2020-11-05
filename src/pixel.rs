use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use custom_error::custom_error;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Pixel {
    x: u32,
    y: u32,
    color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Pixel {
        Pixel { x, y, color }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PX {} {} {}", self.x, self.y, self.color)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        // First part is 'PX'
        if parts.next().is_none() {
            return Err(ParsePixelError::WrongFormat);
        }

        let pixel = Pixel::new(
            parts.next().ok_or(ParsePixelError::WrongFormat)?.parse()?,
            parts.next().ok_or(ParsePixelError::WrongFormat)?.parse()?,
            parts.next().ok_or(ParsePixelError::WrongFormat)?.parse()?,
        );

        if parts.next().is_some() {
            Err(ParsePixelError::WrongFormat)
        } else {
            Ok(pixel)
        }
    }
}

custom_error! {#[derive(PartialEq)] pub ParsePixelError
    ParseInt{source: ParseIntError} = "no valid integer found",
    ParseColor{source: ParseColorError} = "failed to parse color",
    WrongFormat            = "the string has the wrong format"
}

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: None }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a: Some(a),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.a {
            Some(a) => write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a),
            None => write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b),
        }
    }
}

impl FromStr for Color {
    type Err = ParseColorError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.len() {
            6 => Ok(Color::rgb(
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
            )),
            8 => Ok(Color::rgba(
                u8::from_str_radix(&s[0..2], 16)?,
                u8::from_str_radix(&s[2..4], 16)?,
                u8::from_str_radix(&s[4..6], 16)?,
                u8::from_str_radix(&s[6..8], 16)?,
            )),
            _ => Err(ParseColorError::WrongSize),
        }
    }
}

custom_error! {#[derive(PartialEq)] pub ParseColorError
    ParseInt{source: ParseIntError} = "no valid integer found",
    WrongSize            = "the string has the wrong number of digits"
}

#[cfg(test)]
mod tests {
    use crate::pixel::{Color, ParseColorError, ParsePixelError, Pixel};

    #[test]
    fn display_pixel() {
        let px = Pixel::new(1024, 768, Color::rgb(0x00, 0xff, 0x00));
        assert_eq!(px.to_string(), "PX 1024 768 00ff00")
    }

    #[test]
    fn fromstr_pixel() {
        let pixel: Pixel = "PX 1024 768 ff0f00".parse().unwrap();
        assert_eq!(pixel, Pixel::new(1024, 768, Color::rgb(0xff, 0x0f, 0x00)));
        let pixel: Result<Pixel, ParsePixelError> = "PX 1024 768 ff0f00 hallo".parse();
        assert_eq!(pixel.unwrap_err(), ParsePixelError::WrongFormat);
        let pixel: Result<Pixel, ParsePixelError> = "PX 1024 768".parse();
        assert_eq!(pixel.unwrap_err(), ParsePixelError::WrongFormat);
    }

    #[test]
    fn display_color_rgb() {
        let rgb = Color::rgb(0x00, 0x00, 0x00);
        assert_eq!(format!("{}", rgb), "000000");
        let rgb = Color::rgb(0x0f, 0x0f, 0x0f);
        assert_eq!(format!("{}", rgb), "0f0f0f");
        let rgb = Color::rgb(0xff, 0xff, 0xff);
        assert_eq!(format!("{}", rgb), "ffffff");
    }

    #[test]
    fn display_color_rgba() {
        let rgba = Color::rgba(0x00, 0x00, 0x00, 0x00);
        assert_eq!(format!("{}", rgba), "00000000");
        let rgba = Color::rgba(0x0f, 0x0f, 0x0f, 0x0f);
        assert_eq!(format!("{}", rgba), "0f0f0f0f");
        let rgba = Color::rgba(0xff, 0xff, 0xff, 0xff);
        assert_eq!(format!("{}", rgba), "ffffffff");
    }

    #[test]
    fn fromstr_color() {
        let color: Color = "ff0f00".parse().unwrap();
        assert_eq!(color, Color::rgb(0xff, 0x0f, 0x00));
        let color: Color = "ff0f00f0".parse().unwrap();
        assert_eq!(color, Color::rgba(0xff, 0x0f, 0x00, 0xf0));
        let color: Result<Color, ParseColorError> = "000000f".parse();
        assert_eq!(color.unwrap_err(), ParseColorError::WrongSize);
    }
}

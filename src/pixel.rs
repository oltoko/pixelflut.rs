use std::fmt;
use std::fmt::{Debug, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use custom_error::custom_error;

/// A Pixel!
///
/// It consist out of Coordinates which define where on the Grid it
/// supposed to be and a Color which defines how it should look like.
/// A Pixel can be send by clients to the server to draw it on the Grid.
///
/// # Example
/// ```
/// # use pixelflut_rs::pixel::{Pixel, Coordinate, Color};
/// let pixel: Pixel = "PX 1024 768 ff0f00".parse().unwrap();
/// assert_eq!(pixel, Pixel::new(Coordinate::new(1024, 768), Color::rgb(0xff, 0x0f, 0x00)));
/// ```
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Pixel {
    coordinate: Coordinate,
    color: Color,
}

impl Pixel {
    /// Creates a new Pixel with the given Coordinate and Color.
    pub fn new(coordinate: Coordinate, color: Color) -> Pixel {
        Pixel { coordinate, color }
    }

    /// Returns the Coordinates of this Pixel.
    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    /// Returns the Color of this Pixel.
    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.coordinate, self.color)
    }
}

impl FromStr for Pixel {
    type Err = ParsePixelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        // First part should be 'PX'
        match parts.next() {
            Some("PX") => (),
            Some(_) => return Err(ParsePixelError::WrongFormat),
            None => return Err(ParsePixelError::WrongFormat),
        }

        let pixel = Pixel::new(
            Coordinate::new(
                parts.next().ok_or(ParsePixelError::WrongFormat)?.parse()?,
                parts.next().ok_or(ParsePixelError::WrongFormat)?.parse()?,
            ),
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
    ParseInt{source: ParseIntError} = "no valid 32-bit integer found",
    ParseColor{source: ParseColorError} = "failed to parse color",
    WrongFormat            = "the string has the wrong format"
}

/// Coordinates to uniquely determine the position of a Pixel.
///
/// # Example
/// ```
/// # use pixelflut_rs::pixel::Coordinate;
/// let coord: Coordinate = "PX 1024 768".parse().unwrap();
/// assert_eq!(coord, Coordinate::new(1024, 768));
/// ```
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    /// Creates a new Coordinate for the given x and y values.
    pub fn new(x: u32, y: u32) -> Coordinate {
        Coordinate { x, y }
    }

    /// Returns the x value of this Coordinate.
    pub fn x(&self) -> u32 {
        self.x
    }

    /// Returns the y value of this Coordinate.
    pub fn y(&self) -> u32 {
        self.y
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "PX {} {}", self.x, self.y)
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        // First part should be 'PX'
        match parts.next() {
            Some("PX") => (),
            Some(_) => return Err(ParseCoordinateError::WrongFormat),
            None => return Err(ParseCoordinateError::WrongFormat),
        }

        let coordinate = Coordinate::new(
            parts.next().ok_or(ParseCoordinateError::WrongFormat)?.parse()?,
            parts.next().ok_or(ParseCoordinateError::WrongFormat)?.parse()?,
        );

        if parts.next().is_some() {
            Err(ParseCoordinateError::WrongFormat)
        } else {
            Ok(coordinate)
        }
    }
}

custom_error! {#[derive(PartialEq)] pub ParseCoordinateError
    ParseInt{source: ParseIntError} = "no valid integer found",
    WrongFormat            = "the string has the wrong format"
}

/// The Color which is used for a Pixel.
///
/// You can create a Color as normal RGB or add an alpha channel to it.
///
/// # Example
/// ## RGB Color
/// ```
/// # use pixelflut_rs::pixel::Color;
/// let color: Color = "ff0f00".parse().unwrap();
/// assert_eq!(color, Color::rgb(0xff, 0x0f, 0x00));
/// assert!(color.is_rgb())
/// ```
/// ### RGBA Color
/// ```
/// # use pixelflut_rs::pixel::Color;
/// let color: Color = "ff0f00aa".parse().unwrap();
/// assert_eq!(color, Color::rgba(0xff, 0x0f, 0x00, 0xaa));
/// assert!(color.is_rgba())
/// ```
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: Option<u8>,
}

impl Color {
    /// Creates a new Color without an alpha channel.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// let color = Color::rgb(0xff, 0x0f, 0x00);
    /// assert!(color.is_rgb());
    /// assert!(!color.is_rgba());
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: None }
    }

    /// Creates a new Color with an alpha channel.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// let color = Color::rgba(0xff, 0x0f, 0x00, 0xaa);
    /// assert!(color.is_rgba());
    /// assert!(!color.is_rgb());
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r,
            g,
            b,
            a: Some(a),
        }
    }

    /// Returns the rgb values of this Color.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// assert_eq!((0xff, 0x0f, 0x00), Color::rgb(0xff, 0x0f, 0x00).rgb_values());
    /// ```
    pub fn rgb_values(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Returns the rgb and the alpha value of this Color.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// assert_eq!((0xff, 0x0f, 0x00, None), Color::rgb(0xff, 0x0f, 0x00).rgba_values());
    /// assert_eq!((0xff, 0x0f, 0x00, Some(0xaa)), Color::rgba(0xff, 0x0f, 0x00, 0xaa).rgba_values());
    /// ```
    pub fn rgba_values(&self) -> (u8, u8, u8, Option<u8>) {
        (self.r, self.g, self.b, self.a)
    }

    /// Returns `true` if this Color doesn't have an alpha channel.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// let color = Color::rgb(0xff, 0x0f, 0x00);
    /// assert!(color.is_rgb());
    /// let color = Color::rgba(0xff, 0x0f, 0x00, 0xaa);
    /// assert!(!color.is_rgb())
    /// ```
    pub fn is_rgb(&self) -> bool {
        self.a.is_none()
    }

    /// Returns `true` if this Color does have an alpha channel.
    ///
    /// ```
    /// # use pixelflut_rs::pixel::Color;
    /// let color = Color::rgb(0xff, 0x0f, 0x00);
    /// assert!(!color.is_rgba());
    /// let color = Color::rgba(0xff, 0x0f, 0x00, 0xaa);
    /// assert!(color.is_rgba())
    /// ```
    pub fn is_rgba(&self) -> bool {
        self.a.is_some()
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
    use crate::pixel::{Color, Coordinate, ParseColorError, ParseCoordinateError, ParsePixelError, Pixel};

    #[test]
    fn display_pixel() {
        let px = Pixel::new(Coordinate::new(1024, 768), Color::rgb(0x00, 0xff, 0x00));
        assert_eq!(px.to_string(), "PX 1024 768 00ff00")
    }

    #[test]
    fn fromstr_pixel() {
        let pixel: Pixel = "PX 1024 768 ff0f00".parse().unwrap();
        assert_eq!(pixel, Pixel::new(Coordinate::new(1024, 768), Color::rgb(0xff, 0x0f, 0x00)));
        let pixel: Result<Pixel, ParsePixelError> = "PX 1024 768 ff0f00 hallo".parse();
        assert_eq!(pixel.unwrap_err(), ParsePixelError::WrongFormat);
        let pixel: Result<Pixel, ParsePixelError> = "PX 1024 768".parse();
        assert_eq!(pixel.unwrap_err(), ParsePixelError::WrongFormat);
        let pixel: Result<Pixel, ParsePixelError> = "nope 1024 768 ff0f00".parse();
        assert_eq!(pixel.unwrap_err(), ParsePixelError::WrongFormat);
    }

    #[test]
    fn display_coordinate() {
        let coord = Coordinate::new(1024, 768);
        assert_eq!(coord.to_string(), "PX 1024 768")
    }

    #[test]
    fn fromstr_coordinate() {
        let coord: Coordinate = "PX 1024 768".parse().unwrap();
        assert_eq!(coord, Coordinate::new(1024, 768));
        let pixel: Result<Coordinate, ParseCoordinateError> = "PX 1024 768 ff0f00".parse();
        assert_eq!(pixel.unwrap_err(), ParseCoordinateError::WrongFormat);
        let pixel: Result<Coordinate, ParseCoordinateError> = "PX 1024".parse();
        assert_eq!(pixel.unwrap_err(), ParseCoordinateError::WrongFormat);
        let pixel: Result<Coordinate, ParseCoordinateError> = "nope 1024 768".parse();
        assert_eq!(pixel.unwrap_err(), ParseCoordinateError::WrongFormat);
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

use crate::pixel::{Coordinate, Pixel};

/// The size of a Grid, defined by x and y.
///
/// The size of the Grid is defined so that the following can actually be drawn on the grid:
/// ```compile_fail
/// # use pixelflut_rs::grid::{Grid, Size};
/// # use pixelflut_rs::pixel::Pixel;
/// # let grid: Grid;
/// assert_eq!(grid.size(), Size::new(1024, 768));
/// let pixel: Pixel = "PX 1023 767 ff0f00".parse()?;
/// grid.draw(pixel);
/// ```
/// The following is not working because it is out of bounds:
/// ```compile_fail
/// # use pixelflut_rs::grid::{Grid, Size};
/// # use pixelflut_rs::pixel::Pixel;
/// # let grid: Grid;
/// assert_eq!(grid.size(), Size::new(1024, 768));
/// let pixel: Pixel = "PX 1024 768 ff0f00".parse()?;
/// grid.draw(pixel);
/// ```
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub struct Size {
    x: usize,
    y: usize,
}

impl Size {
    /// Creates a new Size for the given x and y values.
    pub fn new(x: usize, y: usize) -> Size {
        Size { x, y }
    }

    /// Returns the x value.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns the y value.
    pub fn y(&self) -> usize {
        self.y
    }
}

/// The Grid which can be implemented by your Project to attach the Pixelflut interface to it.
pub trait Grid {
    /// Returns the Size of this Grid.
    ///
    /// The size of the Grid is defined so that the following can actually be drawn on the grid:
    /// ```compile_fail
    /// # use pixelflut_rs::grid::{Grid, Size};
    /// # use pixelflut_rs::pixel::Pixel;
    /// # let grid: Grid;
    /// assert_eq!(grid.size(), Size::new(1024, 768));
    /// let pixel: Pixel = "PX 1023 767 ff0f00".parse()?;
    /// grid.draw(pixel);
    /// ```
    /// The following is not working because it is out of bounds:
    /// ```compile_fail
    /// # use pixelflut_rs::grid::{Grid, Size};
    /// # use pixelflut_rs::pixel::Pixel;
    /// # let grid: Grid;
    /// assert_eq!(grid.size(), Size::new(1024, 768));
    /// let pixel: Pixel = "PX 1024 768 ff0f00".parse()?;
    /// grid.draw(pixel);
    /// ```
    fn size(&self) -> Size;

    /// Draw the given Pixel on the Grid.
    fn draw(&mut self, px: &Pixel);

    /// Fetch the current status of the Pixel for the given Coordinates. Returns None if no such
    /// Pixel exists.
    fn fetch(&self, p: Coordinate) -> Option<Pixel>;
}

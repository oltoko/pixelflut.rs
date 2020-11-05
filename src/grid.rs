use crate::pixel::Pixel;

pub struct Size {
    x: u32,
    y: u32,
}

impl Size {
    pub fn new(x: u32, y: u32) -> Size {
        Size { x, y }
    }

    pub fn xy(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

pub trait Grid {
    fn size(&self) -> Size;
    fn draw(&self, px: Pixel);
}

use simple_logger::SimpleLogger;

use pixelflut_rs::grid::{Grid, Size};
use pixelflut_rs::pixel::{Color, Coordinate, Pixel};
use pixelflut_rs::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init().unwrap();

    let grid = VecGrid::new(Size::new(1024, 768));

    let server = Server::new("0.0.0.0".parse()?, 2342, grid);
    server.start().await
}

struct VecGrid {
    size: Size,
    frame: Vec<Vec<Color>>,
}

impl VecGrid {
    pub fn new(size: Size) -> VecGrid {
        let black = Color::rgb(0x00, 0x00, 0x00);
        let frame = vec![vec![black; size.y()]; size.x()];
        VecGrid {
            size,
            frame,
        }
    }
}

impl Grid for VecGrid {
    fn size(&self) -> Size {
        self.size.clone()
    }

    fn draw(&mut self, px: &Pixel) {
        let x = px.coordinate().x();
        let y = px.coordinate().y();

        if x < self.size.x() && y < self.size.y() {
            self.frame[x][y] = px.color();
        }
    }

    fn fetch(&self, coord: Coordinate) -> Option<Pixel> {
        let x = coord.x();
        let y = coord.y();

        if x < self.size.x() && y < self.size.y() {
            let color = self.frame[x][y];
            return Some(Pixel::new(coord, color))
        }

        None
    }
}


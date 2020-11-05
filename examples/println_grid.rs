use pixelflut_rs::grid::{Size, Grid};
use pixelflut_rs::pixel::Pixel;
use pixelflut_rs::server::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let grid = PrintlnGrid {
        size: Size::new(1024, 768),
    };

    let server = Server::new("0.0.0.0".parse()?, 2342, grid);
    server.start().await
}

struct PrintlnGrid {
    size: Size,
}

impl Grid for PrintlnGrid {
    fn size(&self) -> Size {
        self.size.clone()
    }

    fn draw(&self, px: Pixel) {
        println!("{}", px);
    }
}


/// # Pixelflut-RS
/// A library providing a Pixelflut server to easily connect your display or whatever to it.
///
/// The library enables you to use a working Pixelflut server with your Project. So you don't need
/// to care about the details with the server and the parsing of the commands but concentrate on
/// your visualisation project. In this library we call that a Grid.
///
/// ## Grid
///
/// A Grid can do 3 things
/// 1. Return it's size
/// 2. Draw a given Pixel on it
/// 3. Fetch the current state of a Pixel on the Grid for a given Coordinate
///
/// A really naive implementation could look like this:
/// ```no_run
/// # use pixelflut_rs::grid::{Grid, Size};
/// # use pixelflut_rs::pixel::{Pixel, Coordinate};
/// struct PrintlnGrid {
///     size: Size,
/// }
///
/// impl Grid for PrintlnGrid {
///     fn size(&self) -> Size {
///         self.size.clone()
///     }
///
///     fn draw(&mut self, px: Pixel) {
///         println!("{}", px);
///     }
///
///     fn fetch(&self, p: Coordinate) -> Pixel {
///         "PX 1024 768 ff0f00".parse().unwrap()
///     }
/// }
/// ```
///
/// ## Server
/// To actually run your Grid implementation and attach the Pixelflut interface to it you just
/// need to do the following:
///
/// ```no_run
/// # use pixelflut_rs::grid::Grid;
/// # use pixelflut_rs::pixel::{Pixel, Coordinate};
/// use pixelflut_rs::server::Server;
/// use pixelflut_rs::grid::Size;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>>{
///
///     let grid = PrintlnGrid {
///        size: Size::new(1024, 768),
///     };
///
///     let server = Server::new("0.0.0.0".parse()?, 2342, grid);
///     server.start().await
/// }
///
/// # struct PrintlnGrid {
/// #     size: Size,
/// # }
///
/// # impl Grid for PrintlnGrid {
/// #     fn size(&self) -> Size {
/// #         self.size.clone()
/// #     }
///
/// #     fn draw(&mut self, px: Pixel) {
/// #         println!("{}", px);
/// #     }
///
/// #     fn fetch(&self, p: Coordinate) -> Pixel {
/// #         "PX 1024 768 ff0f00".parse().unwrap()
/// #     }
/// # }
/// ```
pub mod grid;
pub mod pixel;
pub mod server;

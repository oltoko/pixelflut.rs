use core::fmt;
use std::fmt::Formatter;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Instant;

use custom_error::custom_error;
use log::{error, info, warn};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, RwLock};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

use crate::grid::{Grid, Size};
use crate::pixel::Pixel;

const PIXEL_BUFFER: usize = 1024;

const HELP: &str = "\
HELP Pixelflut Commands:\n\
HELP - PX <x> <y> <RRGGBB[AA]>\n\
HELP - PX <x> <y>   >>  PX <x> <y> <RRGGBB>\n\
HELP - SIZE         >>  SIZE <width> <height>\n\
HELP - HELP         >>  HELP ...";

custom_error! { ServerError
    UnknownCommand = "Unknown command send!"
}

/// The Pixelflut Server.
///
/// The Server is defined by an interface and a port where it should listen on. It
/// also requires a Grid on which the Pixels should be drawn on. To start everything you just
/// need to do the following:
///
/// ```compile_fail
/// let server = Server::new("0.0.0.0".parse()?, 2342, grid);
/// server.start().await
/// ```
pub struct Server<G: Grid + std::marker::Send + std::marker::Sync> {
    interface: IpAddr,
    port: u16,
    grid: Arc<RwLock<G>>,
}

impl<G> Server<G>
    where
        G: 'static + Grid + std::marker::Send + std::marker::Sync,
{
    /// Creates a new Server for the given interface, port and Grid.
    pub fn new(interface: IpAddr, port: u16, grid: G) -> Server<G> {
        Server {
            interface,
            port,
            grid: Arc::new(RwLock::new(grid)),
        }
    }

    /// This method will start your server and will never return without an error.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        // Bind the listener to the address
        let listener = TcpListener::bind((self.interface, self.port)).await?;
        let (tx, rx) = mpsc::channel(PIXEL_BUFFER);

        // Start a dedicated task to draw the pixels in bulks to the grid
        let write_grid = Arc::clone(&self.grid);
        task::spawn(async move {
            draw_pixels(rx, write_grid).await;
        });

        info!("Server is ready and listening to {}:{}", self.interface, self.port);
        loop {
            match listener.accept().await {
                // The second item contains the IP and port of the new connection.
                Ok((mut socket, addr)) => {
                    info!("New connection from {}", addr);
                    let grid = Arc::clone(&self.grid);
                    let tx = tx.clone();
                    task::spawn(async move {
                        match process(&mut socket, grid, tx).await {
                            Ok(()) => info!("{} disconnects", addr),
                            Err(e) => warn!("{} disconnects because of: {}", addr, e),
                        }
                    });
                }
                Err(e) => error!("Error opening socket connection: {}", e),
            };
        }
    }
}

async fn draw_pixels<G: Grid>(mut rx: Receiver<Pixel>, grid: Arc<RwLock<G>>) {
    let buf: &mut Vec<Pixel> = &mut vec!();
    let mut time = Instant::now();

    loop {
        if let Some(px) = rx.recv().await {
            buf.push(px);
        }

        if !buf.is_empty() && (buf.len() > PIXEL_BUFFER || time.elapsed().as_micros() > 900) {
            let mut grid = grid.write().await;
            buf.iter().for_each(|px| grid.draw(px));
            buf.clear();
            time = Instant::now();
        }
    }
}

async fn process<G: Grid>(
    socket: &mut TcpStream,
    grid: Arc<RwLock<G>>,
    tx: Sender<Pixel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rd, mut wr) = io::split(socket);
    let reader = BufReader::new(rd);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("PX") => {
                match parts.count() {
                    // PX <x> <y>
                    2 => {
                        let pixel: Option<Pixel>;
                        {
                            let grid = grid.read().await;
                            pixel = grid.fetch(line.parse()?);
                        }
                        if pixel.is_some() {
                            let pixel = format!("{}\n", pixel.unwrap());
                            wr.write(pixel.as_bytes()).await?;
                        }
                    }
                    // PX <x> <y> <RRGGBB[AA]>
                    3 => {
                        tx.send(line.parse()?).await?;
                    }
                    _ => return Err(Box::new(ServerError::UnknownCommand)),
                }
            }
            Some("SIZE") => {
                let size;
                {
                    let grid = grid.read().await;
                    size = format!("{}\n", grid.size());
                }
                wr.write(size.as_bytes()).await?;
            }
            Some("HELP") => {
                let help = format!("{}\n", HELP);
                wr.write(help.as_bytes()).await?;
            }
            _ => return Err(Box::new(ServerError::UnknownCommand)),
        }
    }

    Ok(())
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "SIZE {} {}", self.x(), self.y())
    }
}

#[cfg(test)]
mod tests {
    use crate::grid::Size;

    #[test]
    fn display_size() {
        let size = Size::new(1024, 768);
        assert_eq!(size.to_string(), "SIZE 1024 768\n");
    }
}

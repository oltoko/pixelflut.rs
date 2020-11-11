use core::fmt;
use std::fmt::Formatter;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use custom_error::custom_error;
use log::{error, info, warn};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use crate::grid::{Grid, Size};

const HELP: &str = "\
HELP Pixelflut Commands:\n\
HELP - PX <x> <y> <RRGGBB[AA]>\n\
HELP - SIZE         >>  SIZE <width> <height>\n\
HELP - HELP         >>  HELP ...\n";

custom_error! { ServerError
    UnknownCommand = "Unknown command send!"
}

pub struct Server<G: Grid + std::marker::Send> {
    interface: IpAddr,
    port: u16,
    grid: Arc<Mutex<G>>,
}

impl<G> Server<G>
    where
        G: 'static + Grid + std::marker::Send,
{
    pub fn new(interface: IpAddr, port: u16, grid: G) -> Server<G> {
        Server {
            interface,
            port,
            grid: Arc::new(Mutex::new(grid)),
        }
    }

    /// This method never returns.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        // Bind the listener to the address
        let listener = TcpListener::bind((self.interface, self.port)).await?;

        info!("Server is ready and listening to {}:{}", self.interface, self.port);
        loop {
            match listener.accept().await {
                // The second item contains the IP and port of the new connection.
                Ok((mut socket, addr)) => {
                    info!("New connection from {}", addr);
                    let grid = Arc::clone(&self.grid);
                    task::spawn(async move {
                        match process(&mut socket, grid).await {
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

async fn process<G: Grid>(
    socket: &mut TcpStream,
    grid: Arc<Mutex<G>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rd, mut wr) = io::split(socket);
    let reader = BufReader::new(rd);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("PX") => {
                let grid = grid.lock().unwrap();
                grid.draw(line.parse()?)
            },
            Some("SIZE") => {
                let size;
                {
                    let grid = grid.lock().unwrap();
                    size = grid.size().to_string();
                }
                wr.write(size.as_bytes()).await?;
            },
            Some("HELP") => { wr.write(HELP.as_bytes()).await?; }
            _ => return Err(Box::new(ServerError::UnknownCommand)),
        }
    }

    Ok(())
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let xy = self.xy();
        writeln!(f, "SIZE {} {}", xy.0, xy.1)
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

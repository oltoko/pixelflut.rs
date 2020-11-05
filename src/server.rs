use core::fmt;
use std::fmt::Formatter;
use std::net::IpAddr;

use custom_error::custom_error;
use log::{error, info};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task;

use crate::grid::{Grid, Size};
use crate::pixel::Pixel;

custom_error! { CommError
    WrongCommand = "Wrong command send!"
}

pub struct Server<G: Grid + std::marker::Send> {
    interface: IpAddr,
    port: u16,
    grid: G,
}

impl<G> Server<G>
where
    G: 'static + Grid + std::marker::Send,
{
    pub fn new(interface: IpAddr, port: u16, grid: G) -> Server<G> {
        Server {
            interface,
            port,
            grid,
        }
    }

    /// This method never returns.
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error>> {
        // Bind the listener to the address
        let listener = TcpListener::bind((self.interface, self.port)).await?;

        let size = self.grid.size().to_string();

        let (tx, mut rx) = mpsc::channel(32);
        let grid = self.grid;
        task::spawn(async move {
            while let Some(px) = rx.recv().await {
                grid.draw(px);
            }
        });

        info!("Server is ready and listening to {}:{}", self.interface, self.port);
        loop {
            match listener.accept().await {
                // The second item contains the IP and port of the new connection.
                Ok((mut socket, addr)) => {
                    info!("New connection from {}", addr);
                    let size = size.clone();
                    let txpx = tx.clone();
                    task::spawn(async move {
                        let _ = process(&mut socket, size, txpx).await;
                    });
                }
                Err(e) => error!("Error opening socket connection: {}", e),
            };
        }
    }
}

async fn process(
    socket: &mut TcpStream,
    size: String,
    txpx: Sender<Pixel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (rd, mut wr) = io::split(socket);
    let reader = BufReader::new(rd);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        match line {
            cmd if cmd.starts_with("PX ") => txpx.send(cmd.parse()?).await?,
            cmd if cmd.starts_with("SIZE") => {
                wr.write(size.as_bytes()).await?;
                return Ok(());
            }
            _ => return Err(Box::new(CommError::WrongCommand)),
        }
    }

    Ok(())
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let xy = self.xy();
        write!(f, "SIZE {} {}\n", xy.0, xy.1)
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

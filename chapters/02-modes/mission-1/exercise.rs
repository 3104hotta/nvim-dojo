use std::io;
use std::net::SocketAddr;

#[derive(Debug)]
pub struct Server {
    addr: SocketAddr,
    port: u16,
    max_clients: usize,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server {
            addr,
            port: addr.port(),
            max_clients: 128,
        }
    }

    fn start(&mut self) -> Result<(), io::Error> {
        let addr = format!("{}:{}", self.addr.ip(), self.port);
        println!("Listening on {}", addr);
        Ok(())
    }

    pub fn shutdown(&mut self) {
        println!("Shutting down server");
    }
}

pub fn bind(host: &str, port: u16) -> Result<SocketAddr, io::Error> {
    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    Ok(addr)
}

fn main() -> Result<(), io::Error> {
    // TODO: load config from file
    let addr = bind("0.0.0.0", 8080)?;
    let mut server = Server::new(addr);
    server.start()?;
    Ok(())
}

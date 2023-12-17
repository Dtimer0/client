use std::io::prelude::*;
use std::net::TcpStream;

pub fn main() -> Result<TcpStream, std::io::Error> {
    let mut stream = TcpStream::connect("0.0.0.0:50000")?;
    Ok(stream)
}
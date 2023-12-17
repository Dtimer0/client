use std::io::prelude::*;
use std::net::TcpStream;
use std::{net::TcpListener, io::Read};
mod connecter;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("0.0.0.0:50000")?;
    let bytes = b"Entry";
    let byte_vec = bytes.to_vec();
    stream.write_all(&add_type(byte_vec, 0))?;
    stream.flush()?;
    Ok(())
}

fn add_type(bytes: Vec<u8>, dtype: u8) -> Vec<u8> {
    let mut new_bytes = Vec::new();
    new_bytes.push(dtype);
    new_bytes.extend(bytes);
    new_bytes
}



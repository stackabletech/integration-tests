use anyhow::{anyhow, Result};
use std::net::TcpStream;

/// Scan port of an address.
pub fn scan_port(address: &str) -> Result<()> {
    match TcpStream::connect(address) {
        Ok(_) => {
            println!("PortCheck for [{}] successful", address);
            Ok(())
        }
        Err(err) => Err(anyhow!(
            "TCP error occurred when connecting to [{}]: {}",
            address,
            err.to_string()
        )),
    }
}

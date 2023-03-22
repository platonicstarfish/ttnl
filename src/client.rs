use std::io::{Error, ErrorKind};
use std::net::{SocketAddr};
use std::{io};


pub fn client(_tor_socket_addr: SocketAddr, _dest: SocketAddr) -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet"))
}

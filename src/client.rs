use std::net::{SocketAddr};
use std::time::Duration;
use std::{io};


const THREAD_COUNT: usize = 12;


pub fn client(tor_socket_addr: SocketAddr, (host, port): (String, u16))-> io::Result<()> {
    Ok(())
}

fn maintain_connection_pool((host, port): (String, u16)) -> io::Result<()> {
    //let mut pool = Vec::new();
    loop {
    //    if pool.len() < THREAD_COUNT {
            
            //pool.push(connection);
    //    }
    }
    Ok(())
}

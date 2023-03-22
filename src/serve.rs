use std::io::Write;
use std::net::TcpListener;
use std::net::SocketAddr;
use std::io;
use std::net::IpAddr;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use prost::Message;
use rand::Rng;

use crate::msg;

pub(crate) fn serve(ipaddr: IpAddr, port: u16) -> io::Result<()>{
    debug!("Starting in server mode");

    let socket_addr = SocketAddr::new(ipaddr, port);
    let listener = TcpListener::bind(socket_addr)?;

    info!("Listening on {0}", socket_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(move || {
                    handle_connection(stream)
                });
            },
            Err(e) => {
                return Err(e);
            },
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    info!("Accepted connection from {0}", stream.peer_addr()?);
    let mut rng = rand::thread_rng();
    loop {
        thread::sleep(Duration::from_millis(1000));

        debug!("Measuring latency");
        let msg = msg::PingPong {
            id: rng.gen(),
        };

        let buf = msg.encode_length_delimited_to_vec();
        stream.write_all(&buf)?;
    }
}

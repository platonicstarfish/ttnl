use std::io::{Error, ErrorKind, Write};
use std::{io, net::SocketAddr};
use std::net::{IpAddr, TcpListener};
use clap::{Parser};
use init_log::LevelFilter;
#[macro_use] extern crate log;

mod init_log;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    // Verbosity level (can be repeated twice)
    #[arg(short, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Interface to bind to
    #[arg(short = 'b', default_value = "::")]
    bind_addr: IpAddr,
    
    /// Server mode (i.e. the server end of the tunnel)
    #[arg(short = 's', default_value_t = false)]
    server: bool,

    /// Port used for TBD....
    #[arg(short = 'p')]
    bind_port: u16,
    
}

fn main() {
    let cli = Cli::parse();

    let filter_level = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2.. => LevelFilter::Trace,
    };

    init_log::init_log(filter_level);

    let result = match cli.server {
        true => serve(cli.bind_addr, cli.bind_port),
        false => client(),
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1);
    };
}

fn serve(ipaddr: IpAddr, port: u16) -> io::Result<()>{
    debug!("Starting in server mode");

    let socket_addr = SocketAddr::new(ipaddr, port);
    let listener = TcpListener::bind(socket_addr)?;

    info!("Listening on {0}", socket_addr);

    for stream in listener.incoming() {
        let mut stream = stream?;
        info!("Accepted connection from {0}", stream.peer_addr()?);
        stream.flush()?;
    }

    Ok(())
}

fn client() -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet"))
}

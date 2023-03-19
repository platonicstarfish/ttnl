use std::io::{Error, ErrorKind, Write};
use std::{io, net::SocketAddr};
use std::net::{IpAddr, TcpListener};
use clap::{Parser};
use pretty_env_logger;
use log::LevelFilter;
#[macro_use] extern crate log;
use colored::*;


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

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let filter_level = match cli.verbose {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2.. => LevelFilter::Trace,
    };

    pretty_env_logger::formatted_builder()
       .filter_level(filter_level)
       .format(move | buf, record| {
            let level_symbol = match record.level() {
                log::Level::Error => "✖".red(),
                log::Level::Warn => "⚠".yellow(),
                log::Level::Info => "ℹ".cyan(),
                log::Level::Debug => "⚙".blue(),
                log::Level::Trace => "➤".magenta(),
            };
            let module_path = record.module_path().unwrap_or("<unknown>");
            writeln!(
                buf,
                "{} [{:<12}] {}",
                level_symbol,
                module_path,
                record.args()
            )
        }).init();

    match cli.server {
        true => serve(cli.bind_addr, cli.bind_port),
        false => client(),
    }?;

    Ok(())
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

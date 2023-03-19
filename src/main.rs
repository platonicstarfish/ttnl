use std::io::{Error, ErrorKind};
use std::{io};
use std::net::{IpAddr};
use clap::{Parser};
use init_log::LevelFilter;
#[macro_use] extern crate log;

mod init_log;
mod serve;


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
        true => serve::serve(cli.bind_addr, cli.bind_port),
        false => client(),
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1);
    };
}

fn client() -> io::Result<()> {
    Err(Error::new(ErrorKind::Other, "Not implemented yet"))
}

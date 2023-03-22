use std::net::{IpAddr, SocketAddr};
use clap::{Parser};
use init_log::LevelFilter;
#[macro_use] extern crate log;

mod init_log;
mod serve;
mod client;
pub mod msg;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {

    /// Verbosity level (can be repeated twice)
    #[arg(short, action = clap::ArgAction::Count, default_value_t = 0, conflicts_with = "quiet")]
    verbose: u8,

    /// Quiet mode
    #[arg(short = 'q', conflicts_with = "verbose", default_value_t = false)]
    quiet: bool,

    /// Interface to bind to in server mode
    #[arg(short = 'b', default_value = "::")]
    bind_addr: IpAddr,
    
    /// Server mode (i.e. the server end of the tunnel)
    #[arg(short = 's', default_value_t = false)]
    server: bool,

    /// Port to bind server to in server mode, port to connect to in client mode
    #[arg(short = 'p')]
    port: u16,

    /// Address to connect to in client mode
    #[arg(name = "DEST_ADDR", required_if_eq("server", "false"))]
    dest_addr: Option<IpAddr>,

    /// Tor SOCKS proxy address
    #[arg(short = 't', default_value = "127.0.0.1:9050")]
    tor_socks: SocketAddr,
}

fn main() {
    let cli = Cli::parse();

    let filter_level = if cli.quiet {
        LevelFilter::Warn
    } else {
        match cli.verbose {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            2.. => LevelFilter::Trace,
        }
    };

    init_log::init_log(filter_level);

    let result = match cli.server {
        true => serve::serve(cli.bind_addr, cli.port),
        false => 
        {
            let dest = SocketAddr::new(cli.dest_addr.unwrap(), cli.port);
            client::client(cli.tor_socks, dest)
        },
    };

    if let Err(e) = result {
        error!("{}", e);
        std::process::exit(1);
    };
}

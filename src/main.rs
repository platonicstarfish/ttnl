use clap::{Parser};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'b', default_value = "::1")]
    bind_addr: std::net::IpAddr,
    
    #[arg(short = 'p')]
    bind_port: u16,
    
}

fn main() {
    let cli = Cli::parse();

    println!("{0} : {1}", cli.bind_addr, cli.bind_port);
}

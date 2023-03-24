use std::io::{Error, ErrorKind};
use std::net::{SocketAddr};
use std::time::Duration;
use std::{io};
use arti_client::config::TorClientConfigBuilder;
use arti_client::isolation::Isolation;
use arti_client::{TorClient, TorClientBuilder, TorClientConfig, IsolationToken, StreamPrefs, TorAddr};
use tokio::runtime::{Runtime};


pub fn client(tor_socket_addr: SocketAddr, dest: SocketAddr) -> io::Result<()> {
    let rt = Runtime::new()?;
    rt.block_on(async_client(tor_socket_addr, dest))
}


async fn async_client(tor_socket_addr: SocketAddr, dest: SocketAddr) -> io::Result<()> {
    info!("Creating client");
    let mut conf_builder = TorClientConfigBuilder::default();
    conf_builder.preemptive_circuits().set_initial_predicted_ports(vec![dest.port()]);
    conf_builder.stream_timeouts().connect_timeout(Duration::from_secs(1));
    let config = conf_builder.build()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create config: {}", e)))?;
    let  client_builder = TorClient::builder().config(config);
    let client = client_builder.create_unbootstrapped()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create client: {}", e)))?;

    info!("Bootstrapping");
    client.bootstrap().await.map_err(|e| Error::new(ErrorKind::Other, format!("Failed to bootstrap: {}", e)))?;
    
    Ok(())
}


async fn maintain_thread_pool(client: TorClient, dest: SocketAddr) -> io::Result<()> {
    let mut pool = Vec::new();
    loop {
        if pool.len() < 12 {
            let iclient = client.isolated_client();
            let connection = iclient.connect(dest);
            
        }
    }
    Ok(())
}
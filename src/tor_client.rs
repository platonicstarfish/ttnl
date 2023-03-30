use std::io::{Error, ErrorKind};
use std::net::{SocketAddr};
use std::time::Duration;
use std::{io};
use arti_client::config::TorClientConfigBuilder;
use arti_client::isolation::Isolation;
use arti_client::{TorClient, TorClientBuilder, TorClientConfig, IsolationToken, StreamPrefs, TorAddr};
use tokio::runtime::{Runtime};
use tor_rtcompat::PreferredRuntime;


const THREAD_COUNT: usize = 12;


pub fn client(tor_socket_addr: SocketAddr, (host, port): (String, u16))-> io::Result<()> {
    let rt = Runtime::new()?;
    let client = rt.block_on(async_client(tor_socket_addr, port))?;
    rt.block_on(maintain_connection_pool(client, (host, port)))
}


async fn async_client(tor_socket_addr: SocketAddr, port: u16) -> io::Result<(TorClient<PreferredRuntime>)> {
    info!("Creating client");
    let mut conf_builder = TorClientConfigBuilder::default();
    conf_builder.preemptive_circuits().set_initial_predicted_ports(vec![port]);
    conf_builder.stream_timeouts().connect_timeout(Duration::from_secs(1));
    conf_builder.address_filter().allow_local_addrs(true);
    let config = conf_builder.build()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create config: {}", e)))?;
    let  client_builder = TorClient::builder().config(config);
    let client = client_builder.create_unbootstrapped()
        .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create client: {}", e)))?;

    info!("Bootstrapping");
    client.bootstrap().await.map_err(|e| Error::new(ErrorKind::Other, format!("Failed to bootstrap: {}", e)))?;
    
    Ok(client)
}


async fn maintain_connection_pool(client: TorClient<PreferredRuntime>, (host, port): (String, u16)) -> io::Result<()> {
    let mut pool = Vec::new();
    loop {
        if pool.len() < THREAD_COUNT {
            let iclient = client.isolated_client();
            let connection = iclient.connect((host.to_owned(), port)).await
                .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to connect: {}", e)))?;
            pool.push(connection);
        }
    }
    Ok(())
}

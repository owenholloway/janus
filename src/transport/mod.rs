// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use async_trait::async_trait;
use log::error;
use tokio::net::TcpListener;

pub async fn bind_tcp(
    bind_address: String,
    bind_port: String,
) -> Result<TcpListener, std::io::Error> {
    match TcpListener::bind(format!("{}:{}", bind_address, bind_port)).await {
        Ok(socket) => Ok(socket),
        Err(stack) => {
            error!("Could not open socket failed");
            error!("{:?}", stack);
            Err(stack)
        }
    }
}

#[async_trait]
pub trait TcpTransport {
    async fn open_connection(&self, listener: &TcpListener);
}

use dotenv::dotenv;
use std::env;

use janus::transport::{bind_tcp, TcpTransport};
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    simple_logger::init_with_env().unwrap();

    info!("Project Janus");

    let bind_address = match env::var("BIND_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            warn!("Could not find env BIND_ADDRESS, using defaul value 127.0.0.1");
            "127.0.0.1".to_string()
        }
    };

    let bind_port = match env::var("BIND_PORT") {
        Ok(value) => value,
        Err(_) => {
            warn!("Could not find env BIND_PORT, using defaul value 5000");
            "5000".to_string()
        }
    };

    let listener = bind_tcp(bind_address, bind_port).await.unwrap();

    let device = janus::protocols::modbus::device::create_device();

    device.open_connection(&listener).await;

    Ok(())
}

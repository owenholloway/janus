use std::env;

use dotenv::dotenv;
use log::{info, warn};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
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

    let address = 
        format!("{}:{}", bind_address, bind_port);

    let listener = TcpListener::bind(address).await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = vec![0, 255];

            loop {
                let block = socket
                    .read(&mut buffer)
                    .await
                    .expect("failed to read data from socket");

                if block == 0x00 {

                }

            }

        });

    }

}

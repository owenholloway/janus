use std::env;

use dotenv::dotenv;
use log::info;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    simple_logger::init_with_env().unwrap();

    info!("Project Janus");

    let bind_address = 
        env::var("BIND_ADDRESS")
        .unwrap_or("127.0.0.1".to_string());
    let bind_port = 
        env::var("BIND_PORT")
        .unwrap_or("5000".to_string());

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


    Ok(())

}

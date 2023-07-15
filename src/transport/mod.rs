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
    async fn open_connection(&self, listener: TcpListener);
}

/*
async fn open_socket() {

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {

            let mut buffer = vec![0, 255];

            socket.write(b"Hello who are you? ").await;

            let mut frame: Vec<u8> = vec![];

            loop {
                let block = socket
                    .read(&mut buffer)
                    .await
                    .expect("failed to read data from socket");

                for byte in &buffer {
                    if *byte as char == '\n' {
                        socket.write(&frame).await;
                        trace!("{:?}", &frame);
                        frame.clear();
                        break;
                    }
                    frame.push(*byte);
                }



            }
        });
    }
} */

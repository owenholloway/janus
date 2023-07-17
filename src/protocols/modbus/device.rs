use super::data::{
    coil::Coil, discrete_input::DiscreteInput, holding_register::HoldingRegister,
    input_register::InputRegister,
};

use async_trait::async_trait;
use log::{error, trace};

use crate::transport::TcpTransport;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

pub struct Device {
    pub discrete_inputs: Vec<DiscreteInput>,
    pub coils: Vec<Coil>,
    pub input_registers: Vec<InputRegister>,
    pub holding_registers: Vec<HoldingRegister>,
}

pub fn create_device() -> Device {
    Device {
        discrete_inputs: [DiscreteInput::Disabled; 65535].to_vec(),
        coils: [Coil::Disabled; 65535].to_vec(),
        input_registers: [InputRegister::Disabled; 65535].to_vec(),
        holding_registers: [HoldingRegister::Disabled; 65535].to_vec(),
    }
}

#[async_trait]
impl TcpTransport for Device {
    async fn open_connection(&self, listener: &TcpListener) {
        loop {
            listener_loop(listener).await;
        }
    }
}

async fn listener_loop(listener: &TcpListener) {
    let result = listener.accept().await;

    //Exit early
    if result.is_err() {
        error!("Failed to open socket {:?}", result.err());
        return;
    }

    let (mut socket, _) = result.unwrap();

    tokio::spawn(async move {
        let mut buffer = vec![0, 255];
        let mut frame: Vec<u8> = vec![];

        loop {
            let _ = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");

            for byte in &buffer {
                trace!("{:?}", byte);

                if *byte as char == '\n' {
                    trace!("{:?}", &frame);
                    frame.clear();
                    break;
                }

                frame.push(*byte);
            }
        }
    });
}

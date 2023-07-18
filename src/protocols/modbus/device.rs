// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use super::data::{
    coil::Coil, discrete_input::DiscreteInput, holding_register::HoldingRegister,
    input_register::InputRegister,
};

use async_trait::async_trait;
use log::{error, trace};

use crate::protocols::modbus::frame::RequestFrame;
use crate::protocols::modbus::frame::ResponseFrame;
use crate::protocols::modbus::program_data_unit::ProtocolDataUnitResponse;
use crate::protocols::modbus::read_data::ReadData;
use crate::transport::TcpTransport;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
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
            listener_loop(self, listener).await;
        }
    }
}

async fn listener_loop(device: &Device, listener: &TcpListener) {
    let result = listener.accept().await;

    //Exit early
    if result.is_err() {
        error!("Failed to open socket {:?}", result.err());
        return;
    }

    let (mut socket, _) = result.unwrap();

    let device = (*device).clone();

    tokio::spawn(async move {
        let mut buffer = vec![0, 255];

        let mut read_bytes: u16 = 0;
        let mut remaining_bytes: u16 = 0;
        let mut transaction: u16 = 0;
        let mut unit: u8 = 0;
        let mut header: Vec<u8> = vec![];
        let mut frame: Vec<u8> = vec![];

        loop {
            let _ = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");
            for byte in &buffer {
                if read_bytes < 7 {
                    header.push(*byte);
                }

                if read_bytes == 7 {
                    transaction = u16::from(header[0]) << 8 | u16::from(header[1]);
                    unit = header[6];
                    remaining_bytes = u16::from(header[4]) << 8 | u16::from(header[5]);
                    remaining_bytes -= 1;
                }

                if read_bytes >= 7 && remaining_bytes > 0 {
                    frame.push(*byte);
                    remaining_bytes -= 1;
                }

                if read_bytes >= 7 && remaining_bytes == 0 {
                    trace!("header {:?}", &header);
                    trace!("frame {:?}", &frame);

                    let mut return_frame: Vec<u8> = vec![];
                    for append_bytes in transaction.to_be_bytes() {
                        return_frame.push(append_bytes);
                    }
                    for append_bytes in 0u16.to_be_bytes() {
                        return_frame.push(append_bytes);
                    }
                    return_frame.push(0u8);

                    let pdu = frame.generate_request_frame();

                    let result = device.process_request(pdu);

                    let response_frame = match result {
                        Ok(good_frame) => good_frame,
                        Err(error_frame) => {
                            
                            return_frame.push(3u8);
                            return_frame.push(unit);
                            return_frame.push(error_frame.function_code);
                            return_frame.push(error_frame.exception_code);

                            trace!("return_frame {:?}", &return_frame);

                            let _ = socket.write_all(&return_frame).await;

                            header.clear();
                            frame.clear();
                            read_bytes = 0;
                            break;
                        }
                    };

                    match response_frame {
                        ProtocolDataUnitResponse::ReadCoilsResponse(response) => {
                            let response_frame = response.generate_result_frame();
                            return_frame.push(response_frame.len() as u8 + 1);
                            return_frame.push(unit);
                            for response_byte in response_frame {
                                return_frame.push(response_byte);
                            }
                        }
                        ProtocolDataUnitResponse::ReadDiscreteInputsResponse(response) => todo!(),
                    }


                    let _ = socket.write_all(&return_frame).await;
                    trace!("return_frame {:?}", &return_frame);
                    header.clear();
                    frame.clear();
                    read_bytes = 0;
                    break;
                }

                read_bytes += 1;

                // if *byte as char == '' {
                //     trace!("{:?}", &frame);
                //     frame.clear();
                //     break;
                // }
            }
        }
    });
}

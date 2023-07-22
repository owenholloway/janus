// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use async_trait::async_trait;

use log::info;
use log::{error, trace};
use tokio::net::TcpListener;
use tokio_serial::new;

use crate::protocols::modbus::device::Device;
use crate::protocols::modbus::frame::RequestFrame;
use crate::protocols::modbus::frame::ResponseFrame;
use crate::protocols::modbus::program_data_unit::{ProtocolDataUnitResponse, ProtocolDataUnitRequest, ExceptionResponse};
use crate::protocols::modbus::read_data::ReadData;
use crate::transport::TcpTransport;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;


#[derive(Debug)]
struct TcpFrameControl {
    request_frame: Vec<u8>,
    header_complete: bool,
    frame_complete: bool,
    read_bytes: u16,
    remaining_bytes: u16,
    transaction: u16,
    unit: u8,
}

impl TcpFrameControl {

    fn new() -> TcpFrameControl {
        TcpFrameControl { 
            request_frame: vec![],
            header_complete: false,
            frame_complete: false,
            read_bytes: 0, 
            remaining_bytes: 0, 
            transaction: 0, 
            unit: 0
        }
    }

    fn process_bytes(mut self, byte: u8) -> TcpFrameControl {

        self.request_frame.push(byte);
        self.read_bytes+=1;

        self.header_complete |= self.read_bytes == 7;

        if !self.header_complete {
            return self;
        }

        if self.read_bytes == 7 {
            self.transaction = u16::from(self.request_frame[0]) << 8 | u16::from(self.request_frame[1]);
            self.unit = self.request_frame[6];
            self.remaining_bytes = u16::from(self.request_frame[4]) << 8 | u16::from(self.request_frame[5]);
            self.remaining_bytes -= 1;
            return self;
        }

        self.remaining_bytes-=1;

        if self.remaining_bytes == 0 {
            self.frame_complete = true;
        }

        self
    }

    fn prepare_return_good_frame(&self, response: ProtocolDataUnitResponse) -> Vec<u8> {
        let mut frame = vec![];

        for append_bytes in self.transaction.to_be_bytes() {
            frame.push(append_bytes);
        }
        for append_bytes in 0u16.to_be_bytes() {
            frame.push(append_bytes);
        }
        
        let data_bytes = match response {
            ProtocolDataUnitResponse::ReadCoilsResponse(pdu) => pdu.generate_result_frame(),
            ProtocolDataUnitResponse::ReadDiscreteInputsResponse(pdu) => pdu.generate_result_frame(),
        };

        for append_bytes in ((data_bytes.len() + 1) as u16).to_be_bytes() {
            frame.push(append_bytes);
        } 

        frame.push(self.unit);

        for append_bytes in data_bytes {
            frame.push(append_bytes);
        } 

        frame
    }

    fn prepare_return_bad_frame(&self, response: ExceptionResponse) -> Vec<u8> {
        let mut frame = vec![];

        for append_bytes in self.transaction.to_be_bytes() {
            frame.push(append_bytes);
        }
        for append_bytes in 0u16.to_be_bytes() {
            frame.push(append_bytes);
        }
        frame.push(4u8);
        frame.push(self.unit);
        frame.push(response.function_code);
        frame.push(response.exception_code);
        frame
    }

    fn modbus_pdu_frame(&self) -> Vec<u8> {
        self.request_frame[7..].to_vec()
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

        let mut frame_control = TcpFrameControl::new();

        loop {
            let _ = socket
                .read(&mut buffer)
                .await
                .expect("failed to read data from socket");
            for byte in &buffer {

                frame_control = frame_control.process_bytes(*byte);

                if frame_control.frame_complete {
                    info!("request  {:?}", &frame_control.request_frame);

                    let pdu = ProtocolDataUnitRequest::new(frame_control.modbus_pdu_frame());

                    let response = match device.process_request(pdu) {
                        Ok(good) => frame_control.prepare_return_good_frame(good),
                        Err(bad) => frame_control.prepare_return_bad_frame(bad),
                    };

                    info!("response {:?}", &response);
                    let _ = socket.write_all(&response);

                    frame_control = TcpFrameControl::new();
                }

            }
        }
    });
}
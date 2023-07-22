// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use std::u8;

use super::program_data_unit::{
    ProtocolDataUnitRequest, ReadCoilsRequest, ReadCoilsResponse, ReadDiscreteInputsRequest,
    ReadDiscreteInputsResponse,
};

pub trait RequestFrame {
    fn generate_request_frame(&self) -> ProtocolDataUnitRequest;
}

impl RequestFrame for Vec<u8> {
    fn generate_request_frame(&self) -> ProtocolDataUnitRequest {
        match self[0] {
            0x01 => generate_read_coil_request(self.to_vec()),
            0x02 => generate_read_discrete_input_request(self.to_vec()),
            _ => ProtocolDataUnitRequest::UnknownRequest,
        }
    }
}

impl ProtocolDataUnitRequest {
    pub fn new(bytes: Vec<u8>) -> ProtocolDataUnitRequest {
        bytes.generate_request_frame()
    }
}

fn generate_read_coil_request(frame: Vec<u8>) -> ProtocolDataUnitRequest {
    if frame.len() < 5 {
        return ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 0,
            quantity_of_coils: 0,
        });
    }

    ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
        starting_address: (frame[1] << 4) as u16 + frame[2] as u16,
        quantity_of_coils: (frame[3] << 4) as u16 + frame[4] as u16,
    })
}

fn generate_read_discrete_input_request(frame: Vec<u8>) -> ProtocolDataUnitRequest {
    if frame.len() < 5 {
        return ProtocolDataUnitRequest::ReadDiscreteInputsRequest(ReadDiscreteInputsRequest {
            starting_address: 0,
            quantity_of_inputs: 0,
        });
    }

    ProtocolDataUnitRequest::ReadDiscreteInputsRequest(ReadDiscreteInputsRequest {
        starting_address: (frame[1] << 4) as u16 + frame[2] as u16,
        quantity_of_inputs: (frame[3] << 4) as u16 + frame[4] as u16,
    })
}

pub trait ResponseFrame {
    fn generate_result_frame(&self) -> Vec<u8>;
}

impl ResponseFrame for ReadCoilsResponse {
    fn generate_result_frame(&self) -> Vec<u8> {
        let size = self.byte_count;

        let mut frame: Vec<u8> = vec![0x1, size];

        let mut bytes = self.coil_status.clone();

        frame.append(&mut bytes);

        frame
    }
}

impl ResponseFrame for ReadDiscreteInputsResponse {
    fn generate_result_frame(&self) -> Vec<u8> {
        let size = self.byte_count;

        let mut frame: Vec<u8> = vec![0x1, size];

        let mut bytes = self.coil_status.clone();

        frame.append(&mut bytes);

        frame
    }
}

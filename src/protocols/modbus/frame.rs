use std::u8;

use super::program_data_unit::ReadCoilsResponse;

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

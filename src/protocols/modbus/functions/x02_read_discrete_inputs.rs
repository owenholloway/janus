// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use crate::protocols::modbus::{
    data::discrete_input::DiscreteInput,
    program_data_unit::{
        ExceptionResponse, ProtocolDataUnitResponse, ReadDiscreteInputsRequest,
        ReadDiscreteInputsResponse,
    },
    read_data::ProtocolDataUnitProcessing,
};

impl ProtocolDataUnitProcessing for ReadDiscreteInputsRequest {
    fn process(
        &self,
        device: &crate::protocols::modbus::device::Device,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse> {
        if !(self.quantity_of_inputs <= 0x07D0 && self.quantity_of_inputs >= 0x0001) {
            return Err(ExceptionResponse {
                function_code: 0x82,
                exception_code: 0x03,
            });
        }

        let start_address = self.starting_address as usize;
        let final_address = ((self.starting_address as u16) + self.quantity_of_inputs) as usize;

        let discrete_inputs = &device.discrete_inputs[start_address..final_address];

        let mut result: Vec<u8> = vec![];

        let mut index: u8 = 0;
        let mut block: u8 = 0x0;

        for discrete_input in discrete_inputs {
            match discrete_input {
                DiscreteInput::EnabledReadOnly { discrete_value } => {
                    (index, block) = super::boolean_block_addition(index, block, discrete_value.0);
                    if index == 0 {
                        result.push(block);
                        block = 0x0;
                    }
                }
                DiscreteInput::EnabledReadWrite { discrete_value } => {
                    (index, block) = super::boolean_block_addition(index, block, discrete_value.0);
                    if index == 0 {
                        result.push(block);
                        block = 0x0;
                    }
                }
                DiscreteInput::Disabled => {
                    return Err(ExceptionResponse {
                        function_code: 0x82,
                        exception_code: 0x04,
                    })
                }
            }
        }

        result.push(block);

        Ok(ProtocolDataUnitResponse::ReadDiscreteInputsResponse(
            ReadDiscreteInputsResponse {
                byte_count: result.len() as u8,
                coil_status: result,
            },
        ))
    }
}

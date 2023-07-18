use crate::protocols::modbus::{
    data::coil::Coil,
    program_data_unit::{
        ExceptionResponse, 
        ProtocolDataUnitResponse, 
        ReadCoilsRequest, 
        ReadCoilsResponse,
    },
    read_data::ProtocolDataUnitProcessing,
};

impl ProtocolDataUnitProcessing for ReadCoilsRequest {
    fn process(
        &self,
        device: &crate::protocols::modbus::device::Device,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse> {
        if !(self.quantity_of_coils <= 0x07D0 && self.quantity_of_coils >= 0x0001) {
            return Err(ExceptionResponse {
                function_code: 0x81,
                exception_code: 0x03,
            });
        }

        let start_address = self.starting_address as usize;
        let final_address = ((self.starting_address as u16) + self.quantity_of_coils) as usize;

        let coils = &device.coils[start_address..final_address];

        let mut result: Vec<u8> = vec![];

        let mut index: u8 = 0;
        let mut block: u8 = 0x0;

        for coil in coils {
            match coil {
                Coil::EnabledReadOnly { coil_value } => {
                    (index, block) = super::boolean_block_addition(index, block, coil_value.0);
                    if index == 0 {
                        result.push(block);
                        block = 0x0;
                    }
                }
                Coil::EnabledReadWrite { coil_value } => {
                    (index, block) = super::boolean_block_addition(index, block, coil_value.0);
                    if index == 0 {
                        result.push(block);
                        block = 0x0;
                    }
                }
                Coil::Disabled => {
                    return Err(ExceptionResponse {
                        function_code: 0x81,
                        exception_code: 0x04,
                    })
                }
            }
        }

        result.push(block);

        Ok(ProtocolDataUnitResponse::ReadCoilsResponse(
            ReadCoilsResponse {
                byte_count: result.len() as u8,
                coil_status: result,
            },
        ))
    }
}

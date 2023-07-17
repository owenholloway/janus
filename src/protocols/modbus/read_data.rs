use super::{
    device::Device,
    program_data_unit::{
        ExceptionResponse, ProtocolDataUnitRequest, ProtocolDataUnitResponse, ReadCoilsRequest,
        ReadCoilsResponse,
    },
};

pub trait ReadData {
    fn process_request(
        &self,
        pdu: ProtocolDataUnitRequest,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse>;
}

impl ReadData for super::device::Device {
    fn process_request(
        &self,
        pdu: ProtocolDataUnitRequest,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse> {
        match pdu {
            ProtocolDataUnitRequest::ReadCoilsRequest(data) => process_coil(self, data),
            ProtocolDataUnitRequest::UnknownRequest => todo!(),
        }
    }
}

fn process_coil(
    device: &Device,
    request: ReadCoilsRequest,
) -> Result<ProtocolDataUnitResponse, ExceptionResponse> {
    if !(request.quantity_of_coils <= 0x07D0 && request.quantity_of_coils >= 0x0001) {
        return Err(ExceptionResponse {
            function_code: 0x81,
            exception_code: 0x03,
        });
    }

    let start_address = request.starting_address as usize;
    let final_address = ((request.starting_address as u16) + request.quantity_of_coils) as usize;

    let coils = &device.coils[start_address..final_address];

    let mut result: Vec<u8> = vec![];

    let mut index: u8 = 0;
    let mut block: u8 = 0x0;

    for coil in coils {
        match coil {
            super::data::coil::Coil::EnabledReadOnly { coil_value } => {
                (index, block) = boolean_block_addition(index, block, coil_value.0);
                if index == 0 {
                    result.push(block);
                    block = 0x0;
                }
            }
            super::data::coil::Coil::EnabledReadWrite { coil_value } => {
                (index, block) = boolean_block_addition(index, block, coil_value.0);
                if index == 0 {
                    result.push(block);
                    block = 0x0;
                }
            }
            super::data::coil::Coil::Disabled => {
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

fn boolean_block_addition(index: u8, block: u8, value: bool) -> (u8, u8) {
    let add_value = (value as u8) << index;

    if index == 0x7 {
        return (0, block + add_value);
    }

    (index + 1, block + add_value)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolDataUnitRequest {
    ReadCoilsRequest(ReadCoilsRequest),
}

pub struct ExceptionResponse {
    pub function_code: u16,
    pub exception_code: u16,
}

pub enum ProtocolDataUnitResponse {
    ReadCoilsResponse(ReadCoilsResponse),
    ReadDiscreteInputsResponse(ReadDiscreteInputsResponse),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadCoilsRequest {
    pub starting_address: u16,
    pub quantity_of_coils: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadCoilsResponse {
    pub byte_count: u8,
    pub coil_status: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReadDiscreteInputsResponse {
    pub byte_count: u8,
    pub coil_status: Vec<u8>,
}

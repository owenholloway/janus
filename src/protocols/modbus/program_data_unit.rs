#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolDataUnitRequest {
    /// Code 0x01 <br/> Section 6.1
    ReadCoilsRequest(ReadCoilsRequest),
    /// Code 0x02 <br/> Section 6.2
    ReadDiscreteInputsRequest(ReadDiscreteInputsRequest),
    UnknownRequest,
}

pub struct ExceptionResponse {
    pub function_code: u16,
    pub exception_code: u16,
}

pub enum ProtocolDataUnitResponse {
    ReadCoilsResponse(ReadCoilsResponse),
    ReadDiscreteInputsResponse(ReadDiscreteInputsResponse),
}

/// Code 0x01 <br/> Section 6.1
#[derive(Debug, Clone, PartialEq)]
pub struct ReadCoilsRequest {
    pub starting_address: u16,
    pub quantity_of_coils: u16,
}

/// Code 0x02 <br/> Section 6.2
#[derive(Debug, Clone, PartialEq)]
pub struct ReadDiscreteInputsRequest {
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

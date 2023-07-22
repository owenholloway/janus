// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use super::{
    unit::Unit,
    program_data_unit::{ExceptionResponse, ProtocolDataUnitRequest, ProtocolDataUnitResponse},
};

pub trait ReadData {
    fn process_request(
        &self,
        pdu: ProtocolDataUnitRequest,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse>;
}

impl ReadData for super::unit::Unit {
    fn process_request(
        &self,
        pdu: ProtocolDataUnitRequest,
    ) -> Result<ProtocolDataUnitResponse, ExceptionResponse> {
        match pdu {
            ProtocolDataUnitRequest::ReadCoilsRequest(data) => data.process(self),
            ProtocolDataUnitRequest::ReadDiscreteInputsRequest(data) => data.process(self),
            ProtocolDataUnitRequest::UnknownRequest => Err(ExceptionResponse { function_code: 00, exception_code: 00 }),
        }
    }
}

pub trait ProtocolDataUnitProcessing {
    fn process(&self, device: &Unit) -> Result<ProtocolDataUnitResponse, ExceptionResponse>;
}

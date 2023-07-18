use super::{
    device::Device,
    program_data_unit::{
        ExceptionResponse, ProtocolDataUnitRequest, ProtocolDataUnitResponse,
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
            ProtocolDataUnitRequest::ReadCoilsRequest(data) => data.process(self),
            ProtocolDataUnitRequest::ReadDiscreteInputsRequest(data) => data.process(self),
            ProtocolDataUnitRequest::UnknownRequest => todo!(),
        }
    }
}

pub trait ProtocolDataUnitProcessing {
    fn process(&self, device: &Device) -> Result<ProtocolDataUnitResponse, ExceptionResponse>;
}

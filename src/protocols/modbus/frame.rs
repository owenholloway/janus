use super::function_code::FunctionCode;

pub struct ProtocolDataUnit {
    pub function_code: FunctionCode,
    pub data: ProtocolDataUnitData,
}

pub enum ProtocolDataUnitData {
    ReadCoilsProtocolDataUnit{ read_coils_protocol_data_unit: ReadCoilsProtocolDataUnit}
}

pub struct ReadCoilsProtocolDataUnit {
    pub starting_address: u16
}

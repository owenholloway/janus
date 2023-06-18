use super::function_code::FunctionCode;

pub struct ApplicationDataUnit {
    pub additional_address: i8,

}

pub struct ProtocolDataUnit {
    pub function_code: FunctionCode,
    
}

use super::data::{
    coil::Coil, discrete_input::DiscreteInput, holding_register::HoldingRegister,
    input_register::InputRegister,
};

pub struct Device {
    pub discrete_inputs: Vec<DiscreteInput>,
    pub coils: Vec<Coil>,
    pub input_registers: Vec<InputRegister>,
    pub holding_registers: Vec<HoldingRegister>,
}

pub fn create_device() -> Device {
    Device {
        discrete_inputs: [DiscreteInput::Disabled; 65535].to_vec(),
        coils: [Coil::Disabled; 65535].to_vec(),
        input_registers: [InputRegister::Disabled; 65535].to_vec(),
        holding_registers: [HoldingRegister::Disabled; 65535].to_vec(),
    }
}

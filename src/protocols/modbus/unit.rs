// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use super::data::{
    coil::Coil, discrete_input::DiscreteInput, holding_register::HoldingRegister,
    input_register::InputRegister,
};

#[derive(Debug, Clone)]
pub struct Unit {
    pub discrete_inputs: Vec<DiscreteInput>,
    pub coils: Vec<Coil>,
    pub input_registers: Vec<InputRegister>,
    pub holding_registers: Vec<HoldingRegister>,
}

// https://docs.rs/tokio/latest/tokio/sync/index.html#broadcast-channel
// https://docs.rs/tokio/latest/tokio/sync/index.html#watch-channel

pub fn create_device() -> Unit {
    Unit {
        discrete_inputs: [DiscreteInput::Disabled; 65535].to_vec(),
        coils: [Coil::Disabled; 65535].to_vec(),
        input_registers: [InputRegister::Disabled; 65535].to_vec(),
        holding_registers: [HoldingRegister::Disabled; 65535].to_vec(),
    }
}

impl Default for Unit {
    fn default() -> Unit {
        Unit {
            discrete_inputs: [DiscreteInput::Disabled; 65535].to_vec(),
            coils: [Coil::Disabled; 65535].to_vec(),
            input_registers: [InputRegister::Disabled; 65535].to_vec(),
            holding_registers: [HoldingRegister::Disabled; 65535].to_vec(),
        }
    }
}
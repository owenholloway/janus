// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use crate::protocols::modbus::{
    data::{
        coil::{Coil, CoilValue},
        discrete_input::DiscreteInput,
        input_register::InputRegister,
    },
    unit::Unit,
};

#[test]
fn device_news_all_discrete_inputs_disabled() {
    let device: Unit = crate::protocols::modbus::unit::create_device();

    for discrete_input in device.discrete_inputs {
        assert_eq!(discrete_input, DiscreteInput::Disabled);
    }
}

#[test]
fn device_news_all_coils_disabled() {
    let device: Unit = crate::protocols::modbus::unit::create_device();

    for coil in device.coils {
        assert_eq!(coil, Coil::Disabled);
    }
}

#[test]
fn device_can_update_coil_to_enabled() {
    let mut device: Unit = crate::protocols::modbus::unit::create_device();

    for coil in device.coils.clone() {
        assert_eq!(coil, Coil::Disabled);
    }

    device.coils[100] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };

    assert_eq!(
        device.coils[100],
        Coil::EnabledReadOnly {
            coil_value: CoilValue(false)
        }
    );
}

#[test]
fn device_news_all_input_register_disabled() {
    let device: Unit = crate::protocols::modbus::unit::create_device();

    for input_register in device.input_registers {
        assert_eq!(input_register, InputRegister::Disabled);
    }
}

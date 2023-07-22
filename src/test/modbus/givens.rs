// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use crate::protocols::modbus::{
    data::{
        coil::{Coil, CoilValue},
        discrete_input::{DiscreteInput, DiscreteInputValue},
    },
    unit::Unit,
};

pub fn given_device_coils_test_setup_01(mut device: Unit) -> Unit {
    device.coils[100] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[101] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[102] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[103] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[104] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[105] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[106] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[107] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[108] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[109] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[110] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };

    device
}

pub fn given_device_coils_test_setup_02(mut device: Unit) -> Unit {
    device.discrete_inputs[100] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.discrete_inputs[101] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[102] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.discrete_inputs[103] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[104] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[105] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[106] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[107] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[108] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.discrete_inputs[109] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.discrete_inputs[110] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };

    device
}

use crate::protocols::modbus::{
    data::coil::{Coil, CoilValue},
    device::Device,
};

pub fn given_device_coils_test_setup_01(mut device: Device) -> Device {
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

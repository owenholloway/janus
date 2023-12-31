// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InputRegisterValue(pub u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InputRegister {
    EnabledReadOnly {
        input_register_value: InputRegisterValue,
    },
    EnabledReadWrite {
        input_register_value: InputRegisterValue,
    },
    Disabled,
}

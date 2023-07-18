// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HoldingRegisterValue(pub u16);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum HoldingRegister {
    EnabledReadOnly {
        holding_register_value: HoldingRegisterValue,
    },
    EnabledReadWrite {
        holding_register_value: HoldingRegisterValue,
    },
    Disabled,
}

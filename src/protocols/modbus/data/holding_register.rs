#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HoldingRegisterValue(pub u16);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HoldingRegister {
    EnabledReadOnly {
        holding_register_value: HoldingRegisterValue,
    },
    EnabledReadWrite {
        holding_register_value: HoldingRegisterValue,
    },
    Disabled,
}

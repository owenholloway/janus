#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InputRegisterValue(pub u16);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputRegister {
    EnabledReadOnly {
        input_register_value: InputRegisterValue,
    },
    EnabledReadWrite {
        input_register_value: InputRegisterValue,
    },
    Disabled,
}

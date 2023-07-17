#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CoilValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Coil {
    EnabledReadOnly { coil_value: CoilValue },
    EnabledReadWrite { coil_value: CoilValue },
    Disabled,
}

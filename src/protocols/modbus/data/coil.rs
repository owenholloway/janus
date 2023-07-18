#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoilValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Coil {
    EnabledReadOnly { coil_value: CoilValue },
    EnabledReadWrite { coil_value: CoilValue },
    Disabled,
}

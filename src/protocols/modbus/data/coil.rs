// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoilValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Coil {
    EnabledReadOnly { coil_value: CoilValue },
    EnabledReadWrite { coil_value: CoilValue },
    Disabled,
}

// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use super::BooleanValueOperations;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CoilValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Coil {
    EnabledReadOnly { coil_value: CoilValue },
    EnabledReadWrite { coil_value: CoilValue },
    Disabled,
}

impl BooleanValueOperations for Coil {
    fn get_value(&self) -> bool {
        match self {
            Coil::EnabledReadOnly { coil_value } => coil_value.0,
            Coil::EnabledReadWrite { coil_value } => coil_value.0,
            Coil::Disabled => false,
        }
    }
}

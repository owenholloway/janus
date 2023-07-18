// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct DiscreteInputValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiscreteInput {
    EnabledReadOnly { discrete_value: DiscreteInputValue },
    EnabledReadWrite { discrete_value: DiscreteInputValue },
    Disabled,
}

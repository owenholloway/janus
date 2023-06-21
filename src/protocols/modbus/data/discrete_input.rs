#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DiscreteInputValue(pub bool);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DiscreteInput {
    EnabledReadOnly { discrete_value: DiscreteInputValue },
    EnabledReadWrite { discrete_value: DiscreteInputValue },
    Disabled,
}

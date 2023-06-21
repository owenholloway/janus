use self::coil::Coil;

pub mod coil;
pub mod discrete_input;
pub mod holding_register;
pub mod input_register;

pub struct AddressedCoil {
    pub address: u16,
    pub coil: Coil,
}

// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use self::coil::Coil;

pub mod coil;
pub mod discrete_input;
pub mod holding_register;
pub mod input_register;

pub struct AddressedCoil {
    pub address: u16,
    pub coil: Coil,
}


pub trait BooleanValueOperations {
    fn get_value(&self) -> bool;
}


pub trait ByteValueOperations {
    fn get_value(&self) -> u16;
}
use self::x01_read_coil::Coil;

pub mod x01_read_coil;
pub mod discrete_input;
pub mod holding_register;
pub mod input_register;

pub struct AddressedCoil {
    pub address: u16,
    pub coil: Coil,
}

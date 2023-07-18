pub mod x01_read_coils;
pub mod x02_read_discrete_inputs;

fn boolean_block_addition(index: u8, block: u8, value: bool) -> (u8, u8) {
    let add_value = (value as u8) << index;

    if index == 0x7 {
        return (0, block + add_value);
    }

    (index + 1, block + add_value)
}

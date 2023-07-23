// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use crate::protocols::modbus::{
    program_data_unit::{ProtocolDataUnitRequest, ReadCoilsRequest},
    read_data::ReadData,
    unit::Unit,
};

#[test]
fn invalid_coil_request_gets_error() {
    let device: Unit = crate::protocols::modbus::unit::create_device();

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 100,
            quantity_of_coils: 1,
        });

    let result = device.process_request(pdu);

    assert!(result.is_err());

    match result {
        Ok(_) => {
            panic!();
        }
        Err(response) => {
            assert_eq!(response.function_code, 0x81);
            assert_eq!(response.exception_code, 0x04);
        }
    }
}

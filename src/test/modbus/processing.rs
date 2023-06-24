use crate::{
    protocols::modbus::{
        device::Device,
        frame::ResponseFrame,
        program_data_unit::{ProtocolDataUnitRequest, ProtocolDataUnitResponse, ReadCoilsRequest},
        read_data::ReadData,
    },
    test::modbus::givens::given_device_coils_test_setup_01,
};

#[test]
fn valid_coil_request_gets_response() {
    let mut device: Device = crate::protocols::modbus::device::create_device();

    device = given_device_coils_test_setup_01(device);

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 100,
            quantity_of_coils: 11,
        });

    let result = device.process_request(pdu);

    assert!(!result.is_err());

    match result {
        Ok(pdu) => match pdu {
            ProtocolDataUnitResponse::ReadCoilsResponse(response) => {
                assert_eq!(response.coil_status[0], 0x5);
                assert_eq!(response.coil_status[1], 0x6)
            }
            _ => assert!(false),
        },
        Err(_) => {
            assert!(false);
        }
    }
}

#[test]
fn valid_coil_request_gets_frame() {
    let mut device: Device = crate::protocols::modbus::device::create_device();

    device = given_device_coils_test_setup_01(device);

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 100,
            quantity_of_coils: 11,
        });

    let result = device.process_request(pdu);

    assert!(!result.is_err());

    match result {
        Ok(pdu) => match pdu {
            ProtocolDataUnitResponse::ReadCoilsResponse(response) => {
                let frame = response.generate_result_frame();

                assert_eq!(frame[0], 0x1);
                assert_eq!(frame[1], 0x2);
                assert_eq!(frame[2], 0x5);
                assert_eq!(frame[3], 0x6);
            }
            _ => assert!(false),
        },
        Err(_) => {
            assert!(false);
        }
    }
}

#[test]
fn invalid_coil_request_gets_error() {
    let device: Device = crate::protocols::modbus::device::create_device();

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 100,
            quantity_of_coils: 1,
        });

    let result = device.process_request(pdu);

    assert!(result.is_err());

    match result {
        Ok(_) => {
            assert!(false);
        }
        Err(response) => {
            assert_eq!(response.function_code, 0x81);
            assert_eq!(response.exception_code, 0x04);
        }
    }
}

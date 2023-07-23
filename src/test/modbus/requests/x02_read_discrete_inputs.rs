// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use crate::{
    protocols::modbus::{
        frame::{RequestFrame, ResponseFrame},
        program_data_unit::{
            ProtocolDataUnitRequest, ProtocolDataUnitResponse, ReadDiscreteInputsRequest,
        },
        read_data::ReadData,
        unit::Unit,
    },
    test::modbus::givens::given_device_coils_test_setup_02,
};

#[test]
fn valid_request_gets_response() {
    let mut device: Unit = crate::protocols::modbus::unit::create_device();

    device = given_device_coils_test_setup_02(device);

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(ReadDiscreteInputsRequest {
            starting_address: 100,
            quantity_of_inputs: 11,
        });

    let result = device.process_request(pdu);

    assert!(result.is_ok());

    match result {
        Ok(pdu) => match pdu {
            ProtocolDataUnitResponse::ReadDiscreteInputsResponse(response) => {
                assert_eq!(response.coil_status[0], 0x05);
                assert_eq!(response.coil_status[1], 0x06)
            }
            _ => panic!(),
        },
        Err(_) => {
            panic!();
        }
    }
}

#[test]
fn valid_request_gets_frame() {
    let mut device: Unit = crate::protocols::modbus::unit::create_device();

    device = given_device_coils_test_setup_02(device);

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(ReadDiscreteInputsRequest {
            starting_address: 100,
            quantity_of_inputs: 11,
        });

    let result = device.process_request(pdu);

    assert!(result.is_ok());

    match result {
        Ok(pdu) => match pdu {
            ProtocolDataUnitResponse::ReadDiscreteInputsResponse(response) => {
                let frame = response.generate_result_frame();

                assert_eq!(frame[0], 0x01);
                assert_eq!(frame[1], 0x02);
                assert_eq!(frame[2], 0x05);
                assert_eq!(frame[3], 0x06);
            }
            _ => panic!(),
        },
        Err(_) => {
            panic!();
        }
    }
}

#[test]
fn valid_frame_translates_to_request_lsb() {
    let frame: Vec<u8> = vec![2, 0, 1, 0, 1];

    let pdu = frame.generate_request_frame();

    match &pdu {
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(request) => {
            assert_eq!(request.starting_address, 1);
            assert_eq!(request.quantity_of_inputs, 1);
        }
        _ => panic!(),
    }
}

#[test]
fn valid_frame_translates_to_request_msb() {
    let frame: Vec<u8> = vec![2, 1, 0, 1, 0];

    let pdu = frame.generate_request_frame();

    match &pdu {
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(request) => {
            assert_eq!(request.starting_address, 16);
            assert_eq!(request.quantity_of_inputs, 16);
        }
        _ => panic!(),
    }
}

#[test]
fn valid_frame_translates_to_request() {
    let frame: Vec<u8> = vec![2, 1, 1, 1, 1];

    let pdu = frame.generate_request_frame();

    match &pdu {
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(request) => {
            assert_eq!(request.starting_address, 17);
            assert_eq!(request.quantity_of_inputs, 17);
        }
        _ => panic!(),
    }
}

#[test]
fn valid_frame_translates_to_request_gets_response() {
    let frame: Vec<u8> = vec![2, 6, 4, 0, 11];

    let pdu = frame.generate_request_frame();

    let mut device: Unit = crate::protocols::modbus::unit::create_device();

    device = given_device_coils_test_setup_02(device);

    match &pdu {
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(request) => {
            assert_eq!(request.starting_address, 100);
            assert_eq!(request.quantity_of_inputs, 11);
        }
        _ => panic!(),
    }

    let result = device.process_request(pdu);

    assert!(result.is_ok());

    match result {
        Ok(pdu) => match pdu {
            ProtocolDataUnitResponse::ReadDiscreteInputsResponse(response) => {
                let frame = response.generate_result_frame();

                assert_eq!(frame[0], 0x01);
                assert_eq!(frame[1], 0x02);
                assert_eq!(frame[2], 0x05);
                assert_eq!(frame[3], 0x06);
            }
            _ => panic!(),
        },
        Err(_) => {
            panic!();
        }
    }
}

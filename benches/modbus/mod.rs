// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use janus::protocols::modbus::{
    data::{
        coil::{Coil, CoilValue},
        discrete_input::{DiscreteInput, DiscreteInputValue},
    },
    unit::{create_device, Unit},
    program_data_unit::{ProtocolDataUnitRequest, ReadCoilsRequest, ReadDiscreteInputsRequest},
    read_data::ReadData,
};

pub fn bench_coil_processing(coil_count: u16) {
    let mut device: Unit = create_device();

    for index in 0..coil_count {
        device.coils[index as usize] = Coil::EnabledReadOnly {
            coil_value: CoilValue(true),
        };
    }

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadCoilsRequest(ReadCoilsRequest {
            starting_address: 0,
            quantity_of_coils: coil_count,
        });

    let result = device.process_request(pdu);

    match result {
        Ok(_) => {}
        Err(_) => panic!(),
    }
}

pub fn bench_discrete_input_processing(discrete_input_count: u16) {
    let mut device: Unit = create_device();

    for index in 0..discrete_input_count {
        device.discrete_inputs[index as usize] = DiscreteInput::EnabledReadOnly {
            discrete_value: DiscreteInputValue(true),
        };
    }

    let pdu: ProtocolDataUnitRequest =
        ProtocolDataUnitRequest::ReadDiscreteInputsRequest(ReadDiscreteInputsRequest {
            starting_address: 0,
            quantity_of_inputs: discrete_input_count,
        });

    let result = device.process_request(pdu);

    match result {
        Ok(_) => {}
        Err(_) => panic!(),
    }
}

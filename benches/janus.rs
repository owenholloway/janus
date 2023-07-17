use criterion::{criterion_group, criterion_main, Criterion};

use janus::protocols::modbus::{
    data::x01_read_coil::{Coil, CoilValue},
    device::{create_device, Device},
    program_data_unit::{ProtocolDataUnitRequest, ReadCoilsRequest},
    read_data::ReadData,
};

fn bench_coil_processing(coil_count: u16) {
    let mut device: Device = create_device();

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
        Ok(_) => {
            assert!(true)
        }
        Err(_) => assert!(false),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bench_coil_processing 10", |b| {
        b.iter(|| bench_coil_processing(10))
    });
    c.bench_function("bench_coil_processing 100", |b| {
        b.iter(|| bench_coil_processing(100))
    });
    c.bench_function("bench_coil_processing 1000", |b| {
        b.iter(|| bench_coil_processing(1000))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

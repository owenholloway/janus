// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use dotenv::dotenv;
use std::{env, time::Duration};
use tokio::time::sleep;

use janus::{
    protocols::modbus::{
        data::{
            coil::{Coil, CoilValue},
            discrete_input::{DiscreteInput, DiscreteInputValue},
            BooleanValueOperations,
        },
        unit::Unit,
    },
    supporting::{print_license, units_db::ModbusUnit},
    transport::{bind_tcp, TcpTransport},
};
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_license();

    dotenv().ok();

    simple_logger::init_with_env().unwrap();

    info!("Project Janus");

    let bind_address = match env::var("BIND_ADDRESS") {
        Ok(value) => value,
        Err(_) => {
            warn!("Could not find env BIND_ADDRESS, using defaul value 0.0.0.0");
            "0.0.0.0".to_string()
        }
    };

    let bind_port = match env::var("BIND_PORT") {
        Ok(value) => value,
        Err(_) => {
            warn!("Could not find env BIND_PORT, using defaul value 5002");
            "5002".to_string()
        }
    };

    let listener = bind_tcp(bind_address, bind_port).await.unwrap();

    let mut device = ModbusUnit::default();

    device.unit.coils[99] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.unit.coils[100] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.unit.coils[101] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[102] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.unit.coils[103] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[104] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[105] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[106] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[107] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[108] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.unit.coils[109] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.unit.coils[110] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };

    device.unit.discrete_inputs[99] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.unit.discrete_inputs[100] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.unit.discrete_inputs[101] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[102] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.unit.discrete_inputs[103] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[104] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[105] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[106] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[107] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[108] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(false),
    };
    device.unit.discrete_inputs[109] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };
    device.unit.discrete_inputs[110] = DiscreteInput::EnabledReadOnly {
        discrete_value: DiscreteInputValue(true),
    };

    let device_00 = device.unit.clone();
    let tx_mpsc_00 = device.tx_mpsc.clone();
    let rx_watch00 = device.rx_watch.clone();

    let tx_mpsc = device.tx_mpsc.clone();
    let rx_watch = device.rx_watch.clone();

    let flipper = async move {

        loop {
            sleep(Duration::from_millis(2000)).await;

            let unit = rx_watch.borrow().clone();

            let coil = unit.coils[100].get_value();

            tx_mpsc.send(janus::supporting::units_db::UpdatedObject::Coil {
                value: Coil::EnabledReadOnly {
                    coil_value: CoilValue(!coil),
                },
                no: 100,
            }).await;

            let coil = unit.coils[101].get_value();

            tx_mpsc.send(janus::supporting::units_db::UpdatedObject::Coil {
                value: Coil::EnabledReadOnly {
                    coil_value: CoilValue(!coil),
                },
                no: 101,
            }).await;

            let coil = unit.coils[102].get_value();

            tx_mpsc.send(janus::supporting::units_db::UpdatedObject::Coil {
                value: Coil::EnabledReadOnly {
                    coil_value: CoilValue(!coil),
                },
                no: 102,
            }).await;
        }
    };

    tokio::join!(
        device.create_listener(),
        janus::protocols::modbus::transport::tcp_frame::open_connection(
            &listener,
            &device_00,
            &tx_mpsc_00,
            &rx_watch00
        ),
        flipper
    );

    Ok(())
}

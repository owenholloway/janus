// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use dotenv::dotenv;
use std::env;

use janus::{
    protocols::modbus::{data::coil::{Coil, CoilValue}, device::Device},
    supporting::print_license,
    transport::{bind_tcp, TcpTransport},
};
use log::{info, warn};

static DEVICE: Device;

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

    let mut device = janus::protocols::modbus::device::create_device();
    
    device.coils[99] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[100] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[101] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[102] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[103] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[104] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[105] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[106] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[107] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[108] = Coil::EnabledReadOnly {
        coil_value: CoilValue(false),
    };
    device.coils[109] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };
    device.coils[110] = Coil::EnabledReadOnly {
        coil_value: CoilValue(true),
    };

    device.open_connection(&listener).await;

    Ok(())
}

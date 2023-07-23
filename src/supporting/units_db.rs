// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use log::info;
use log::warn;
use tokio::sync::mpsc;
use tokio::sync::watch;

use crate::protocols::modbus::{
    data::{coil::Coil, discrete_input::DiscreteInput},
    unit::Unit,
};

#[derive(Debug)]
pub struct ModbusUnit {
    pub unit: Unit,
    pub tx_mpsc: mpsc::Sender<UpdatedObject>,
    rx_mpsc: mpsc::Receiver<UpdatedObject>,
    tx_watch: watch::Sender<Unit>,
    pub rx_watch: watch::Receiver<Unit>,
}

impl Default for ModbusUnit {
    fn default() -> ModbusUnit {
        let (tx_mpsc, rx_mpsc) = mpsc::channel::<UpdatedObject>(16);

        let (tx_watch, rx_watch) = watch::channel::<Unit>(Unit::default());

        ModbusUnit {
            unit: Unit::default(),
            tx_mpsc,
            rx_mpsc,
            tx_watch,
            rx_watch,
        }
    }
}

impl ModbusUnit {
    pub async fn create_listener(&mut self) {
        while let Some(update) = self.rx_mpsc.recv().await {
            info!("Got update {:?}", update);

            match update {
                UpdatedObject::NoUpdate => todo!(),
                UpdatedObject::Coil { value, no } => self.unit.coils[no] = value,
                UpdatedObject::DiscreteInput { value, no } => self.unit.discrete_inputs[no] = value,
            }

            match self.tx_watch.send(self.unit.clone()) {
                Ok(_) => {}
                Err(error) => warn!("Failed to send update {:}", error),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UpdatedObject {
    NoUpdate,
    Coil { value: Coil, no: usize },
    DiscreteInput { value: DiscreteInput, no: usize },
}

// Copyright Owen Holloway 2023
// License: AGPL-3.0-or-later

use log::info;
use tokio::sync::mpsc::{Sender, Receiver};

use crate::protocols::modbus::{unit::Unit, data::coil::{CoilValue, Coil}};

pub struct ModbusUnit {
    pub unit: Unit,
    pub tx_coil: Sender<CoilUpdate>,
    rx_coil: Receiver<CoilUpdate>
}

impl Default for ModbusUnit {
    fn default() -> ModbusUnit {
        
        let (tx_coil, rx_coil) = tokio::sync::mpsc::channel::<CoilUpdate>(16);

        ModbusUnit { 
            unit: Unit::default(),
            tx_coil,
            rx_coil
        }
    }
}

impl ModbusUnit {
    pub async fn create_listener(&mut self) {

        while let Some(coil_update) = self.rx_coil.recv().await {
            
            self.unit.coils[coil_update.no+0] = coil_update.value;
            self.unit.coils[coil_update.no+1] = coil_update.value;
            
            info!("Got changed unit {:?}", self.unit.coils[coil_update.no+0]);
            info!("Got changed unit {:?}", self.unit.coils[coil_update.no+1]);
        }

    }
}

#[derive(Debug, Clone, Copy)]
pub struct CoilUpdate {
    pub value: Coil,
    pub no: usize,
}
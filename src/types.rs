use std::fs::read_to_string;

use crate::{config::app::AppBatteryConfig, utils::read_to_usize};


#[derive(Copy, Clone, Debug)]
pub enum NetworkStatus {
    Ethernet,
    Wireless,
    UsbTether,
    Disconnected,
}

#[derive(Clone, Debug)]
pub enum BatteryState {
    Charging(usize),
    Discharging(usize),
    NotCharging(usize),
    Unknown(usize, String),
}


impl BatteryState {
    pub fn from_config(config: &AppBatteryConfig) -> Option<Self> {
        let charge_path = config.charge_file.as_ref()?;
        let charge = read_to_usize(charge_path).ok()?;
        let status_path = config.status_file.as_ref()?;
        let status = read_to_string(status_path).ok()?;
        match status.as_str().trim() {
            "Charging" => {
                Some(BatteryState::Charging(charge))
            }
            "Discharging" => {
                Some(BatteryState::Discharging(charge))
            }
            "Not charging" => {
                Some(BatteryState::NotCharging(charge))
            }
            s => {
                Some(BatteryState::Unknown(charge, s.to_string()))
            }
        }
    }

    pub fn to_status_string(&self) -> String {
        match self {
            Self::Charging(charge) => {
                format!("🔋{charge:.1}%+ \u{2502} ")
            }
            Self::Discharging(charge) => {
                if *charge < 30 {
                    format!("🔴🔴🔴🔴🔴{charge:.1}%- \u{2502} ")
                } else {
                    format!("🔋{charge:.1}%- \u{2502} ")
                }
            }
            Self::NotCharging(charge) => {
                format!("🔋{charge:.1}%* \u{2502} ")
            }
            Self::Unknown(charge, status) => {
                format!("🔋{charge:.1}%* \u{2502} {status} ")
            }
        }
    }
}
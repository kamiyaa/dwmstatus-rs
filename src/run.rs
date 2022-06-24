use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use systemstat::{saturating_sub_bytes, Platform};

use crate::config::AppConfig;

#[derive(Copy, Clone, Debug)]
enum NetworkStatus {
    Ethernet,
    Wireless,
    UsbTether,
    Disconnected,
}

fn read_to_usize(p: &Path) -> io::Result<usize> {
    let mut file = fs::File::open(p)?;

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {
            let stripped = s.trim();
            match stripped.parse::<usize>() {
                Ok(u) => Ok(u),
                Err(_) => Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to parse value".to_string(),
                )),
            }
        }
        Err(_) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Failed to read value".to_string(),
        )),
    }
}

fn print_info(config: &AppConfig, sys: &systemstat::System) -> Result<(), std::io::Error> {
    let cpu_temp = if let Some(p) = config.cpu_temp_file.as_ref() {
        match read_to_usize(p) {
            Ok(temp) => Some(temp as f32 / 1000.0),
            _ => None,
        }
    } else {
        None
    };
    let battery_charge = if let Some(p) = config.battery_file.as_ref() {
        match read_to_usize(p) {
            Ok(charge) => Some(charge),
            _ => None,
        }
    } else {
        None
    };
    let battery_drain = if let Some(p) = config.battery_drain.as_ref() {
        match read_to_usize(p) {
            Ok(charge) => Some(charge),
            _ => None,
        }
    } else {
        None
    };

    let uptime: chrono::Duration = match chrono::Duration::from_std(sys.uptime()?) {
        Ok(s) => s,
        Err(e) => {
            let err = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
            return Err(err);
        }
    };

    let mem = sys.memory()?;
    let networks = sys.networks()?;

    let mut network_status = NetworkStatus::Disconnected;
    for (name, network) in networks {
        if network.addrs.is_empty() {
            continue;
        }
        match network_status {
            NetworkStatus::Wireless => break,
            _ => {
                if name.starts_with("wlp") || name.starts_with("wlan") {
                    network_status = NetworkStatus::Wireless;
                } else if name.starts_with("enp") {
                    if name.len() > 7 {
                        network_status = NetworkStatus::UsbTether;
                    } else {
                        network_status = NetworkStatus::Ethernet;
                    }
                }
            }
        }
    }

    let network_str = match network_status {
        NetworkStatus::Wireless => "(---)",
        NetworkStatus::Ethernet => "]---[",
        NetworkStatus::UsbTether => "]~~~[",
        NetworkStatus::Disconnected => "--/--",
    };

    let local_time = chrono::Local::now();
    let local_time_str = local_time.format("%a %m/%d  %I:%M");

    let pst_time = local_time - chrono::Duration::hours(3);
    let pst_time_str = pst_time.format("%I:%M PST");

    print!(
        "{} \u{2502} {} \u{2502} ",
        network_str,
        saturating_sub_bytes(mem.total, mem.free)
    );

    if let Some(temp) = cpu_temp {
        print!("{:.1}\u{00B0}C \u{2502} ", temp);
    }
    if let Some(battery) = battery_charge {
        match battery_drain {
            Some(0) => print!("[{:.1}%+] \u{2502} ", battery),
            _ => print!("[{:.1}%] \u{2502} ", battery),
        }
    }

    println!(
        "{:02}:{:02} \u{2502} {} {} ",
        uptime.num_hours(),
        uptime.num_minutes() % 60,
        pst_time_str,
        local_time_str
    );
    Ok(())
}

pub fn run(config: AppConfig) {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_secs(5);

    loop {
        match print_info(&config, &sys) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }
        std::thread::sleep(refresh_rate);
    }
}

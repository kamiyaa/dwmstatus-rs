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

fn generate_status_bar_string(
    config: &AppConfig,
    sys: &systemstat::System,
) -> Result<String, std::io::Error> {
    let mut status_bar: String = String::with_capacity(128);

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
    status_bar.push_str(&format!("{network_str} \u{2502} "));

    let mem = sys.memory()?;

    status_bar.push_str(&format!(
        "{} \u{2502} ",
        saturating_sub_bytes(mem.total, mem.free)
    ));

    if let (Some(path), Some(denom)) = (
        config.cpu_temp_file.as_ref(),
        config.cpu_temp_denominator.as_ref(),
    ) {
        if let Ok(temp) = read_to_usize(path) {
            let temp_normalized = temp as f32 / *denom;
            status_bar.push_str(&format!("{:.1}\u{00B0}C \u{2502} ", temp_normalized));
        }
    }

    if let Some(path) = config.battery_file.as_ref() {
        if let Ok(charge) = read_to_usize(path) {
            let ac_online = config
                .ac_file
                .as_ref()
                .and_then(|path| read_to_usize(path).ok());
            match ac_online {
                Some(ac_online) if ac_online == 1 => {
                    status_bar.push_str(&format!("[{:.1}%+] \u{2502} ", charge));
                }
                _ => {
                    status_bar.push_str(&format!("[{:.1}%] \u{2502} ", charge));
                }
            }
        }
    }

    let local_time = chrono::Local::now();
    let local_time_str = local_time.format("%a %m/%d  %H:%M");
    if let Ok(uptime) = chrono::Duration::from_std(sys.uptime()?) {
        status_bar.push_str(&format!(
            "{:02}:{:02} \u{2502} {} ",
            uptime.num_hours(),
            uptime.num_minutes() % 60,
            local_time_str
        ));
    };

    Ok(status_bar)
}

pub fn run(config: AppConfig) {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_secs(5);

    loop {
        match generate_status_bar_string(&config, &sys) {
            Ok(s) => {
                println!("{s}");
            }
            Err(err) => {
                println!("{err}");
            }
        }
        std::thread::sleep(refresh_rate);
    }
}

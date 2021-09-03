use std::fs;
use std::io;
use std::io::prelude::*;

use systemstat::{saturating_sub_bytes, Platform};

#[derive(Copy, Clone, Debug)]
enum NetworkStatus {
    Ethernet,
    Wireless,
    UsbTether,
    Disconnected,
}

const CPU_TEMP_PATH: &str = "/sys/class/hwmon/hwmon0/temp1_input";

fn get_cpu_temp() -> io::Result<usize> {
    let mut file = fs::File::open(CPU_TEMP_PATH)?;

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

fn print_info(sys: &systemstat::System) -> Result<(), std::io::Error> {
    let uptime: chrono::Duration = match chrono::Duration::from_std(sys.uptime()?) {
        Ok(s) => s,
        Err(e) => {
            let err = io::Error::new(io::ErrorKind::InvalidData, e.to_string());
            return Err(err);
        }
    };
    let cpu_temp = get_cpu_temp()? as f32 / 1000.0;
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

    println!(
        "{} \u{2502} {} \u{2502} {:.1}\u{00B0}C \u{2502} {:02}:{:02} \u{2502} {} ",
        network_str,
        saturating_sub_bytes(mem.total, mem.free),
        cpu_temp,
        uptime.num_hours(),
        uptime.num_minutes() % 60,
        local_time_str
    );

    Ok(())
}

fn run() {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_secs(5);

    loop {
        match print_info(&sys) {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e.to_string());
            }
        }
        std::thread::sleep(refresh_rate);
    }
}

fn main() {
    run();
}

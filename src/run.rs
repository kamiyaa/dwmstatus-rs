use systemstat::{saturating_sub_bytes, Platform};

use crate::{config::AppConfig, types::{BatteryState, NetworkStatus}, utils::read_to_usize};

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
        NetworkStatus::Wireless => "\u{1F6DC}",
        NetworkStatus::Ethernet => "\u{1F5A5}",
        NetworkStatus::UsbTether => "\u{1F4F1}",
        NetworkStatus::Disconnected => "\u{274C}",
    };
    status_bar.push_str(&format!("\u{1F310} {network_str} \u{2502} "));

    let mem = sys.memory()?;

    status_bar.push_str(&format!(
        "{} \u{2502} ",
        saturating_sub_bytes(mem.total, mem.free)
    ));

    if let Some(path) = config.cpu.temperature_file.as_ref() {
        if let Ok(temp) = read_to_usize(path) {
            let temp_normalized = match &config.cpu.temperature_denominator {
                Some(denom) => temp as f32 / *denom,
                None => temp as f32,
            };
            status_bar.push_str(&format!("{:.1}\u{00B0}C \u{2502} ", temp_normalized));
        }
    }

    let battery_state = BatteryState::from_config(&config.battery);
    if let Some(state) = battery_state {
        status_bar.push_str(&state.to_status_string());
    }

    let local_time = chrono::Local::now();
    // 24 hr clock
    // let local_time_str = local_time.format("%a %m/%d  %H:%M");
    let local_time_str = local_time.format("%a %m/%d  %I:%M %p");
    if let Ok(uptime) = chrono::Duration::from_std(sys.uptime()?) {
        status_bar.push_str(&format!(
            "\u{25b2} {}d {:02}h {:02}m \u{2502} {} ",
            uptime.num_days(),
            uptime.num_hours() % 24,
            uptime.num_minutes() % 60,
            local_time_str
        ));
    };
    Ok(status_bar)
}

pub fn run(config: AppConfig) {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_millis(config.poll_rate);

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

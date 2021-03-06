use systemstat::{Platform, saturating_sub_bytes};

fn run() {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_secs(10);

    loop {
        let battery = match sys.battery_life() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
        let uptime: chrono::Duration = match sys.uptime() {
            Ok(s) => match chrono::Duration::from_std(s) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                },
            },
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
        let cpu_temp = match sys.cpu_temp() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
        let mem = match sys.memory() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
        let networks = match sys.networks() {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            },
        };
        let mut network_str = "--/--";
        for (name, network) in networks {
            match name.as_str() {
                "wlan0" if network.addrs.len() > 0 => {
                    network_str = "<--->";
                    break;
                }
                "enp3s0" if network.addrs.len() > 0 => {
                    network_str = "[---]";
                    break;
                }
                _ => {},
            }
        }

        let local_time = chrono::Local::now();
        let local_time_str = local_time.format("%a %m/%d  %I:%M");

        println!("{} \u{2502} {} \u{2502} {}\u{00B0}C \u{2502} [{:.00}%] \u{2502} {:02}:{:02} \u{2502} {} ",
            network_str,
            saturating_sub_bytes(mem.total, mem.free),
            cpu_temp,
            battery.remaining_capacity * 100.0,
            uptime.num_hours(), uptime.num_minutes() % 60,
            local_time_str);

        std::thread::sleep(refresh_rate);
    }
}

fn main() {
    run();
}

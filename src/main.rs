use systemstat::{Platform, saturating_sub_bytes};

fn run() {
    let sys = systemstat::System::new();
    let refresh_rate = std::time::Duration::from_secs(10);

    loop {
        let ac = match sys.on_ac_power() {
            Ok(s) => s,
            _ => false,
        };
        let ac_str = if ac {
            "+"
        } else {
            "-"
        };

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
        let network_str = match networks.get("wlan0") {
            Some(s) if s.addrs.len() > 0 => "<--->",
            _ => match networks.get("enp4s0") {
                Some(s) if s.addrs.len() > 0 => "[---]",
                _ => "--/--",
            }
        };

        let local_time = chrono::Local::now();
        let local_time_str = local_time.format("%a %m/%d  %I:%M");

        println!("{} \u{2502} {} \u{2502} {}\u{00B0}C \u{2502} [{:.00}%{}] \u{2502} {:02}:{:02} \u{2502} {} ",
            network_str,
            saturating_sub_bytes(mem.total, mem.free),
            cpu_temp,
            battery.remaining_capacity * 100.0,
            ac_str,
            uptime.num_hours(), uptime.num_minutes() % 60,
            local_time_str);

        std::thread::sleep(refresh_rate);
    }
}

fn main() {
    run();
}

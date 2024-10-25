use crate::prelude::*;
use std::cmp::Ordering;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Process {
    pub name: String,
    pub pid: String,
    pub load: f32,
}

pub fn run(cfg: Config) {
    println!(
        "Start CPU load logging (show every {} secs and log top {} processes).",
        cfg.repetition_time, cfg.nb_of_processes
    );
    if cfg.show_each_cpu {
        loop_with_each_cpu_load(cfg);
    } else {
        loop_with_avg_load(cfg);
    }
}

fn loop_with_avg_load(cfg: Config) {
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    std::thread::sleep(Duration::from_secs(1));

    let mut avg_load: f32 = 0.0;
    let cpus: f32 = sys.cpus().len() as f32;

    loop {
        sys.refresh_cpu_usage();

        for cpu in sys.cpus() {
            avg_load += cpu.cpu_usage();
        }

        let cur_load: f32 = avg_load / cpus;
        if cur_load > cfg.cpu_threshold {
            println!(
                "[{}] CPU avg load: {:.4} %",
                get_current_date_as_string(),
                cur_load
            );
        }

        if cur_load > cfg.process_threshold {
            get_highest_processes(cfg.nb_of_processes);
        }

        avg_load = 0.0;
        std::thread::sleep(Duration::from_secs(cfg.repetition_time as u64));
    }
}

fn loop_with_each_cpu_load(cfg: Config) {
    let mut sys = System::new();
    sys.refresh_cpu_usage();
    std::thread::sleep(Duration::from_secs(1));

    let cpus = sys.cpus().len();
    let cpu_avg: f32 = cfg.cpu_threshold / cpus as f32;

    let mut cpu_no: i32 = 1;
    let mut highest_load: f32 = 0.0;

    loop {
        sys.refresh_cpu_usage();
        let mut output: bool = false;

        print!("[{}] ", get_current_date_as_string());
        for cpu in sys.cpus() {
            let usage: f32 = cpu.cpu_usage();

            if usage >= highest_load {
                highest_load = usage;
            }

            if usage >= cpu_avg {
                print!("cpu#{}: {:.2}%  ", cpu_no, usage);
                output = true;
            }

            cpu_no += 1;
        }
        if output {
            print!("\n");
        } else {
            print!("all cpus below {:.2}% usage\n", cpu_avg);
        }

        if highest_load > cfg.process_threshold {
            get_highest_processes(cfg.nb_of_processes);
        }

        cpu_no = 1;
        highest_load = 0.0;

        std::thread::sleep(Duration::from_secs(cfg.repetition_time as u64));
    }
}

fn get_highest_processes(max_processes: u32) {
    let mut procs: Vec<Process> = Vec::new();
    let sys = System::new_with_specifics(
        RefreshKind::new().with_processes(ProcessRefreshKind::everything()),
    );
    std::thread::sleep(Duration::from_millis(500));
    for (p_id, proc) in sys.processes() {
        let p = Process {
            name: proc
                .exe()
                .as_ref()
                .and_then(|name| name.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or("default")
                .to_string(), //format!("{:?}", proc.exe()),
            pid: p_id.to_string(),
            load: proc.cpu_usage(),
        };
        procs.push(p);
    }
    procs.sort_by(|a, b| b.load.partial_cmp(&a.load).unwrap_or(Ordering::Equal));

    println!("-----------------------");
    println!("TOP cpu load processes:");
    for (n, p) in procs.iter().enumerate() {
        if n == max_processes as usize {
            break;
        }
        println!("{}.) CPU usage: {:.2}%  program: {}", n + 1, p.load, p.name);
    }
    println!("-----------------------");
}

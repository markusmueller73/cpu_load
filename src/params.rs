use std::{env, process};

#[derive(Debug)]
pub struct Config {
    pub repetition_time: u32,
    pub show_each_cpu: bool,
    pub nb_of_processes: u32,
    pub cpu_threshold: f32,
    pub process_threshold: f32,
}

impl Config {
    fn new() -> Config {
        Config {
            repetition_time: 5,
            show_each_cpu: false,
            nb_of_processes: 5,
            cpu_threshold: 50.0,
            process_threshold: 90.0,
        }
    }
    pub fn parse() -> Result<Config, String> {
        let mut cfg = Config::new();

        let mut params = env::args().skip(1);
        let prg_name = env::args().nth(0).unwrap();
        let version = option_env!("CARGO_PKG_VERSION").unwrap();

        while let Some(param) = params.next() {
            match &param[..] {
                "-h" | "--help" => {
                    show_help(&prg_name);
                }
                "-v" | "--version" => {
                    println!("{} v{}\n", prg_name, version);
                    process::exit(0);
                }
                "-e" | "--each-cpu" => {
                    cfg.show_each_cpu = true;
                }
                "-m" | "--max-processes" => {
                    if let Some(next_param) = params.next() {
                        cfg.nb_of_processes = next_param.parse::<u32>().unwrap();
                    } else {
                        return Err("No value specified for parameter --processes.".to_string());
                    }
                }
                "-t" | "--threshold" => {
                    if let Some(next_param) = params.next() {
                        cfg.cpu_threshold = next_param.parse::<f32>().unwrap();
                    } else {
                        return Err("No value specified for parameter --threshold.".to_string());
                    }
                }
                "-s" | "--show-processes" => {
                    if let Some(next_param) = params.next() {
                        cfg.process_threshold = next_param.parse::<f32>().unwrap();
                    } else {
                        return Err("No value specified for parameter --threshold.".to_string());
                    }
                }
                "-u" | "--update" => {
                    if let Some(next_param) = params.next() {
                        cfg.repetition_time = next_param.parse::<u32>().unwrap();
                    } else {
                        return Err("No value specified for parameter --update.".to_string());
                    }
                }
                _ => {
                    if param.starts_with('-') {
                        return Err(format!("Unkown argument: {}.", param));
                    } else {
                        return Err(format!("Unkown positional argument: {}.", param));
                    }
                }
            }
        }

        // dbg!(&cfg);
        Ok(cfg)
    }
}

fn show_help(name: &String) {
    println!("\nUsage:");
    println!("{} [OPTIONS]\n", name);
    println!("Options:");
    println!("-e, --each-cpu                watch the load of each cpu core, default is watching average load,");
    println!(
        "                              default is to watch the average load of all cpus in system."
    );
    println!("-h, --help                    show this help");
    println!("-m, --max-processes [NUMBER]  show NUMBER processes with max load (default is 5)");
    println!("-s, --show-processes [NUMBER] set the minimum load of all cpus to report the processes with max");
    println!("                              load in percent without % sign (default is 90%)");
    println!("-t, --threshold [NUMBER]      set the minimum load in percent of the cpus for reporting as a");
    println!("                              NUMBER without % sign (default is 50%)");
    println!(
        "-u, --update [SECONDS]        update the cpu usage every SECONDS seconds (default is 5)"
    );
    println!("-v, --version                 show the program version and exit");
    println!("");
    process::exit(0);
}

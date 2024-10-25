mod cpuload;
mod date;
mod params;

mod prelude {
    pub use crate::cpuload::*;
    pub use crate::date::*;
    pub use crate::params::*;
}
use prelude::*;
use std::process;

fn main() {
    let config = Config::parse().unwrap_or_else(|err| {
        eprintln!("Error while parsing options: {err}");
        process::exit(1);
    });
    run(config);
}

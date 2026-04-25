mod config;
mod error;
mod runner;

use crate::config::Config;
use crate::runner::Runner;

fn run() -> Result<(), error::Error> {
    let cfg = Config::default();
    let runner = Runner::new(cfg);
    runner.execute()?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

extern crate csv_convert_job_rs;
use csv_convert_job_rs::arg::get_nth_arg;
use csv_convert_job_rs::cfg::load_cfg;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let config_csv_path = get_nth_arg::get(1)?;
    let cfg = load_cfg::load(config_csv_path);
    println!("{:#?}", cfg);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

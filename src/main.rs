extern crate csv_convert_job_rs;
use csv_convert_job_rs::arg::get_nth_arg;

extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::process;

#[derive(Debug, Deserialize)]
struct Record {
    row: usize,
    column: usize,
    name: String,
}

fn run() -> Result<(), Box<Error>> {
    let config_csv_path = get_nth_arg::get(1)?;
    let mut rdr = csv::Reader::from_path(config_csv_path)?;
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

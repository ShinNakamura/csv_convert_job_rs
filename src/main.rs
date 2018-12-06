extern crate csv_convert_job_rs;
use csv_convert_job_rs::arg::get_nth_arg;

use std::error::Error;
use std::process;

fn run() -> Result<(), Box<Error>> {
    let first_arg = get_nth_arg::get(1)?;
    println!("first arg is {:?}", first_arg);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

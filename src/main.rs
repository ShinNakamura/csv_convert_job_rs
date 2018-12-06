extern crate csv_convert_job_rs;

use csv_convert_job_rs::get_nth_arg::get_nth_arg as gna;

fn main() {
    let first_arg = gna::get_nth_arg(1).unwrap();
    println!("first arg is {:?}", first_arg);
}

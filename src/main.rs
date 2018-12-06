extern crate csv_convert_job_rs;

use csv_convert_job_rs::arg::get_nth_arg;

fn main() {
    let first_arg = get_nth_arg::get(1).unwrap();
    println!("first arg is {:?}", first_arg);
}

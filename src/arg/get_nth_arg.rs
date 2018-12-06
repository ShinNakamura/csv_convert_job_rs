use std::env;
use std::error::Error;
use std::ffi::OsString;

pub fn get(n: usize) -> Result<OsString, Box<Error>> {
    match env::args_os().nth(n) {
        None => Err(From::from(format!("expected nth({}) argument, but got None", n))),
        Some(val) => Ok(val),
    }
}
